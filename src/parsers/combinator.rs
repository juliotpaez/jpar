use crate::parsers::Quantifier;
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

/// Maps the result of a parser into a new value.
pub fn map_result<'a, C, R, Rf>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut mapper: impl FnMut(R) -> Rf,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Rf> {
    move |reader| {
        let result = parser(reader)?;

        Ok(mapper(result))
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
pub fn verify<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut verifier: impl FnMut(&R) -> bool,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |reader| {
        let init_cursor = reader.save_cursor();
        let result = parser(reader)?;

        if verifier(&result) {
            Ok(result)
        } else {
            reader.restore(init_cursor);
            Err(ParserResultError::NotFound)
        }
    }
}

/// Returns the provided value if the child parser succeeds.
pub fn value<'a, C, R, V: Clone>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    value: V,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<V> {
    move |reader| {
        let _ = parser(reader)?;

        Ok(value.clone())
    }
}

/// Always succeeds with given value without consuming any input.
pub fn success<'a, C, R: Clone>(value: R) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |_| Ok(value.clone())
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

/// Repeats a parser a quantified number of times.
#[cfg(feature = "alloc")]
pub fn repeat<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    quantifier: impl Into<Quantifier>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Vec<R>> {
    let quantifier = quantifier.into();

    move |reader| {
        let mut result = Vec::new();

        while !quantifier.is_finished(result.len()) {
            result.push(match parser(reader) {
                Ok(v) => v,
                Err(ParserResultError::NotFound) => break,
                Err(e) => return Err(e),
            });
        }

        if quantifier.contains(result.len()) {
            Ok(result)
        } else {
            Err(ParserResultError::NotFound)
        }
    }
}

/// Repeats a parser a quantified number of times and returns it.
pub fn repeat_and_count<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    quantifier: impl Into<Quantifier>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<usize> {
    let quantifier = quantifier.into();

    move |reader| {
        let mut result = 0;
        while !quantifier.is_finished(result) {
            match parser(reader) {
                Ok(_) => {}
                Err(ParserResultError::NotFound) => break,
                Err(e) => return Err(e),
            }

            result += 1;
        }

        if quantifier.contains(result) {
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
    use crate::parsers::characters::ascii_alpha_quantified;
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
    fn test_map_result() {
        let mut reader = Reader::new("This is a test");
        let mut parser = map_result(ascii_alpha_quantified(1..), |_| 32);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_not_consume() {
        let mut reader = Reader::new("This is a test");
        let mut parser = not_consume(ascii_alpha_quantified(1..));

        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));
        assert_eq!(reader.byte_offset(), 0);
    }

    #[test]
    fn test_verify() {
        let mut reader = Reader::new("This is a test");
        let mut parser = verify(ascii_alpha_quantified(1..), |x| *x == "This");

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
    fn test_value() {
        let mut reader = Reader::new("This is a test");
        let mut parser = value(ascii_alpha_quantified(1..), true);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(true));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_success() {
        let mut reader = Reader::new("This is a test");
        let mut parser = success(true);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(true));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_optional() {
        let mut reader = Reader::new("This is a test");
        let mut parser = optional(ascii_alpha_quantified(1..));

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
        let mut parser = optional_default(ascii_alpha_quantified(1..));

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
        let mut parser = not(ascii_alpha_quantified(1..));

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
        let mut parser = all_consumed(ascii_alpha_quantified(1..));

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

    #[test]
    fn test_repeat_and_count() {
        let mut reader = Reader::new("Test");
        let mut parser = repeat_and_count(ascii_alpha_quantified(1), 1..);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(4));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_repeat_and_count_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = repeat_and_count(
            |reader| -> ParserResult<()> {
                Err(ParserResultError::Error(ParserError {
                    origin: "".into(),
                    cursor: reader.save_cursor(),
                }))
            },
            ..,
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
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod test_alloc {
    use crate::parsers::characters::ascii_alpha;
    use crate::result::ParserError;

    use super::*;

    #[test]
    fn test_repeat() {
        let mut reader = Reader::new("Test");
        let mut parser = repeat(ascii_alpha, 3);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(vec!['T', 'e', 's']));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_repeat_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = repeat(
            |reader| -> ParserResult<()> {
                Err(ParserResultError::Error(ParserError {
                    origin: "".into(),
                    cursor: reader.save_cursor(),
                }))
            },
            3,
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
}
