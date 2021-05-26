use thiserror::*;

#[derive(Debug, Error)]
#[error("Syntax error in line {line_num}. `{line}`")]
pub struct ParseError<'a> {
    pub line_num: usize,
    pub line: &'a str,
}
