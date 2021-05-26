use thiserror::*;


#[derive(Debug, Default, Error, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[error("Syntax error in line {line_num}. `{line}`")]
pub struct ParseError {
    pub line_num: usize,
    pub line: String,
}
}
