pub use alternative::*;

use crate::parsers::helpers::not_found_restore;
use crate::result::ParserResult;
use crate::Reader;

mod alternative;

/// Executes the `condition` parser that returns a bool and depending on that executes or not the `then` parser.
/// Returns None when condition is false.
pub fn branch_if<'a, C, R>(
    mut condition: impl FnMut(&mut Reader<'a, C>) -> ParserResult<bool>,
    mut then: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Option<R>> {
    not_found_restore(move |reader| {
        if condition(reader)? {
            Ok(Some(then(reader)?))
        } else {
            Ok(None)
        }
    })
}

/// Executes the `condition` parser that returns a bool and depending on that executes either `then` or `else_parser` parsers.
pub fn branch_if_else<'a, C, R>(
    mut condition: impl FnMut(&mut Reader<'a, C>) -> ParserResult<bool>,
    mut then: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut else_parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<(bool, R)> {
    not_found_restore(move |reader| {
        if condition(reader)? {
            Ok((true, then(reader)?))
        } else {
            Ok((false, else_parser(reader)?))
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::read_text;
    use crate::result::ParserResultError;

    use super::*;

    #[test]
    fn test_branch_if() {
        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if(|_| Ok(true), read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(Some("This")));

        let mut parser = branch_if(|_| Ok(false), read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(None));

        let mut parser = branch_if(|_| Err(ParserResultError::NotFound), read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_branch_if_else() {
        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if_else(|_| Ok(true), read_text("This"), read_text("Th"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok((true, "This")));

        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if_else(|_| Ok(false), read_text("This"), read_text("Th"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok((false, "Th")));

        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if_else(
            |_| Err(ParserResultError::NotFound),
            read_text("This"),
            read_text("Th"),
        );
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }
}
