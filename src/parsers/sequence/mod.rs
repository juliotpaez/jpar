pub use repeats::*;
pub use tuples::*;

use crate::parsers::helpers::not_found_restore;
use crate::result::ParserResult;
use crate::Reader;

mod repeats;
mod tuples;

/// Matches an object from the first parser and discards it,
/// then gets an object from the second parser, and finally
/// matches an object from the third parser and discards it.
pub fn delimited<'a, Pre, Con, Pos, C, RPre, R, RPos>(
    mut prefix: Pre,
    mut content: Con,
    mut postfix: Pos,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    Pre: FnMut(&mut Reader<'a, C>) -> ParserResult<RPre>,
    Con: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    Pos: FnMut(&mut Reader<'a, C>) -> ParserResult<RPos>,
{
    not_found_restore(move |reader| {
        let _ = prefix(reader)?;
        let result = content(reader)?;
        let _ = postfix(reader)?;

        Ok(result)
    })
}

/// Matches an object from the first parser and discards it, then gets an object from the second parser.
pub fn preceded<'a, Pre, Con, C, RPre, R>(
    mut prefix: Pre,
    mut content: Con,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    Pre: FnMut(&mut Reader<'a, C>) -> ParserResult<RPre>,
    Con: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
{
    not_found_restore(move |reader| {
        let _ = prefix(reader)?;
        content(reader)
    })
}

/// Gets an object from the first parser, then matches an object from the second parser and discards it.
pub fn terminated<'a, Con, Pos, C, R, RPos>(
    mut content: Con,
    mut postfix: Pos,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    Con: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    Pos: FnMut(&mut Reader<'a, C>) -> ParserResult<RPos>,
{
    not_found_restore(move |reader| {
        let result = content(reader)?;
        let _ = postfix(reader)?;

        Ok(result)
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::{ascii_alpha1, read_text};
    use crate::result::ParserResultError;

    use super::*;

    #[test]
    fn test_delimited() {
        let mut reader = Reader::new("(abcd)");
        let mut parser = delimited(read_text("("), ascii_alpha1, read_text(")"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok("abcd"));

        let mut reader = Reader::new("abcd)");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = Reader::new("()");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = Reader::new("(abcd");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_preceded() {
        let mut reader = Reader::new("(abcd)");
        let mut parser = preceded(read_text("("), ascii_alpha1);
        let result = parser(&mut reader);
        assert_eq!(result, Ok("abcd"));

        let mut reader = Reader::new("abcd)");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = Reader::new("()");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_terminated() {
        let mut reader = Reader::new("abcd)");
        let mut parser = terminated(ascii_alpha1, read_text(")"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok("abcd"));

        let mut reader = Reader::new("abcd");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = Reader::new(")");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }
}
