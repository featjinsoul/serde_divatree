#[cfg(feature = "miette")]
use miette::Diagnostic;
use serde::{de, ser};
use thiserror::*;

use std::fmt::{self, Display};
use std::num::{ParseFloatError, ParseIntError};
use std::ops::Range;

#[derive(Debug, Default, Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[error("Syntax error in line {line_num}. `{line}`")]
pub struct ParseError {
    pub line_num: usize,
    pub line: String,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "miette", derive(Diagnostic))]
#[cfg_attr(
    feature = "miette",
    diagnostic(help("maybe if you stanned loona you wouldn't have broken this 💅"))
)]
pub enum DeserializerError {
    #[error("An internal parser error occured.")]
    ParseError(#[from] ParseError),
    #[cfg_attr(feature = "miette", diagnostic(code(divatree::parser::key_value)))]
    #[error("Expected a key value pair")]
    ExpectedKeyValuePair,
    #[cfg_attr(feature = "miette", diagnostic(code(divatree::parser::value)))]
    #[error("Expected a value node. Found a key node instead.")]
    ExpectedValueNode,
    #[cfg_attr(feature = "miette", diagnostic(code(divatree::parser::key)))]
    #[error("Expected a key node. Found a value node instead.")]
    ExpectedKeyNode,
    #[cfg_attr(feature = "miette", diagnostic(code(divatree::parser::sequence)))]
    #[error("Expected a sequence, found something else")]
    ExpectedSequenece {
        #[cfg_attr(feature = "miette", label("This was unexpected."))]
        unexpected: Range<usize>,
    },
    #[cfg_attr(feature = "miette", diagnostic(code(divatree::parser::atom)))]
    #[error("Failed to parse an atom")]
    ParseAtomError(
        #[from]
        #[cfg_attr(feature = "miette", diagnostic_source)]
        super::de::serde::atom::ParseAtomError,
    ),
    #[error("{0}")]
    Custom(String),
}

impl de::Error for DeserializerError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}
