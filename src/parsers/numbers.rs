use crate::parsers::branch::alternative;
use crate::parsers::characters::{decimal_digit1, read_any_of, read_char};
use crate::parsers::combinator::optional;
use crate::parsers::helpers::{consumed, ensure, ignore_result};
use crate::parsers::sequence::tuple;
use crate::parsers::verifiers::text_verifier;
use crate::{ParserResult, Reader};

/// Reads an integer number.
pub fn read_integer<'a, C, Err>(reader: &mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    consumed(tuple((
        optional(read_any_of(text_verifier("+-"))),
        decimal_digit1,
    )))(reader)
}

/// Reads a float number.
pub fn read_float<'a, C, Err: From<&'static str>>(
    reader: &mut Reader<'a, Err, C>,
) -> ParserResult<&'a str, Err> {
    consumed(tuple((
        optional(read_any_of(text_verifier("+-"))),
        alternative((
            ignore_result(tuple((
                decimal_digit1,
                optional(tuple((read_char('.'), optional(decimal_digit1)))),
            ))),
            ignore_result(tuple((read_char('.'), decimal_digit1))),
        )),
        optional(tuple((
            read_any_of(text_verifier("eE")),
            optional(read_any_of(text_verifier("+-"))),
            ensure(decimal_digit1, |_| {
                "A number is required after the exponent".into()
            }),
        ))),
    )))(reader)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::result::ParserResultError;

    use super::*;

    #[test]
    fn test_read_integer() {
        let mut reader = Reader::new("0123456789");
        let result = read_integer(&mut reader);
        assert_eq!(result, Ok("0123456789"));

        let mut reader = Reader::new("+0123456789");
        let result = read_integer(&mut reader);
        assert_eq!(result, Ok("+0123456789"));

        let mut reader = Reader::new("-0123456789");
        let result = read_integer(&mut reader);
        assert_eq!(result, Ok("-0123456789"));

        let mut reader = Reader::new("a23");
        let result = read_integer(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_float() {
        let mut reader = Reader::new_with_error::<&str>("0123456789");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("0123456789"));

        let mut reader = Reader::new_with_error::<&str>("+0123456789.0123456789");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("+0123456789.0123456789"));

        let mut reader = Reader::new_with_error::<&str>("-.0123456789");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("-.0123456789"));

        let mut reader = Reader::new_with_error::<&str>("-0123456789.");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("-0123456789."));

        let mut reader = Reader::new_with_error::<&str>("0123456789e0123456789");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("0123456789e0123456789"));

        let mut reader = Reader::new_with_error::<&str>("0123456789E+0123456789");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("0123456789E+0123456789"));

        let mut reader = Reader::new_with_error::<&str>("0123456789E-0123456789");
        let result = read_float(&mut reader);
        assert_eq!(result, Ok("0123456789E-0123456789"));

        let mut reader = Reader::new_with_error::<&str>("a23");
        let result = read_float(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }
}
