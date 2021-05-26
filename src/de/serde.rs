use ::serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use ::serde::Deserializer;

use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use super::*;
use crate::error::DeserializerError;

impl<'a> A3daTree<'a> {
    fn get(&self) -> slab_tree::NodeRef<Node> {
        //self.curr is guarranteed to be a valid id into the tree
        self.tree.get(self.curr).unwrap()
    }
    fn get_key(&'a self) -> Result<&'a str, DeserializerError> {
        let data = self.get().data();
        match data {
            Node::Key(e) => Ok(e),
            _ => Err(DeserializerError::ExpectedKeyNode),
        }
    }
    fn get_value(&'a self) -> Result<&'a str, DeserializerError> {
        let data = self.get().data();
        match data {
            Node::Value(e) => Ok(e),
            _ => Err(DeserializerError::ExpectedValueNode),
        }
    }
    fn parse_int<T: FromStr<Err = ParseIntError>>(&self) -> Result<T, DeserializerError> {
        let data = self.get_value()?;
        data.parse().map_err(DeserializerError::ExpectedInteger)
    }
    fn parse_float<T: FromStr<Err = ParseFloatError>>(&self) -> Result<T, DeserializerError> {
        let data = self.get_value()?;
        data.parse().map_err(DeserializerError::ExpectedFloat)
    }
    fn get_char(&self) -> Result<char, DeserializerError> {
        self.get()
            .data()
            .chars()
            .next()
            .ok_or(DeserializerError::ExpectedChar)
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut A3daTree<'de> {
    type Error = DeserializerError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let data = self.get_value()?;
        let val = if data.eq_ignore_ascii_case("true") {
            true
        } else if data.eq_ignore_ascii_case("false") {
            false
        } else {
            return Err(DeserializerError::ExpectedBool);
        };
        visitor.visit_bool(val)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_int()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_int()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_int()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_int()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_int()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_int()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_int()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_int()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_float()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_float()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(self.get_char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        //visitor.visit_borrowed_str(self.get_value()?)
        visitor.visit_str(self.get_value()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.get_value()?.to_string())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self
            .get()
            .data()
            .deref()
            .chars()
            .next()
            .unwrap()
            .is_numeric()
        {
            visitor.visit_seq(SeqParser(self, false))
        } else {
            Err(DeserializerError::ExpectedSequenece)
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
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
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
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
        V: de::Visitor<'de>,
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
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

struct SeqParser<'de, 'a>(&'a mut A3daTree<'de>, bool);

impl<'de, 'a> SeqAccess<'de> for SeqParser<'de, 'a> {
    type Error = DeserializerError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.1 {
            return Ok(None);
        }
        let idx = self.0.get_char().unwrap();
        if !idx.is_numeric() {
            return Ok(None);
        }
        self.1 = self.0.get().next_sibling().is_none();
        let temp = self.0.curr;
        self.0.curr = self.0.get().first_child().unwrap().node_id();
        let val = seed.deserialize(&mut *self.0)?;
        self.0.curr = temp;
        match self.0.get().next_sibling() {
            Some(n) => self.0.curr = n.node_id(),
            None => self.1 = true,
        }
        Ok(Some(val))
    }
}

#[cfg(test)]
mod tests {
    use ::serde::Deserialize;

    use super::*;

    #[test]
    fn bool() {
        let input = "test=true";
        let mut tree = A3daTree::new(input).unwrap();
        tree.curr = tree.get().first_child().unwrap().node_id();
        assert!(bool::deserialize(&mut tree).unwrap())
    }

    #[test]
    fn test_seq() {
        let input = "0 = a
1=b
2=c
welcome = banana";
        let mut tree = A3daTree::new(input).unwrap();
        tree.print();
        let data: Vec<char> = Vec::deserialize(&mut tree).unwrap();
        assert_eq!(data, vec!['a', 'b', 'c']);
    }
}
