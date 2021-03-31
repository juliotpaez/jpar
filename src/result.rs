use std::borrow::Cow;

use crate::Cursor;

/// The result of every parser method.
pub type ParserResult<T> = Result<T, ParserResultError>;

/// The type of errors that parser method can return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserResultError {
    NotFound,
    Error(ParserError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// The name of the method that has failed.
    pub origin: Cow<'static, str>,

    /// Position of the error in the input data.
    pub cursor: Cursor,
}
