pub use quantified::*;
pub use tuple::*;

use crate::parsers::helpers::not_found_restore;
use crate::result::ParserResult;
use crate::Reader;

mod quantified;
mod tuple;

/// Matches an object from the first parser and discards it,
/// then gets an object from the second parser, and finally
/// matches an object from the third parser and discards it.
pub fn delimited<'a, C, RPre, R, RPos>(
    mut prefix: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RPre>,
    mut content: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut postfix: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RPos>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    not_found_restore(move |reader| {
        let _ = prefix(reader)?;
        let result = content(reader)?;
        let _ = postfix(reader)?;

        Ok(result)
    })
}

/// Matches an object from the first parser and discards it, then gets an object from the second parser.
pub fn preceded<'a, C, RPre, R>(
    mut prefix: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RPre>,
    mut content: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    not_found_restore(move |reader| {
        let _ = prefix(reader)?;
        content(reader)
    })
}

/// Gets an object from the first parser, then matches an object from the second parser and discards it.
pub fn terminated<'a, C, R, RPos>(
    mut content: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut postfix: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RPos>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
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
    use crate::parsers::characters::{ascii_alpha_quantified, read_text};
    use crate::result::ParserResultError;

    use super::*;

    #[test]
    fn test_delimited() {
        let mut reader = Reader::new("(abcd)");
        let mut parser = delimited(read_text("("), ascii_alpha_quantified(1..), read_text(")"));
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
        let mut parser = preceded(read_text("("), ascii_alpha_quantified(1..));
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
        let mut parser = terminated(ascii_alpha_quantified(1..), read_text(")"));
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
