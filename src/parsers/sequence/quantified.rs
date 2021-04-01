use crate::parsers::helpers::not_found_restore;
use crate::parsers::Quantifier;
use crate::result::{ParserResult, ParserResultError};
use crate::Reader;

/// Repeats a parser a quantified number of times.
#[cfg(feature = "alloc")]
pub fn repeat<'a, C, R>(
    quantifier: impl Into<Quantifier>,
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Vec<R>> {
    let quantifier = quantifier.into();

    not_found_restore(move |reader| {
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
    })
}

/// Repeats a parser a quantified number of times and returns it.
pub fn repeat_and_count<'a, C, R>(
    quantifier: impl Into<Quantifier>,
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<usize> {
    let quantifier = quantifier.into();

    not_found_restore(move |reader| {
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
    })
}

/// Repeats a parser to fill a slice.
pub fn repeat_to_fill<'a, C, R>(
    buffer: &'a mut [R],
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()> {
    not_found_restore(move |reader| {
        for i in 0..buffer.len() {
            match parser(reader) {
                Ok(v) => {
                    buffer[i] = v;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    })
}

/// Applies a parser until it fails and accumulates the results using a given function and initial value.
pub fn repeat_and_fold<'a, C, R: Clone, Rp>(
    quantifier: impl Into<Quantifier>,
    init: R,
    fold: impl Fn(R, Rp) -> R,
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<Rp>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    let quantifier = quantifier.into();

    not_found_restore(move |reader| {
        let mut result = init.clone();
        let mut iteration = 0;
        while !quantifier.is_finished(iteration) {
            let res = match parser(reader) {
                Ok(v) => v,
                Err(ParserResultError::NotFound) => break,
                Err(e) => return Err(e),
            };

            result = fold(result, res);
            iteration += 1;
        }

        if quantifier.contains(iteration) {
            Ok(result)
        } else {
            Err(ParserResultError::NotFound)
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::{ascii_alpha_quantified, read_any};
    use crate::result::ParserError;

    use super::*;

    #[test]
    fn test_repeat_and_count() {
        let mut reader = Reader::new("Test");
        let mut parser = repeat_and_count(1.., ascii_alpha_quantified(1));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(4));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_repeat_and_count_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = repeat_and_count(.., |reader| -> ParserResult<()> {
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
    fn test_repeat_to_fill() {
        let mut reader = Reader::new("This is a test");
        let mut buffer = [0; 5];

        let result = {
            let mut parser = repeat_to_fill(&mut buffer, |_| Ok(3));
            parser(&mut reader)
        };

        assert_eq!(result, Ok(()));
        assert_eq!(buffer, [3; 5]);
    }

    #[test]
    fn test_repeat_to_fill_not_found() {
        let mut reader = Reader::new("This is a test");
        let mut buffer = [0; 5];

        let result = {
            let mut parser = repeat_to_fill(&mut buffer, |_| Err(ParserResultError::NotFound));
            parser(&mut reader)
        };

        assert_eq!(result, Err(ParserResultError::NotFound));
        assert_eq!(buffer, [0; 5]);
    }

    #[test]
    fn test_repeat_and_fold() {
        let mut reader = Reader::new("This is a test");

        let result = {
            let mut parser = repeat_and_fold(
                ..=4,
                String::new(),
                |a, b| format!("{}{}", a, b),
                read_any(),
            );
            parser(&mut reader)
        };

        assert_eq!(result, Ok("This".to_string()));
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
        let mut parser = repeat(3, ascii_alpha);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(vec!['T', 'e', 's']));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_repeat_error() {
        let mut reader = Reader::new("This is a test");
        let mut parser = repeat(3, |reader| -> ParserResult<()> {
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
