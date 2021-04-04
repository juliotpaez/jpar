pub use alternatives::*;
pub use alternatives_ignore::*;

#[cfg(feature = "alloc")]
use crate::parsers::helpers::map_result;
use crate::parsers::helpers::not_found_restore;
use crate::result::{ParserResult, ParserResultError};
use crate::ParserInput;

mod alternatives;
mod alternatives_ignore;

/// Executes the `condition` parser and if it match, discards its value and parses `then`.
pub fn branch_if<'a, Cond, Then, C, R, Rc, Err>(
    mut condition: Cond,
    mut then: Then,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<Option<R>, Err>
where
    Cond: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<Rc, Err>,
    Then: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    not_found_restore(move |reader| match condition(reader) {
        Ok(_) => Ok(Some(then(reader)?)),
        Err(ParserResultError::NotFound) => Ok(None),
        Err(e) => Err(e),
    })
}

/// Executes the `condition` parser. If it matches, discards its value and parses `then`, otherwise parses `else`.
pub fn branch_if_else<'a, Cond, Then, Else, C, R, Rc, Err>(
    mut condition: Cond,
    mut then: Then,
    mut else_parser: Else,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>
where
    Cond: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<Rc, Err>,
    Then: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    Else: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    not_found_restore(move |reader| match condition(reader) {
        Ok(_) => then(reader),
        Err(ParserResultError::NotFound) => else_parser(reader),
        Err(e) => Err(e),
    })
}

/// Executes the `then` parser while `condition` matches. This method discards `condition` results.
#[cfg(feature = "alloc")]
pub fn branch_while<'a, Cond, Then, C, Rc, R, Err>(
    condition: Cond,
    then: Then,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<Vec<R>, Err>
where
    Cond: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<Rc, Err>,
    Then: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    crate::parsers::sequence::repeat(
        ..,
        map_result(
            crate::parsers::sequence::tuple((condition, then)),
            |_, (_, v)| v,
        ),
    )
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
        let mut reader = ParserInput::new("This is a test");
        let mut parser = branch_if(|_| Ok(()), read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(Some("This")));

        let mut parser = branch_if(
            |_| -> ParserResult<(), ()> { Err(ParserResultError::NotFound) },
            read_text("This"),
        );
        let result = parser(&mut reader);
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_branch_if_else() {
        let mut reader = ParserInput::new("This is a test");
        let mut parser = branch_if_else(|_| Ok(()), read_text("This"), read_text("Th"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));

        let mut reader = ParserInput::new("This is a test");
        let mut parser = branch_if_else(
            |_| -> ParserResult<(), ()> { Err(ParserResultError::NotFound) },
            read_text("This"),
            read_text("Th"),
        );
        let result = parser(&mut reader);
        assert_eq!(result, Ok("Th"));
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod test_alloc {
    use crate::parsers::characters::read_text;

    use super::*;

    #[test]
    fn test_branch_while() {
        let mut reader = ParserInput::new("abcdabcda");
        let mut parser = branch_while(read_text("a"), read_text("bcd"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok(vec!["bcd", "bcd"]));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(vec![]));
    }
}
