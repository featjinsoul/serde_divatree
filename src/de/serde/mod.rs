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
    let mut iter = s.lines().peekable();
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
    fn parse_atom<T: Deserialize<'de>>(&mut self) -> Result<T, DeserializerError> {
        atom::from_str(self.value()?).map_err(Into::into)
    }
}

impl<'a, 'de, I: Iterator<Item = &'de str>> LexerChildren<'de, Peekable<I>> {
    fn peek_key_value(&mut self) -> Result<KeyValue<'de>, DeserializerError> {
        self.lines
            .peek()
            .cloned()
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
        todo!()
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_atom()?)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_atom()?)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_atom()?)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_atom()?)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_atom()?)
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_atom()?)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_atom()?)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_atom()?)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_atom()?)
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_atom()?)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_atom()?)
    }

    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_atom()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
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
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
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
        todo!()
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

    fn deserialize_identifier<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let key = self
            .peek_key_value()?
            .path()
            .next()
            .ok_or(DeserializerError::ExpectedKeyNode)?;
        visitor.visit_borrowed_str(key)
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
        let expected = Test {
            foo: 32,
            bar: 1.234,
            baz: true,
            inner: Inner {
                name: "John Smith".to_string(),
                id: 69,
            },
        };
        assert_eq!(data, expected);
    }
}
