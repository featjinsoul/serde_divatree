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
    todo!()
}
