use crate::parsers::helpers::not_found_restore;
use crate::result::{ParserResult, ParserResultError};
use crate::ParserInput;

/// It is ok only at the end of the input.
pub fn end<C, Err>(reader: &mut ParserInput<Err, C>) -> ParserResult<(), Err> {
    if reader.is_end() {
        Ok(())
    } else {
        Err(ParserResultError::NotFound)
    }
}

/// Executes the parser and returns its value not consuming any character in the process.
pub fn not_consume<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| {
        let init_cursor = reader.save_cursor();
        let result = parser(reader)?;
        reader.restore(init_cursor);

        Ok(result)
    }
}

/// Returns the result of the child parser if it satisfies a verification function.
pub fn verify<'a, C, R, P, V, Err>(
    mut parser: P,
    mut verifier: V,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    V: FnMut(&mut ParserInput<'a, Err, C>, &R) -> bool,
{
    not_found_restore(move |reader| {
        let result = parser(reader)?;

        if verifier(reader, &result) {
            Ok(result)
        } else {
            Err(ParserResultError::NotFound)
        }
    })
}

/// Returns None when the the parser is not found.
pub fn optional<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<Option<R>, Err>
where
    P: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| match parser(reader) {
        Ok(v) => Ok(Some(v)),
        Err(ParserResultError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Returns the default value of `R` when the the parser is not found.
pub fn optional_default<'a, P, C, R: Default, Err>(
    mut parser: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| match parser(reader) {
        Ok(v) => Ok(v),
        Err(ParserResultError::NotFound) => Ok(R::default()),
        Err(e) => Err(e),
    }
}

/// Returns the default value of `R` when the the parser is not found.
pub fn not<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<(), Err>
where
    P: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| match parser(reader) {
        Ok(_) => Err(ParserResultError::NotFound),
        Err(ParserResultError::NotFound) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Succeeds if all the input has been consumed by its child parser.
pub fn all_consumed<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| {
        let result = parser(reader)?;

        if reader.is_end() {
            Ok(result)
        } else {
            Err(ParserResultError::NotFound)
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::ascii_alpha1;

    use super::*;

    #[test]
    fn test_end() {
        let mut reader = ParserInput::new("This is a test");
        let result = end(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = ParserInput::new("");
        let result = end(&mut reader);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_not_consume() {
        let mut reader = ParserInput::new("This is a test");
        let mut parser = not_consume(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));
        assert_eq!(reader.byte_offset(), 0);
    }

    #[test]
    fn test_verify() {
        let mut reader = ParserInput::new("This is a test");
        let mut parser = verify(ascii_alpha1, |_, x| *x == "This");

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_optional() {
        let mut reader = ParserInput::new("This is a test");
        let mut parser = optional(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(Some("This")));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_optional_default() {
        let mut reader = ParserInput::new("This is a test");
        let mut parser = optional_default(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(""));
    }

    #[test]
    fn test_not() {
        let mut reader = ParserInput::new("This is a test");
        let mut parser = not(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = ParserInput::new("   test2");
        let result = parser(&mut reader);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_all_consumed() {
        let mut reader = ParserInput::new("Test");
        let mut parser = all_consumed(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok("Test"));

        let mut reader = ParserInput::new("   test2");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }
}
