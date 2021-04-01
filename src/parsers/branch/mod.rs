pub use alternative::*;

#[cfg(feature = "alloc")]
use crate::parsers::helpers::map_result;
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

/// Executes the `then` parser while `condition` matches. This method discards `condition` results.
#[cfg(feature = "alloc")]
pub fn branch_while<'a, C, Rc, R>(
    condition: impl FnMut(&mut Reader<'a, C>) -> ParserResult<Rc>,
    then: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Vec<R>> {
    crate::parsers::sequence::repeat(
        ..,
        map_result(
            crate::parsers::sequence::tuple((condition, then)),
            |(_, v)| v,
        ),
    )
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

#[cfg(test)]
#[cfg(feature = "alloc")]
mod test_alloc {
    use crate::parsers::characters::read_text;

    use super::*;

    #[test]
    fn test_branch_while() {
        let mut reader = Reader::new("abcdabcda");
        let mut parser = branch_while(read_text("a"), read_text("bcd"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(vec!["bcd", "bcd"]));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(vec![]));
    }
}
