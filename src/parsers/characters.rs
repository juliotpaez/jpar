use std::ops::RangeInclusive;

use crate::parsers::helpers::map_result;
use crate::parsers::Quantifier;
use crate::result::{ParserResult, ParserResultError};
use crate::Reader;

pub static ASCII_ALPHA_CHARS: &[RangeInclusive<char>] = &['A'..='Z', 'a'..='z'];
pub static ASCII_ALPHANUMERIC_CHARS: &[RangeInclusive<char>] = &['0'..='9', 'A'..='Z', 'a'..='z'];

pub static BINARY_DIGITS_CHARS: &[RangeInclusive<char>] = &['0'..='1'];
pub static OCTAL_DIGITS_CHARS: &[RangeInclusive<char>] = &['0'..='7'];
pub static DECIMAL_DIGITS_CHARS: &[RangeInclusive<char>] = &['0'..='9'];
pub static HEXADECIMAL_DIGITS_CHARS: &[RangeInclusive<char>] = &['0'..='9', 'A'..='F', 'a'..='f'];

/// Follow UCD specification: https://www.unicode.org/Public/13.0.0/ucd/PropList.txt
pub static UCD_WHITESPACE_CHARS: &[RangeInclusive<char>] = &[
    '\u{9}'..='\u{D}',
    '\u{20}'..='\u{20}',
    '\u{85}'..='\u{85}',
    '\u{A0}'..='\u{A0}',
    '\u{1680}'..='\u{1680}',
    '\u{2000}'..='\u{200A}',
    '\u{2028}'..='\u{2029}',
    '\u{202F}'..='\u{202F}',
    '\u{205F}'..='\u{205F}',
    '\u{3000}'..='\u{3000}',
];

/// Follow UCD specification: https://www.unicode.org/Public/13.0.0/ucd/PropList.txt
pub static UCD_INLINE_WHITESPACE_CHARS: &[RangeInclusive<char>] = &[
    '\u{9}'..='\u{9}',
    '\u{20}'..='\u{20}',
    '\u{A0}'..='\u{A0}',
    '\u{1680}'..='\u{1680}',
    '\u{2000}'..='\u{200A}',
    '\u{202F}'..='\u{202F}',
    '\u{205F}'..='\u{205F}',
    '\u{3000}'..='\u{3000}',
];

/// Follow UCD specification: https://www.unicode.org/Public/13.0.0/ucd/PropList.txt
pub static UCD_LINE_BREAK_WHITESPACE_CHARS: &[RangeInclusive<char>] = &[
    '\u{A}'..='\u{D}',
    '\u{85}'..='\u{85}',
    '\u{2028}'..='\u{2029}',
];

macro_rules! impl_range_parser {
    ($chars:expr, $name:ident, $comment:literal, $name0:ident, $comment0:literal, $name1:ident, $comment1:literal, $name_qtf:ident, $comment_qtf:literal $(,)?) => {
        #[doc = $comment]
        pub fn $name<'a, C, Err>(reader: &mut Reader<'a, Err, C>) -> ParserResult<char, Err> {
            read_any_of(crate::parsers::verifiers::interval_verifier($chars))(reader)
        }

        #[doc = $comment0]
        pub fn $name0<'a, C, Err>(reader: &mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
            read_any_of_quantified(0.., crate::parsers::verifiers::interval_verifier($chars))(reader)
        }

        #[doc = $comment1]
        pub fn $name1<'a, C, Err>(reader: &mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
            read_any_of_quantified(1.., crate::parsers::verifiers::interval_verifier($chars))(reader)
        }

        #[doc = $comment_qtf]
        pub fn $name_qtf<'a, C, Err>(
            quantifier: impl Into<Quantifier>,
        ) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
            read_any_of_quantified(quantifier, crate::parsers::verifiers::interval_verifier($chars))
        }
    };
}

impl_range_parser!(
    ASCII_ALPHA_CHARS,
    ascii_alpha,
    "Reads one ASCII alpha character",
    ascii_alpha0,
    "Reads zero or more ASCII alpha characters",
    ascii_alpha1,
    "Reads one or more ASCII alpha characters",
    ascii_alpha_quantified,
    "Reads a quantified number of ASCII alpha characters",
);

impl_range_parser!(
    ASCII_ALPHANUMERIC_CHARS,
    ascii_alphanumeric,
    "Reads one ASCII alphanumeric character",
    ascii_alphanumeric0,
    "ASCII zero or more alphanumeric characters",
    ascii_alphanumeric1,
    "ASCII one or more alphanumeric characters",
    ascii_alphanumeric_quantified,
    "Reads a quantified number of ASCII alphanumeric characters"
);

impl_range_parser!(
    BINARY_DIGITS_CHARS,
    binary_digit,
    "Reads one binary digit",
    binary_digit0,
    "Reads zero or more binary digits",
    binary_digit1,
    "Reads one or more binary digits",
    binary_digit_quantified,
    "Reads a quantified number of binary digits"
);

impl_range_parser!(
    OCTAL_DIGITS_CHARS,
    octal_digit,
    "Reads one octal digit",
    octal_digit0,
    "Reads zero or more octal digits",
    octal_digit1,
    "Reads one or more octal digits",
    octal_digit_quantified,
    "Reads a quantified number of octal digits"
);

impl_range_parser!(
    DECIMAL_DIGITS_CHARS,
    decimal_digit,
    "Reads one decimal digit",
    decimal_digit0,
    "Reads zero or more decimal digits",
    decimal_digit1,
    "Reads one or more decimal digits",
    decimal_digit_quantified,
    "Reads a quantified number of decimal digits"
);

impl_range_parser!(
    HEXADECIMAL_DIGITS_CHARS,
    hexadecimal_digit,
    "Reads one hexadecimal digit",
    hexadecimal_digit0,
    "Reads zero or more hexadecimal digits",
    hexadecimal_digit1,
    "Reads one or more hexadecimal digits",
    hexadecimal_digit_quantified,
    "Reads a quantified number of hexadecimal digits"
);

impl_range_parser!(
    UCD_WHITESPACE_CHARS,
    ucd_whitespace,
    "Reads one Unicode whitespace",
    ucd_whitespace0,
    "Reads zero or more Unicode whitespaces",
    ucd_whitespace1,
    "Reads one or more Unicode whitespaces",
    ucd_whitespace_quantified,
    "Reads a quantified number of Unicode whitespaces"
);

impl_range_parser!(
    UCD_INLINE_WHITESPACE_CHARS,
    ucd_inline_whitespace,
    "Reads one Unicode inline whitespace",
    ucd_inline_whitespace0,
    "Reads zero or more Unicode inline whitespaces",
    ucd_inline_whitespace1,
    "Reads one or more Unicode inline whitespaces",
    ucd_inline_whitespace_quantified,
    "Reads a quantified number of Unicode inline whitespaces"
);

impl_range_parser!(
    UCD_LINE_BREAK_WHITESPACE_CHARS,
    ucd_line_break_whitespace,
    "Reads one Unicode line break whitespace",
    ucd_line_break_whitespace0,
    "Reads zero or more Unicode line break whitespaces",
    ucd_line_break_whitespace1,
    "Reads one or more Unicode line break whitespaces",
    ucd_line_break_whitespace_quantified,
    "Reads a quantified number of Unicode line break whitespaces"
);

/// Reads a character.
pub fn read_char<'a, C, Err>(
    character: char,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<char, Err> {
    move |reader| match reader.peek() {
        Some(v) => {
            if v == character {
                Ok(reader.read().unwrap())
            } else {
                Err(ParserResultError::NotFound)
            }
        }
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads a character without taking into account the casing of the text.
pub fn read_char_no_case<'a, C, Err>(
    character: char,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<char, Err> {
    move |reader| match reader.peek() {
        Some(v) => {
            if character
                .to_lowercase()
                .any(|lower_c| lower_c == v.to_lowercase().next().unwrap())
            {
                Ok(reader.read().unwrap())
            } else {
                Err(ParserResultError::NotFound)
            }
        }
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads a text.
pub fn read_text<'a, C, Err>(
    text: &'a str,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    move |reader| {
        if reader.read_text(text) {
            Ok(text)
        } else {
            Err(ParserResultError::NotFound)
        }
    }
}

/// Reads a text without taking into account the casing of the text.
pub fn read_text_no_case<'a, C, Err>(
    text: &'a str,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    move |reader| {
        let mut chars = text.chars();
        let result = reader.read_while(|i, c| {
            i < text.len()
                && chars
                    .next()
                    .unwrap()
                    .to_lowercase()
                    .any(|c_lower| c_lower == c.to_lowercase().next().unwrap())
        });

        if result.len() == text.len() {
            Ok(result)
        } else {
            Err(ParserResultError::NotFound)
        }
    }
}

/// Reads one character a quantified number of times.
pub fn read_any_quantified<'a, C, Err>(
    quantifier: impl Into<Quantifier>,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    let quantifier = quantifier.into();
    move |reader| match reader.read_quantified(quantifier) {
        Some(v) => Ok(v),
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads one character.
pub fn read_any<C, Err>(reader: &mut Reader<Err, C>) -> ParserResult<char, Err> {
    map_result(read_any_quantified(1), |_, v| v.chars().next().unwrap())(reader)
}

/// Reads zero or more character.
pub fn read_any0<'a, C, Err>(reader: &mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    read_any_quantified(0..)(reader)
}

/// Reads one or more characters.
pub fn read_any1<'a, C, Err>(reader: &mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    read_any_quantified(1..)(reader)
}

/// Reads one character that is inside `interval` a quantified number of times.
pub fn read_any_of_quantified<'a, C, Err>(
    quantifier: impl Into<Quantifier>,
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    let quantifier = quantifier.into();
    move |reader| match reader.read_while_quantified(quantifier, |i, c| verifier(i, c)) {
        Some(v) => Ok(v),
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads one character that is inside `interval`.
pub fn read_any_of<'a, C, Err>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<char, Err> {
    map_result(read_any_of_quantified(1, verifier), |_, v| {
        v.chars().next().unwrap()
    })
}

/// Reads zero or more characters that are inside `interval`.
pub fn read_any_of0<'a, C, Err>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    read_any_of_quantified(0.., verifier)
}

/// Reads one or more characters that are inside `interval`.
pub fn read_any_of1<'a, C, Err>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    read_any_of_quantified(1.., verifier)
}

/// Reads one character that is not inside `interval` a quantified number of times.
pub fn read_none_of_quantified<'a, C, Err>(
    quantifier: impl Into<Quantifier>,
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    let quantifier = quantifier.into();
    move |reader| match reader.read_while_quantified(quantifier, |i, c| !verifier(i, c)) {
        Some(v) => Ok(v),
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads one character that is not inside `interval`.
pub fn read_none_of<'a, C, Err>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<char, Err> {
    map_result(read_none_of_quantified(1, verifier), |_, v| {
        v.chars().next().unwrap()
    })
}

/// Reads zero or more characters that are not inside `interval`.
pub fn read_none_of0<'a, C, Err>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    read_none_of_quantified(0.., verifier)
}

/// Reads one or more characters that are not inside `interval`.
pub fn read_none_of1<'a, C, Err>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<&'a str, Err> {
    read_none_of_quantified(1.., verifier)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::verifiers::interval_verifier;

    use super::*;

    #[test]
    fn test_read_char() {
        let mut reader = Reader::new("This is a test");

        let result = read_char('T')(&mut reader);
        assert_eq!(result, Ok('T'));

        let result = read_char('h')(&mut reader);
        assert_eq!(result, Ok('h'));

        let result = read_char('T')(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_char_no_case() {
        let mut reader = Reader::new("This is a test");

        let result = read_char_no_case('t')(&mut reader);
        assert_eq!(result, Ok('T'));

        let result = read_char_no_case('H')(&mut reader);
        assert_eq!(result, Ok('h'));

        let result = read_char_no_case('T')(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_text() {
        let mut reader = Reader::new("This is a text");

        let result = read_text("This")(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = read_text(" is ")(&mut reader);
        assert_eq!(result, Ok(" is "));

        let result = read_text(" and")(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_text_no_case() {
        let mut reader = Reader::new("This is a text");

        let result = read_text_no_case("tHIS")(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = read_text_no_case(" iS ")(&mut reader);
        assert_eq!(result, Ok(" is "));

        let result = read_text_no_case(" And")(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_any_quantified() {
        let mut reader = Reader::new("This is a test");

        let result = read_any_quantified(4)(&mut reader);
        assert_eq!(result, Ok("This"));

        let result = read_any_quantified(4)(&mut reader);
        assert_eq!(result, Ok(" is "));

        let result = read_any_quantified(50)(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_any_of_quantified() {
        let mut reader = Reader::new("This is a test");

        let result = read_any_of_quantified(1.., interval_verifier(&['A'..='Z']))(&mut reader);
        assert_eq!(result, Ok("T"));

        let result = read_any_of_quantified(1.., interval_verifier(&['A'..='Z']))(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));

        let result = read_any_of_quantified(1.., interval_verifier(&['a'..='z']))(&mut reader);
        assert_eq!(result, Ok("his"));
    }

    #[test]
    fn test_read_any1() {
        let mut reader = Reader::new("This is a test");

        let result = read_none_of_quantified(1.., interval_verifier(&['a'..='z']))(&mut reader);
        assert_eq!(result, Ok("T"));

        let result = read_none_of_quantified(1.., interval_verifier(&[' '..=' ']))(&mut reader);
        assert_eq!(result, Ok("his"));

        let result = read_none_of_quantified(1.., interval_verifier(&[' '..=' ']))(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }
}
