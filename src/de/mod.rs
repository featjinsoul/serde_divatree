use slab_tree::*;

use std::iter::{Peekable, Take};
use std::num::NonZeroU8;
use std::str::{Lines, Split};

pub mod serde;
pub use self::serde::from_str;

use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Lexer<I> {
    lines: I,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct KeyValue<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> KeyValue<'a> {
    const PATH_DELIMITER: char = '.';
    const KEY_VALUE_DELIMITER: char = '=';

    fn new(line: &'a str) -> Option<Self> {
        let mut iter = line.split(Self::KEY_VALUE_DELIMITER);
        let key = iter.next()?.trim();
        let value = iter.next()?.trim();
        Some(Self { key, value })
    }
    fn path(&self) -> impl Iterator<Item = &'a str> {
        self.key
            .split(Self::PATH_DELIMITER)
            .filter(|x| !x.is_empty())
    }
    fn prefix(&self, level: usize) -> Option<&'a str> {
        self.prefixes().nth(level)
    }
    fn prefixes(&self) -> impl Iterator<Item = &'a str> + 'a {
        use std::iter::once;
        let key = self.key;
        let mid = self
            .key
            .match_indices(Self::PATH_DELIMITER)
            .flat_map(move |(i, _)| key.get(..i + 1));
        let opt = Some(key).filter(|x| !x.is_empty());
        once("").chain(mid).chain(opt.into_iter())
    }
    fn strip(&self, levels: usize) -> Option<Self> {
        let key = self.key.strip_prefix(self.prefix(levels)?)?;
        Some(Self { key, ..*self })
    }

    fn levels(self) -> impl Iterator<Item = KeyValue<'a>> + 'a {
        let mut level = 0;
        let iter = std::iter::from_fn(move || {
            let val = self.strip(level);
            level += 1;
            val
        });
        std::iter::once(self).chain(iter)
    }
}

impl<I> Lexer<I> {
    const fn new(lines: I) -> Self {
        Self { lines }
    }
}

impl<'de> Lexer<Peekable<std::str::Lines<'de>>> {
    fn from_str(input: &'de str) -> Self {
        Self::new(input.lines().peekable())
    }
}

#[derive(Clone)]
struct LexerChildren<'de, I> {
    lines: I,
    prefix: Option<&'de str>,
    /// Decides how much is taken as the prefix
    prefix_level: u8,
    /// A cached string used when changing the prefix mid iter
    cache: Option<&'de str>,
}

impl<'de, I: Iterator<Item = &'de str>> LexerChildren<'de, I> {
    fn new(lines: I) -> LexerChildren<'de, I> {
        LexerChildren {
            lines,
            prefix: None,
            prefix_level: 0,
            cache: None,
        }
    }

    fn prefix_level(&self) -> u8 {
        self.prefix_level
    }

    fn set_prefix_level(&mut self, prefix_level: u8) -> Option<&'de str> {
        self.prefix_level = prefix_level;
        std::mem::replace(&mut self.prefix, None)
    }

    fn with_prefix_level(mut self, prefix_level: u8) -> Self {
        self.set_prefix_level(prefix_level);
        self
    }

    fn increment_prefix_level(&mut self) -> Option<&'de str> {
        self.set_prefix_level(self.prefix_level.saturating_add(1))
    }

    fn decrement_prefix_level(&mut self) -> Option<&'de str> {
        self.set_prefix_level(self.prefix_level.saturating_sub(1))
    }

    fn get_prefix<'a>(&self, s: &'a str) -> Option<&'a str> {
        KeyValue::new(s)?.prefix(self.prefix_level as usize)
    }
    fn to_lexer(self) -> Lexer<Self> {
        Lexer { lines: self }
    }
}

impl<'de, I: Iterator<Item = &'de str>> LexerChildren<'de, Peekable<I>> {
    fn peek(&mut self) -> Option<&'de str> {
        let line = if self.cache.is_none() || self.prefix.is_some() {
            self.lines.peek().cloned()
        } else {
            self.cache
        }?;
        let prefix = self.prefix.or_else(|| self.get_prefix(line));
        // self.cache = Some(line);
        line.strip_prefix(prefix?)
    }

    fn is_finished(&mut self) -> bool {
        self.cache.is_some() && self.peek().is_none()
    }
}

impl<'de, I> Iterator for LexerChildren<'de, I>
where
    I: Iterator<Item = &'de str>,
{
    type Item = &'de str;

    fn next(&mut self) -> Option<Self::Item> {
        let line = if self.cache.is_none() || self.prefix.is_some() {
            self.lines.next()
        } else {
            self.cache
        }?;
        self.prefix = self.prefix.or_else(|| self.get_prefix(line));
        self.cache = Some(line);
        let stripped = line.strip_prefix(self.prefix?);
        stripped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_children() {
        const INPUT: &'static str = "foo.bar = 1
foo.baz = 1
foo.quux = 1
foobar = 1
";

        let par = Lexer::from_str(INPUT);
        let mut lines = INPUT.lines();
        let mut child = LexerChildren::new(lines.clone());
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), None);

        assert_eq!(LexerChildren::new("".lines()).next(), None);
    }

    #[test]
    fn lexer_chidren_stripped() {
        const INPUT: &'static str = "foo.bar = 1
foo.baz = 1
foo.quux = 1
foobar = 1
";

        let par = Lexer::from_str(INPUT);
        let mut lines = INPUT.lines();
        let mut child = LexerChildren::new(lines).with_prefix_level(1);
        assert_eq!(child.next(), Some("bar = 1"));
        assert_eq!(child.next(), Some("baz = 1"));
        assert_eq!(child.next(), Some("quux = 1"));
        assert_eq!(child.next(), None);
    }

    #[test]
    fn lexer_chidren_strip_switch() {
        const INPUT: &'static str = "foo.bar.baz = 1
foo.bar.quux = 1
test = 1
foobar.baz = 1
foobar.quux = 1
";

        let par = Lexer::from_str(INPUT);
        let mut lines = INPUT.lines();
        let mut child = LexerChildren::new(lines).with_prefix_level(1);
        assert_eq!(child.next(), Some("bar.baz = 1"));
        assert_eq!(child.next(), Some("bar.quux = 1"));
        assert_eq!(child.set_prefix_level(0), Some("foo."));
        assert_eq!(child.next(), Some("foo.bar.quux = 1"));
        assert_eq!(child.next(), Some("test = 1"));
        child.next();
        assert_eq!(child.set_prefix_level(1), Some(""));
        assert_eq!(child.next(), Some("baz = 1"));
        assert_eq!(child.next(), Some("quux = 1"));
        assert_eq!(child.next(), None);
    }

    #[test]
    fn key_value_new() {
        assert_eq!(
            KeyValue::new(" foo.bar = baz "),
            Some(KeyValue {
                key: "foo.bar",
                value: "baz"
            })
        );
        assert_eq!(
            KeyValue::new("= baz "),
            Some(KeyValue {
                key: "",
                value: "baz"
            })
        );
        assert_eq!(
            KeyValue::new(" bar = "),
            Some(KeyValue {
                key: "bar",
                value: ""
            })
        );
    }

    #[test]
    fn key_value_prefix() {
        let kv = KeyValue::new("foo.bar.baz = 1").unwrap();

        assert_eq!(kv.prefix(0), Some(""));
        assert_eq!(kv.prefix(1), Some("foo."));
        assert_eq!(kv.prefix(2), Some("foo.bar."));
        assert_eq!(kv.prefix(3), Some("foo.bar.baz"));
        assert_eq!(kv.prefix(4), None);

        let mut singleton = KeyValue::new("foo = 1").unwrap().prefixes();
        assert_eq!(singleton.next(), Some(""));
        assert_eq!(singleton.next(), Some("foo"));
        assert_eq!(singleton.next(), None);

        let mut empty = KeyValue::new(" = 1").unwrap().prefixes();
        assert_eq!(empty.next(), Some(""));
        assert_eq!(empty.next(), None);
    }

    #[test]
    fn key_value_path() {
        let kv = KeyValue::new(" = 1").unwrap();

        assert_eq!(kv.path().next(), None);
    }
}
