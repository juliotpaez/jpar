use crate::result::{ParserError, ParserResult, ParserResultError};
use crate::Reader;

/// Restores the reader when a not found error is returned.
pub fn not_found_restore<'a, C, R>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
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
pub fn map_result<'a, P, M, C, R, Rf>(
    mut parser: P,
    mut mapper: M,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Rf>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    M: FnMut(R) -> Rf,
{
    move |reader| {
        let result = parser(reader)?;

        Ok(mapper(result))
    }
}

/// Maps the result of a parser into a new `ParserResult`.
pub fn and_then<'a, P, M, C, R, Rf>(
    mut parser: P,
    mut mapper: M,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Rf>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    M: FnMut(R) -> ParserResult<Rf>,
{
    not_found_restore(move |reader| {
        let result = parser(reader)?;

        mapper(result)
    })
}

/// Applies a parser over the result of another one.
pub fn map_parser<'a, O, P, C: Clone, R>(
    mut origin: O,
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    O: FnMut(&mut Reader<'a, C>) -> ParserResult<&'a str>,
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
{
    not_found_restore(move |reader| {
        let result = origin(reader)?;
        let mut new_reader = Reader::new_with_context(result, reader.context().clone());

        parser(&mut new_reader)
    })
}

/// Applies a parser discarding its result and return the consumed content as result.
pub fn consumed<'a, P, C, R>(
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<&'a str>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
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
pub fn ignore_result<'a, P, C, R>(
    mut parser: P,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
{
    not_found_restore(move |reader| {
        let _ = parser(reader)?;
        Ok(())
    })
}

/// Always succeeds with given value without consuming any input.
pub fn value<'a, C, R: Clone>(value: R) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |_| Ok(value.clone())
}

/// Ensures that `parser` always success or returns an error.
pub fn ensure<'a, P, C, R, Efn>(
    mut parser: P,
    mut error_fn: Efn,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    Efn: FnMut(&mut Reader<'a, C>) -> ParserError,
{
    move |reader| match parser(reader) {
        Ok(v) => Ok(v),
        Err(ParserResultError::NotFound) => Err(ParserResultError::Error(error_fn(reader))),
        Err(e) => Err(e),
    }
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
        let mut parser = map_result(ascii_alpha1, |_| 32);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_and_then() {
        let mut reader = Reader::new("This is a test");
        let mut parser = and_then(ascii_alpha1, |_| Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        // Case when mapper fails.

        let mut reader = Reader::new("This is a test");
        let mut parser = and_then(ascii_alpha_quantified(1), |_| -> ParserResult<()> {
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
}
