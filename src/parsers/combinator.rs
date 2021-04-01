use crate::parsers::helpers::not_found_restore;
use crate::result::{ParserResult, ParserResultError};
use crate::Reader;

/// It is ok only at the end of the input.
pub fn end<C>(reader: &mut Reader<C>) -> ParserResult<()> {
    if reader.is_end() {
        Ok(())
    } else {
        Err(ParserResultError::NotFound)
    }
}

/// Executes the parser and returns its value not consuming any character in the process.
pub fn not_consume<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |reader| {
        let init_cursor = reader.save_cursor();
        let result = parser(reader)?;
        reader.restore(init_cursor);

        Ok(result)
    }
}

/// Returns the result of the child parser if it satisfies a verification function.
pub fn verify<'a, C, R, P, V>(
    mut parser: P,
    mut verifier: V,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    V: FnMut(&R) -> bool,
{
    not_found_restore(move |reader| {
        let result = parser(reader)?;

        if verifier(&result) {
            Ok(result)
        } else {
            Err(ParserResultError::NotFound)
        }
    })
}

/// Returns None when the the parser is not found.
pub fn optional<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Option<R>> {
    move |reader| match parser(reader) {
        Ok(v) => Ok(Some(v)),
        Err(ParserResultError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Returns the default value of `R` when the the parser is not found.
pub fn optional_default<'a, C, R: Default>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |reader| match parser(reader) {
        Ok(v) => Ok(v),
        Err(ParserResultError::NotFound) => Ok(R::default()),
        Err(e) => Err(e),
    }
}

/// Returns the default value of `R` when the the parser is not found.
pub fn not<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()> {
    move |reader| match parser(reader) {
        Ok(_) => Err(ParserResultError::NotFound),
        Err(ParserResultError::NotFound) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Succeeds if all the input has been consumed by its child parser.
pub fn all_consumed<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
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
    use crate::result::ParserError;

    use super::*;

    #[test]
    fn test_end() {
        let mut reader = Reader::new("This is a test");
        let result = end(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = Reader::new("");
        let result = end(&mut reader);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_not_consume() {
        let mut reader = Reader::new("This is a test");
        let mut parser = not_consume(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));
        assert_eq!(reader.byte_offset(), 0);
    }

    #[test]
    fn test_verify() {
        let mut reader = Reader::new("This is a test");
        let mut parser = verify(ascii_alpha1, |x| *x == "This");

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_verify_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = verify(
            |reader| {
                Err(ParserResultError::Error(ParserError {
                    origin: "".into(),
                    cursor: reader.save_cursor(),
                }))
            },
            |x: &&str| *x == "This",
        );

        let result = parser(&mut reader);
        assert_eq!(
            result,
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        );
    }

    #[test]
    fn test_optional() {
        let mut reader = Reader::new("This is a test");
        let mut parser = optional(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(Some("This")));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_optional_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = optional(|reader| {
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        });

        let result: ParserResult<Option<()>> = parser(&mut reader);
        assert_eq!(
            result,
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        );
    }

    #[test]
    fn test_optional_default() {
        let mut reader = Reader::new("This is a test");
        let mut parser = optional_default(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(""));
    }

    #[test]
    fn test_optional_default_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = optional_default(|reader| {
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        });

        let result: ParserResult<Option<()>> = parser(&mut reader);
        assert_eq!(
            result,
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        );
    }

    #[test]
    fn test_not() {
        let mut reader = Reader::new("This is a test");
        let mut parser = not(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let mut reader = Reader::new("   test2");
        let result = parser(&mut reader);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_not_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = not(|reader| -> ParserResult<()> {
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        });

        let result = parser(&mut reader);
        assert_eq!(
            result,
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        );
    }

    #[test]
    fn test_all_consumed() {
        let mut reader = Reader::new("Test");
        let mut parser = all_consumed(ascii_alpha1);

        let result = parser(&mut reader);
        assert_eq!(result, Ok("Test"));

        let mut reader = Reader::new("   test2");
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_all_consumed_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = all_consumed(|reader| -> ParserResult<()> {
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        });

        let result = parser(&mut reader);
        assert_eq!(
            result,
            Err(ParserResultError::Error(ParserError {
                origin: "".into(),
                cursor: reader.save_cursor(),
            }))
        );
    }
}
