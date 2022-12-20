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
    let mut lex = Lexer::from_str(s);
    T::deserialize(&mut lex)
}

impl<'de, I: Iterator<Item = &'de str>> Lexer<Peekable<I>> {
    fn peek_key_value(&mut self) -> Result<KeyValue<'de>, DeserializerError> {
        self.lines
            .peek()
            .cloned()
            .and_then(KeyValue::new)
            .ok_or(DeserializerError::ExpectedKeyValuePair)
    }
    fn value(&mut self) -> Result<&'de str, DeserializerError> {
        dbg!(self.lines.peek());
        self.peek_key_value().map(|x| x.value).or(self
            .lines
            .next()
            .ok_or(DeserializerError::ExpectedValueNode))
    }
    fn convert_value<T>(&mut self) -> Result<T, DeserializerError>
    where
        T: FromStr,
        DeserializerError: From<T::Err>,
    {
        self.value()?.parse().map_err(DeserializerError::from)
    }
    fn parse_atom<T: Deserialize<'de>>(&mut self) -> Result<T, DeserializerError> {
        atom::from_str(self.value()?).map_err(Into::into)
    }
}

impl<'a, 'de, I: 'de> Deserializer<'de> for &'a mut Lexer<Peekable<I>>
where
    I: Iterator<Item = &'de str> + Clone,
{
    type Error = DeserializerError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_atom()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_atom()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_atom()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_atom()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_atom()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_atom()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_atom()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_atom()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_atom()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_atom()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_atom()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
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
        #[derive(Clone)]
        struct MapParser<'a, T> {
            iter: LexerChildren<'a, T>,
            // cur: KeyValue<'a>,
        }

        impl<'a, T: Iterator<Item = &'a str> + Clone + 'a> MapParser<'a, T> {
            fn peek_key_value(&mut self) -> Option<KeyValue<'a>> {
                let mut iter = self.iter.clone().peekable();
                iter.peek().cloned().and_then(KeyValue::new)
            }
        }

        impl<'a, T: Iterator<Item = &'a str> + Clone + 'a> MapAccess<'a> for MapParser<'a, T> {
            type Error = DeserializerError;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
            where
                K: DeserializeSeed<'a>,
            {
                println!("reading map key");
                let keyval = if let Some(keyval) = self.peek_key_value() {
                    keyval
                } else {
                    return Ok(None);
                };
                let ident = keyval.path().next().expect("path");
                dbg!(ident);
                let lexer = AtomParser(ident);
                seed.deserialize(lexer).map(Some).map_err(Into::into)
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: DeserializeSeed<'a>,
            {
                println!("reading map value");
                let mut iter = LexerChildren::new(self.clone().iter)
                    .strip_prefix(true)
                    .chain(std::iter::once(
                        self.peek_key_value().expect("key value").value,
                    ))
                    .peekable();
                dbg!(iter.peek());
                let mut lexer = Lexer::new(iter);
                self.iter.next();
                seed.deserialize(&mut lexer)
            }
        }

        let mut iter = self.lines.clone().peekable();
        let children = LexerChildren::new(iter);
        visitor.visit_map(MapParser { iter: children })
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
        todo!()
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    fn read_map_nested() {
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
}
