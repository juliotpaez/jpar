use crate::Cursor;

/// The result of every parser method.
pub type ParserResult<T, Err = ()> = Result<T, ParserResultError<Err>>;

/// The type of errors that parser method can return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserResultError<Err> {
    NotFound,
    Error((Cursor, Err)),
}
