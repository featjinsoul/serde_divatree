use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::{FromStr, Split},
};

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use thiserror::Error;

use crate::DeserializerError;

pub(crate) fn from_str<'a, T>(s: &'a str) -> Result<T, ParseAtomError>
where
    T: Deserialize<'a>,
{
    let mut lex = AtomParser(s);
    T::deserialize(lex)
}

#[derive(Debug, Clone, Copy)]
struct AtomParser<'de>(&'de str);

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ParseAtomError {
    #[error("Expected an integer, found something else")]
    ExpectedInteger(#[from] ParseIntError),
    #[error("Expected an float, found something else")]
    ExpectedFloat(#[from] ParseFloatError),
    #[error("Expected a boolean, found something else")]
    ExpectedBool,
    #[error("Expected the start of a tuple, found something else")]
    ExpectedTupleStart,
    #[error("Expected a non empty tuple, found an empty tuple")]
    ExpectedNonEmptyTuple,
    #[error("Expected the end of a tuple, found something else")]
    ExpectedTupleEnd,
    #[error("{0}")]
    Custom(String),
}

impl serde::de::Error for ParseAtomError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

impl<'de> AtomParser<'de> {
    fn convert_value<T>(self) -> Result<T, ParseAtomError>
    where
        T: FromStr,
        ParseAtomError: From<T::Err>,
    {
        self.0.parse().map_err(ParseAtomError::from)
    }
}

impl<'de> Deserializer<'de> for AtomParser<'de> {
    type Error = ParseAtomError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.0.starts_with('(') {
            let len = self.0.split(',').count();
            self.deserialize_tuple(len, visitor)
        } else if self.convert_value::<i64>().is_ok() {
            self.deserialize_i64(visitor)
        } else if self.convert_value::<f64>().is_ok() {
            self.deserialize_f64(visitor)
        } else if self.0.eq_ignore_ascii_case("true") || self.0.eq_ignore_ascii_case("false") {
            self.deserialize_bool(visitor)
        } else {
            self.deserialize_str(visitor)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let val = self.0;
        if val.eq_ignore_ascii_case("true") {
            visitor.visit_bool(true)
        } else if val.eq_ignore_ascii_case("false") {
            visitor.visit_bool(false)
        } else {
            Err(ParseAtomError::ExpectedBool)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.convert_value()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.convert_value()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.convert_value()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.convert_value()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.convert_value()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.convert_value()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.convert_value()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.convert_value()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.convert_value()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.convert_value()?)
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
        visitor.visit_borrowed_str(self.0)
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
        struct TupleParser<'a>(Split<'a, char>);

        impl<'a, 'de> SeqAccess<'de> for TupleParser<'de> {
            type Error = ParseAtomError;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                self.0
                    .next()
                    .map(|x| seed.deserialize(AtomParser(x.trim())))
                    .transpose()
            }
        }

        if let Some(prefix) = self.0.strip_prefix('(') {
            if let Some(vals) = prefix.strip_suffix(')') {
                visitor.visit_seq(TupleParser(vals.split(',')))
            } else {
                Err(ParseAtomError::ExpectedTupleEnd)
            }
        } else {
            Err(ParseAtomError::ExpectedTupleStart)
        }
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
        todo!()
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
        self.deserialize_str(visitor)
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
    use super::*;

    #[test]
    fn read_bool() {
        assert_eq!(from_str("TrUe"), Ok(true));
        assert_eq!(from_str("false"), Ok(false));
        assert_eq!(from_str::<bool>("foo"), Err(ParseAtomError::ExpectedBool));
    }

    #[test]
    fn read_int() {
        assert_eq!(from_str("0"), Ok(0u8));
        assert_eq!(from_str("-128"), Ok(-128i8));
        assert_eq!(from_str("255"), Ok(255u8));
        assert_eq!(from_str("+256"), Ok(256u16));
        match from_str::<u8>("foo") {
            Err(ParseAtomError::ExpectedInteger(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn read_float() {
        assert_eq!(from_str("0."), Ok(0f32));
        assert_eq!(from_str(".123"), Ok(0.123f32));
        assert_eq!(from_str("+0.0"), Ok(0f32));
        assert_eq!(from_str("1.234"), Ok(1.234f32));
        assert_eq!(from_str("-1.234"), Ok(-1.234f32));
        assert_eq!(from_str("6.02e23"), Ok(6.02e23f64));
        match from_str::<f32>("foo") {
            Err(ParseAtomError::ExpectedFloat(_)) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn read_tuple() {
        assert_eq!(
            from_str("( 123, 3.1415 , Hello World!, )",),
            Ok((123u8, 3.1415f32, "Hello World!"))
        );
        assert_eq!(from_str("( 123, )",), Ok((123u8,)));
        assert_eq!(from_str("()",), Ok(()));
        match from_str::<(u8,)>("123)") {
            Err(ParseAtomError::ExpectedTupleStart) => {}
            _ => unreachable!(),
        }
        match from_str::<(u8,)>("(123") {
            Err(ParseAtomError::ExpectedTupleEnd) => {}
            e => unreachable!("{:?}", e),
        }
    }
}
