use crate::result::{ParserResult, ParserResultError};
use crate::{Cursor, Reader};

/// Restores the reader when a not found error is returned.
pub fn not_found_restore<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| {
        let init_cursor = reader.save_cursor();

        match parser(reader) {
            Ok(v) => Ok(v),
            Err(ParserResultError::NotFound) => {
                reader.restore(init_cursor);
                Err(ParserResultError::NotFound)
            }
            Err(e) => Err(e),
        }
    }
}

/// Maps the result of a parser into a new value.
pub fn map_result<'a, P, M, C, R, Rf, Err>(
    mut parser: P,
    mut mapper: M,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<Rf, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
    M: FnMut(&mut Reader<'a, Err, C>, R) -> Rf,
{
    move |reader| {
        let result = parser(reader)?;

        Ok(mapper(reader, result))
    }
}

/// Maps the result of a parser into a new `ParserResult`.
pub fn and_then<'a, P, M, C, R, Rf, Err>(
    mut parser: P,
    mut mapper: M,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<Rf, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
    M: FnMut(&mut Reader<'a, Err, C>, R) -> ParserResult<Rf, Err>,
{
    not_found_restore(move |reader| {
        let result = parser(reader)?;

        mapper(reader, result)
    })
}

/// Applies a parser over the result of another one.
pub fn map_parser<'a, O, P, C: Clone, R, Err>(
    mut origin: O,
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>
where
    O: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err>,
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
{
    not_found_restore(move |reader| {
        let result = origin(reader)?;
        let mut new_reader = Reader::new_with_context_and_error(result, reader.context().clone());

        parser(&mut new_reader)
    })
}

/// Applies a parser discarding its result and return the consumed content as result.
pub fn consumed<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
{
    move |reader| {
        let init_cursor = reader.save_cursor();
        match parser(reader) {
            Ok(_) => Ok(reader.substring_to_current(&init_cursor).content()),
            Err(ParserResultError::NotFound) => {
                reader.restore(init_cursor);
                Err(ParserResultError::NotFound)
            }
            Err(e) => Err(e),
        }
    }
}

/// Applies a parser discarding its result.
pub fn ignore_result<'a, P, C, R, Err>(
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<(), Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
{
    not_found_restore(move |reader| {
        let _ = parser(reader)?;
        Ok(())
    })
}

/// Always succeeds with given value without consuming any input.
pub fn value<'a, C, R: Clone, Err>(
    value: R,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err> {
    move |_| Ok(value.clone())
}

/// Always fails with the given error without consuming any input.
pub fn error<'a, C, R, Err: Clone>(
    error: Err,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err> {
    move |reader| {
        Err(ParserResultError::Error((
            reader.save_cursor(),
            error.clone(),
        )))
    }
}

/// Always fails with the given error without consuming any input.
/// The error is built dynamically.
pub fn error_dyn<'a, C, R, Err, EFn>(
    mut error_fn: EFn,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>
where
    EFn: FnMut(&mut Reader<'a, Err, C>) -> Err,
{
    move |reader| {
        Err(ParserResultError::Error((
            reader.save_cursor(),
            error_fn(reader),
        )))
    }
}

/// Ensures that `parser` always success or returns an error.
pub fn ensure<'a, P, C, R, Efn, Err>(
    mut parser: P,
    mut error_fn: Efn,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
    Efn: FnMut(&mut Reader<'a, Err, C>) -> Err,
{
    move |reader| match parser(reader) {
        Ok(v) => Ok(v),
        Err(ParserResultError::NotFound) => Err(ParserResultError::Error((
            reader.save_cursor(),
            error_fn(reader),
        ))),
        Err(e) => Err(e),
    }
}

/// Applies a parser but allowing to recover in case of an error.
pub fn recover<'a, P, C, R, Rfn, Err>(
    mut parser: P,
    mut recover_fn: Rfn,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
    Rfn: FnMut(&mut Reader<'a, Err, C>, Cursor, Err) -> ParserResult<R, Err>,
{
    not_found_restore(move |reader| match parser(reader) {
        Ok(v) => Ok(v),
        Err(ParserResultError::NotFound) => Err(ParserResultError::NotFound),
        Err(ParserResultError::Error((cursor, e))) => recover_fn(reader, cursor, e),
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::{
        ascii_alpha1, ascii_alpha_quantified, read_any, read_any_quantified, read_text,
    };
    use crate::parsers::sequence::tuple;
    use crate::ParserResultError;

    use super::*;

    #[test]
    fn test_not_found_restore() {
        let mut reader = Reader::new("This is a test");
        let mut parser = not_found_restore(read_text("This"));
        let result = parser(&mut reader);
        assert_eq!(result, Ok("This"));
        assert_eq!(reader.byte_offset(), 4);

        let mut parser = not_found_restore(|reader| {
            read_text(" is ")(reader).unwrap();
            read_text("This")(reader)
        });
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
        assert_eq!(reader.byte_offset(), 4);
    }

    #[test]
    fn test_map_result() {
        let mut reader = Reader::new("This is a test");
        let mut parser = map_result(ascii_alpha1, |_, _| 32);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_and_then() {
        let mut reader = Reader::new("This is a test");
        let mut parser = and_then(ascii_alpha1, |_, _| Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        // Case when mapper fails.

        let mut reader = Reader::new("This is a test");
        let mut parser = and_then(ascii_alpha_quantified(1), |_, _| -> ParserResult<(), ()> {
            Err(ParserResultError::NotFound)
        });
        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_map_parser() {
        let mut reader = Reader::new("Test 123");
        let mut parser = map_parser(read_any_quantified(3), read_any);

        let result = parser(&mut reader);
        assert_eq!(result, Ok('T'));

        let result = parser(&mut reader);
        assert_eq!(result, Ok('t'));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_consumed() {
        let mut reader = Reader::new("Test 123");
        let mut parser = consumed(tuple((read_text("Te"), read_text("st"))));

        let result = parser(&mut reader);
        assert_eq!(result, Ok("Test"));
    }

    #[test]
    fn test_ignore_result() {
        let mut reader = Reader::new("Test 123");
        let mut parser = ignore_result(tuple((read_text("Te"), read_text("st"))));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_value() {
        let mut reader = Reader::new("This is a test");
        let mut parser = value(true);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(true));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_error() {
        let mut reader = Reader::new_with_error("This is a test");
        let mut parser = error("test");

        let result: ParserResult<(), &str> = parser(&mut reader);

        match result {
            Err(ParserResultError::Error((_, e))) => {
                assert_eq!(e, "test")
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_error_dyn() {
        let mut reader = Reader::new_with_error("This is a test");
        let mut parser = error_dyn(|r| format!("test at {}", r.save_cursor().char_offset()));

        let result: ParserResult<(), String> = parser(&mut reader);

        match result {
            Err(ParserResultError::Error((_, e))) => {
                assert_eq!(e.as_str(), "test at 0")
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_ensure() {
        let mut reader = Reader::new_with_error::<&str>("This is a test");
        let mut parser = ensure(read_text("Test"), |_| "test");

        let result = parser(&mut reader);

        match result {
            Err(ParserResultError::Error((_, e))) => {
                assert_eq!(e, "test")
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_recover() {
        let mut reader = Reader::new_with_error::<&str>("This is a test");
        let mut parser = recover(error("test1"), |_, _, _| Ok("recover"));

        let result = parser(&mut reader);
        assert_eq!(result, Ok("recover"));
    }
}
