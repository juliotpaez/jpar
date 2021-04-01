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

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::read_text;

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
}
