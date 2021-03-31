pub use alternative::*;

use crate::result::ParserResult;
use crate::Reader;

mod alternative;

/// Executes the parser depending on a condition. Returns None when condition is false.
pub fn branch_if<'a, C, R>(
    mut condition: impl FnMut(&mut Reader<'a, C>) -> bool,
    mut then: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Option<R>> {
    move |reader| {
        if condition(reader) {
            Ok(Some(then(reader)?))
        } else {
            Ok(None)
        }
    }
}

/// Executes `then` or `else_parser` depending on a condition.
pub fn branch_if_else<'a, C, R>(
    mut condition: impl FnMut(&mut Reader<'a, C>) -> bool,
    mut then: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut else_parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<(bool, R)> {
    move |reader| {
        if condition(reader) {
            Ok((true, then(reader)?))
        } else {
            Ok((false, else_parser(reader)?))
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::read_text;

    use super::*;

    #[test]
    fn test_branch_if() {
        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if(|_| true, read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(Some("This")));

        let mut parser = branch_if(|_| false, read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_branch_if_else() {
        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if_else(|_| true, read_text("This"), read_text("Th"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok((true, "This")));

        let mut reader = Reader::new("This is a test");
        let mut parser = branch_if_else(|_| false, read_text("This"), read_text("Th"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok((false, "Th")));
    }
}
