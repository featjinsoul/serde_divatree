use slab_tree::*;

use std::iter::{Peekable, Take};
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
        if key.is_empty() {
            return None;
        }
        let value = iter.next()?.trim();
        if value.is_empty() {
            return None;
        }
        Some(Self { key, value })
    }
    fn path(&self) -> Split<'a, char> {
        self.key.split(Self::PATH_DELIMITER)
    }
    fn strip(&self, levels: usize) -> Option<Self> {
        let dot_pos = self
            .key
            .match_indices(Self::PATH_DELIMITER)
            .nth(levels)
            .map(|(i, _)| i);
        let key = dot_pos.and_then(|i| self.key.get(i + 1..))?;
        Some(Self {
            key,
            value: self.value,
        })
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

impl<'de, I> Lexer<I>
where
    I: Iterator<Item = &'de str> + Clone,
{
    fn children(&self) -> Option<impl Iterator<Item = &'de str>> {
        fn get_parent(s: &str) -> Option<&str> {
            s.find(KeyValue::PATH_DELIMITER)
                .and_then(|i| s.get(..i))
                .map(|x| x.trim())
        }

        let mut lines = self.lines.clone().peekable();
        let first = lines.next()?;
        let parent = get_parent(first);
        Some(
            std::iter::once(first)
                .chain(lines.clone().take_while(move |x| get_parent(x) == parent)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_children() {
        const INPUT: &'static str = "foo.bar
foo.baz
foo.quux
foobar
";

        let par = Lexer::from_str(INPUT);
        let mut lines = INPUT.lines();
        let mut child = par.children().unwrap();
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), lines.next());
        assert_eq!(child.next(), None);

        assert_eq!(
            Lexer::from_str("").children().and_then(|mut x| x.next()),
            None
        );
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
        assert_eq!(KeyValue::new("= baz "), None);
        assert_eq!(KeyValue::new(" bar = "), None);
    }
}
