use crate::Cursor;

/// The result of every parser method.
pub type ParserResult<T> = Result<T, ParserResultError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// Position of the error in the input data.
    pub cursor: Cursor,
}

/// The type of errors that parser method can return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserResultError {
    NotFound,
    Error(ParserError),
}
