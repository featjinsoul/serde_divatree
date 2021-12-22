use ::serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use ::serde::Deserializer;

use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use super::*;
use crate::error::DeserializerError;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, DeserializerError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = A3daTree::new(s)?;
    T::deserialize(&mut deserializer)
}

impl<'a> A3daTree<'a> {
    fn get(&self) -> slab_tree::NodeRef<'_, Node<'a>> {
        //self.curr is guarranteed to be a valid id into the tree
        self.tree.get(self.curr).unwrap()
    }
    fn get_key(&self) -> Result<&'a str, DeserializerError> {
        let data = self.get().data();
        match data {
            Node::Key(e) => Ok(e),
            _ => Err(DeserializerError::ExpectedKeyNode),
        }
    }
    fn get_value(&self) -> Result<&'a str, DeserializerError> {
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
        let node = self.get().data();
        match node {
            Node::Key(s) => {
                //TODO
                let c = s.chars().next();
                match c {
                    Some(e) if e.is_numeric() => self.deserialize_seq(visitor),
                    _ => self.deserialize_map(visitor),
                }
            }
            Node::Value(s) => {
                let c = s.chars().next().unwrap();
                let len = s.split(',').count();
                match c {
                    't' | 'T' | 'f' | 'F' => self.deserialize_bool(visitor),
                    '0'..='9' => self.deserialize_u64(visitor),
                    '-' => self.deserialize_i64(visitor),
                    '(' => self.deserialize_tuple(len, visitor),
                    _ if s.contains('.') => self.deserialize_f64(visitor),
                    _ => self.deserialize_str(visitor),
                }
            }
        }
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
        let data: &'de str = match self.get().data() {
            Node::Key(v) => *v,
            Node::Value(v) => *v,
        };
        visitor.visit_borrowed_str(data)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.get().data().to_string())
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
        visitor.visit_unit()
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
        let data = self.get_value()?;
        if data.starts_with('(') {
            let parser = TupleParser::new(data)?;
            if data.ends_with(')') {
                visitor.visit_seq(parser)
            } else {
                Err(DeserializerError::ExpectedTupleEnd)
            }
        } else {
            Err(DeserializerError::ExpectedTuple)
        }
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
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Some(n) = self.get().first_child() {
            //self.curr = n.node_id();
            visitor.visit_map(MapParser {
                tree: self,
                next: None,
                end: false,
            })
        } else {
            Err(DeserializerError::ExpectedSequenece)
        }
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
        self.deserialize_map(visitor)
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
        if let Ok(val) = self.get_value() {
            if val.starts_with("(") {
                visitor.visit_enum(EnumParser { de: self })
            } else {
                visitor.visit_enum(val.into_deserializer())
            }
        } else {
            visitor.visit_enum(EnumParser { de: self })
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqParser<'de, 'a>(&'a mut A3daTree<'de>, bool);
struct TupleParser<'de>(A3daTree<'de>, bool);
struct MapParser<'de, 'a> {
    tree: &'a mut A3daTree<'de>,
    next: Option<NodeId>,
    end: bool,
}
struct EnumParser<'de, 'a> {
    de: &'a mut A3daTree<'de>,
}

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

impl TupleParser<'_> {
    fn new(input: &str) -> Result<TupleParser, DeserializerError> {
        let items = input.trim_matches(&['(', ')'][..]).split(',');
        let mut tree = TreeBuilder::new()
            .with_root(Node::Key("tuple_root"))
            .build();
        let mut root = tree.root_mut().unwrap();
        for item in items {
            root.append(Node::Value(item.trim()));
        }
        let curr = root
            .first_child()
            .ok_or(DeserializerError::ExpectedNonEmptyTuple)?
            .node_id();
        Ok(TupleParser(A3daTree { tree, curr }, false))
    }
}

impl<'de> SeqAccess<'de> for TupleParser<'de> {
    type Error = DeserializerError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.1 {
            return Ok(None);
        }
        let temp = self.0.curr;
        let val = seed.deserialize(&mut self.0)?;
        match self.0.tree.get(temp).unwrap().next_sibling() {
            Some(n) => self.0.curr = n.node_id(),
            None => self.1 = true,
        };
        Ok(Some(val))
    }
}

impl<'de, 'a> MapAccess<'de> for MapParser<'de, 'a> {
    type Error = DeserializerError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.end {
            return Ok(None);
        }
        let val = seed.deserialize(&mut *self.tree)?;
        let node = self.tree.get();
        self.next = node.next_sibling().map(|x| x.node_id());
        self.tree.curr = node.first_child().unwrap().node_id();
        Ok(Some(val))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.tree)?;
        match self.next {
            Some(n) => self.tree.curr = n,
            None => self.end = true,
        };
        Ok(val)
    }
}

impl<'de, 'a> EnumAccess<'de> for EnumParser<'de, 'a> {
    type Error = DeserializerError;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        let node = self.de.get();
        self.de.curr = node.first_child().unwrap().node_id();
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for EnumParser<'de, 'a> {
    type Error = DeserializerError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(DeserializerError::ExpectedValueNode)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        println!("newtyper");
        dbg!(self.de.get_key());
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("tuple enm");
        serde::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("struct enm");
        serde::Deserializer::deserialize_map(self.de, visitor)
    }
}

#[cfg(test)]
mod tests {
    use ::serde::Deserialize;
    use serde_derive::Deserialize;

    use super::*;

    #[test]
    fn read_bool() {
        let input = "test=true";
        let mut tree = A3daTree::new(input).unwrap();
        tree.curr = tree.get().first_child().unwrap().node_id();
        assert!(bool::deserialize(&mut tree).unwrap())
    }

    #[test]
    fn read_seq() {
        let input = "0 = a
1=b
2=c
welcome = banana";
        let mut tree = A3daTree::new(input).unwrap();
        tree.print();
        let data: Vec<char> = Vec::deserialize(&mut tree).unwrap();
        assert_eq!(data, vec!['a', 'b', 'c']);
    }

    #[test]
    fn read_int() {
        let input = "0=123
1=+69
2=-32";
        let mut tree = A3daTree::new(input).unwrap();
        tree.print();
        let data: Vec<i64> = Vec::deserialize(&mut tree).unwrap();
        assert_eq!(data, vec![123, 69, -32]);
    }

    #[test]
    fn read_float() {
        let input = "0=0.0
1=+1.234
2=-1.234
3=6.02e23";
        let mut tree = A3daTree::new(input).unwrap();
        tree.print();
        let data: Vec<f32> = Vec::deserialize(&mut tree).unwrap();
        assert_eq!(data, vec![0.0, 1.234, -1.234, 6.02e23]);
    }

    #[test]
    fn read_tuple() {
        let input = "test=(123, 1.234, hello)";
        let mut tree = A3daTree::new(input).unwrap();
        tree.curr = tree.get().first_child().unwrap().node_id();
        tree.print();
        let data: (u32, f32, &str) = Deserialize::deserialize(&mut tree).unwrap();
        assert_eq!(data, (123, 1.234, "hello"));
    }

    #[test]
    fn read_map() {
        use std::collections::HashMap;

        let input = "one = 1
two = 2
three = 3";
        let mut tree = A3daTree::new(input).unwrap();
        let data: HashMap<String, u32> = HashMap::deserialize(&mut tree).unwrap();
        let mut expected = HashMap::new();
        expected.insert("one".to_string(), 1);
        expected.insert("two".to_string(), 2);
        expected.insert("three".to_string(), 3);
        assert_eq!(data, expected)
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
        let mut tree = A3daTree::new(input).unwrap();
        let data = Test::deserialize(&mut tree).unwrap();
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

    #[test]
    fn read_enum() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            bars: Vec<Bar>,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        enum Bar {
            None,
            Foo(u32),
            Bar(u32, f32),
            Baz(String),
            Quux { foo: u32, bar: f32 },
            Foobar(Foobar),
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foobar {
            foo: u32,
            bar: f32,
        }
        let input = "bars.0=None
bars.1.Foo=123
bars.2.Bar=(123, 3.1415)
bars.3.Baz=Hello World!
bars.4.Quux.foo=123
bars.4.Quux.bar=3.1415
bars.5.Foobar.foo=123
bars.5.Foobar.bar=3.1415
";
        let mut tree = A3daTree::new(input).unwrap();
        let data = Test::deserialize(&mut tree).unwrap();
        let expected = Test {
            bars: vec![
                Bar::None,
                Bar::Foo(123),
                Bar::Bar(123, 3.1415),
                Bar::Baz("Hello World!".to_string()),
                Bar::Quux {
                    foo: 123,
                    bar: 3.1415,
                },
                Bar::Foobar(Foobar {
                    foo: 123,
                    bar: 3.1415,
                }),
            ],
        };
        assert_eq!(data, expected);
    }

    #[test]
    fn read_example() {
        use crate::de::tests::INPUT;
        #[derive(Debug, PartialEq, Deserialize)]
        struct A3da {
            camera_root: Vec<CameraRoot>,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct CameraRoot {
            interest: Interest,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Interest {
            #[serde(rename = "trans")]
            translation: Vec3,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Vec3 {
            x: Keys,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Keys {
            key: Vec<Key>,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Key {
            #[serde(rename = "type")]
            ty: u32,
            data: String,
        }

        let mut tree = A3daTree::new(INPUT).unwrap();
        let a3da = A3da::deserialize(&mut tree).unwrap();
        let expected = A3da {
            camera_root: vec![CameraRoot {
                interest: Interest {
                    translation: Vec3 {
                        x: Keys {
                            key: vec![
                                Key {
                                    ty: 1,
                                    data: "(0,-0.469822)".to_string(),
                                },
                                Key {
                                    ty: 2,
                                    data: "(738,-0.522281,3.31402e-006)".to_string(),
                                },
                            ],
                        },
                    },
                },
            }],
        };
        assert_eq!(a3da, expected);
    }
}
