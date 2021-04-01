use crate::result::{ParserResult, ParserResultError};
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
pub fn map_result<'a, C, R, Rf>(
    mut parser: impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
    mut mapper: impl FnMut(R) -> Rf,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<Rf> {
    move |reader| {
        let result = parser(reader)?;

        Ok(mapper(result))
    }
}

/// Always succeeds with given value without consuming any input.
pub fn value<'a, C, R: Clone>(value: R) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |_| Ok(value.clone())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::{ascii_alpha_quantified, read_text};

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
        let mut parser = map_result(ascii_alpha_quantified(1..), |_| 32);

        let result = parser(&mut reader);
        assert_eq!(result, Ok(32));

        let result = parser(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
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
