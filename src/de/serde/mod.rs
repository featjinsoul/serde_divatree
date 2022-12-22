use ::serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use ::serde::Deserializer;

use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use super::*;
use crate::error::DeserializerError;
use crate::serde::atom::AtomParser;

pub mod atom;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, DeserializerError>
where
    T: Deserialize<'a>,
{
    let mut iter = s
        .lines()
        .filter(|x| !x.trim().is_empty())
        .filter(|x| !x.starts_with('#'))
        .peekable();
    let mut lex = LexerChildren::new(iter);
    T::deserialize(&mut lex)
}

impl<'de, I: Iterator<Item = &'de str>> LexerChildren<'de, I> {
    fn value(&mut self) -> Result<&'de str, DeserializerError> {
        let line = self.next();
        let kv = line
            .and_then(KeyValue::new)
            .ok_or(DeserializerError::ExpectedValueNode)?;
        let val = kv
            .path()
            .nth(0)
            .filter(|x| !x.is_empty())
            .unwrap_or(kv.value);
        println!("{:?}, reading {:?}", line, val);
        Ok(val)
    }
    fn atom(&mut self) -> Result<AtomParser<'de>, DeserializerError> {
        self.value().map(AtomParser)
    }
}

impl<'a, 'de, I: Iterator<Item = &'de str>> LexerChildren<'de, Peekable<I>> {
    fn peek_key_value(&mut self) -> Result<KeyValue<'de>, DeserializerError> {
        self.peek()
            .and_then(KeyValue::new)
            .ok_or(DeserializerError::ExpectedKeyValuePair)
    }
}

impl<'a, 'de, I: 'de> Deserializer<'de> for &'a mut LexerChildren<'de, Peekable<I>>
where
    I: Iterator<Item = &'de str>,
{
    type Error = DeserializerError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let kv = self.peek_key_value()?;
        if !kv.key.is_empty() {
            if kv.key.chars().all(|x| char::is_ascii_digit(&x)) {
                self.deserialize_seq(visitor)
            } else {
                self.deserialize_map(visitor)
            }
        } else {
            AtomParser(kv.value)
                .deserialize_any(visitor)
                .map_err(Into::into)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_bool(visitor).map_err(Into::into)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_i8(visitor).map_err(Into::into)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_i16(visitor).map_err(Into::into)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_i32(visitor).map_err(Into::into)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_i64(visitor).map_err(Into::into)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_u8(visitor).map_err(Into::into)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_u16(visitor).map_err(Into::into)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_u32(visitor).map_err(Into::into)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_u64(visitor).map_err(Into::into)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_f32(visitor).map_err(Into::into)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_f64(visitor).map_err(Into::into)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_char(visitor).map_err(Into::into)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_str(visitor).map_err(Into::into)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_string(visitor).map_err(Into::into)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?.deserialize_bytes(visitor).map_err(Into::into)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?
            .deserialize_byte_buf(visitor)
            .map_err(Into::into)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?
            .deserialize_tuple(len, visitor)
            .map_err(Into::into)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.atom()?
            .deserialize_identifier(visitor)
            .map_err(Into::into)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }
}

impl<'de, I: Iterator<Item = &'de str> + 'de> MapAccess<'de> for LexerChildren<'de, Peekable<I>> {
    type Error = DeserializerError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        println!("reading map key");
        // dbg!(self.peek());
        // dbg!(self.cache, self.prefix, self.lines.peek());
        if self.is_finished() {
            println!("------------- done ----------");
            return Ok(None);
        }
        // dbg!(self.lines.peek());
        let val = seed.deserialize(&mut *self).map(Some);
        self.increment_prefix_level();
        val
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        println!("reading map value");
        // dbg!(self.next());
        let val = seed.deserialize(&mut *self);
        self.decrement_prefix_level();
        self.next();
        val
    }
}

impl<'de, I: Iterator<Item = &'de str> + 'de> SeqAccess<'de> for LexerChildren<'de, Peekable<I>> {
    type Error = DeserializerError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.is_finished() {
            return Ok(None);
        }
        let ident = self.value()?;
        if ident.chars().all(|x| x.is_ascii_digit()) {
            self.increment_prefix_level();
            let val = seed.deserialize(&mut *self);
            self.decrement_prefix_level();
            self.next();
            Some(val).transpose()
        } else {
            Err(DeserializerError::ExpectedSequenece)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_derive::Deserialize;

    use super::*;

    #[test]
    fn read_map() {
        let input = "foo = 1
bar = 2
baz = 3
quux = 4
";
        let mut expected = HashMap::new();
        expected.insert("foo", 1);
        expected.insert("bar", 2);
        expected.insert("baz", 3);
        expected.insert("quux", 4);
        assert_eq!(from_str(input), Ok(expected));
    }
    #[test]
    fn read_nested_map() {
        let input = "foo.bar = 1
foo.baz = 2
bar.baz = 3
bar.quux = 4";
        let mut expected = HashMap::new();

        let mut foo = HashMap::new();
        foo.insert("bar", 1);
        foo.insert("baz", 2);

        let mut bar = HashMap::new();
        bar.insert("baz", 3);
        bar.insert("quux", 4);

        expected.insert("foo", foo);
        expected.insert("bar", bar);
        assert_eq!(from_str(input), Ok(expected));
    }

    #[test]
    fn read_struct() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            foo: u32,
            bar: f32,
            baz: bool,
            inner: Inner,
            #[serde(flatten)]
            custom: HashMap<String, String>,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Inner {
            name: String,
            id: u32,
        }
        let input = "foo=32
bar=1.234
baz=true
inner.name=John Smith
inner.id=69
extra=stuff";
        let data: Test = from_str(input).unwrap();
        let mut custom = HashMap::new();
        custom.insert("extra".to_string(), "stuff".to_string());
        let expected = Test {
            foo: 32,
            bar: 1.234,
            baz: true,
            inner: Inner {
                name: "John Smith".to_string(),
                id: 69,
            },
            custom,
        };
        assert_eq!(data, expected);
    }

    #[test]
    fn read_seq() {
        let input = "0=0
1=1
10=10
11=11
2=2
3=3
4=4
5=5
6=6
7=7
8=8
9=9";
        let data: Vec<i64> = from_str(input).unwrap();
        assert_eq!(data, (0..12).collect::<Vec<_>>());
    }
}
