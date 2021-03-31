use std::ops::RangeInclusive;

use crate::parsers::combinator::map_result;
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
pub static UCD_SINGLE_LINE_WHITESPACE_CHARS: &[RangeInclusive<char>] = &[
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
pub static UCD_MULTILINE_WHITESPACE_CHARS: &[RangeInclusive<char>] = &[
    '\u{A}'..='\u{D}',
    '\u{85}'..='\u{85}',
    '\u{2028}'..='\u{2029}',
];

macro_rules! impl_range_parser {
    ($name:ident, $name_qtf:ident, $chars:expr, $comment:literal, $comment_qtf:literal) => {
        #[doc = $comment]
        pub fn $name<'a, C>(reader: &mut Reader<'a, C>) -> ParserResult<char> {
            read_any_of(crate::parsers::verifier::interval_verifier($chars))(reader)
        }

        #[doc = $comment_qtf]
        pub fn $name_qtf<'a, C>(
            quantifier: impl Into<Quantifier>,
        ) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<&'a str> {
            read_any_of_quantified(quantifier, crate::parsers::verifier::interval_verifier($chars))
        }
    };
}

impl_range_parser!(
    ascii_alpha,
    ascii_alpha_quantified,
    ASCII_ALPHA_CHARS,
    "Reads one ASCII alpha character",
    "Reads a quantified number of ASCII alpha characters"
);

impl_range_parser!(
    ascii_alphanumeric,
    ascii_alphanumeric_quantified,
    ASCII_ALPHANUMERIC_CHARS,
    "ASCII alphanumeric character",
    "Reads a quantified number of ASCII alphanumeric characters"
);

impl_range_parser!(
    binary_digit,
    binary_digit_quantified,
    BINARY_DIGITS_CHARS,
    "Reads one binary digit",
    "Reads a quantified number of binary digits"
);

impl_range_parser!(
    octal_digit,
    octal_digit_quantified,
    OCTAL_DIGITS_CHARS,
    "Reads one octal digit",
    "Reads a quantified number of octal digits"
);

impl_range_parser!(
    decimal_digit,
    decimal_digit_quantified,
    DECIMAL_DIGITS_CHARS,
    "Reads one decimal digit",
    "Reads a quantified number of decimal digits"
);

impl_range_parser!(
    hexadecimal_digit,
    hexadecimal_digit_quantified,
    HEXADECIMAL_DIGITS_CHARS,
    "Reads one hexadecimal digit",
    "Reads a quantified number of hexadecimal digits"
);

impl_range_parser!(
    ucd_whitespace,
    ucd_whitespace_quantified,
    UCD_WHITESPACE_CHARS,
    "Reads one Unicode whitespace",
    "Reads a quantified number of Unicode whitespaces"
);

impl_range_parser!(
    ucd_single_line_whitespace,
    ucd_single_line_whitespace_quantified,
    UCD_SINGLE_LINE_WHITESPACE_CHARS,
    "Reads one Unicode single-line whitespace",
    "Reads a quantified number of Unicode single-line whitespaces"
);

impl_range_parser!(
    ucd_multiline_whitespace,
    ucd_multiline_whitespace_quantified,
    UCD_MULTILINE_WHITESPACE_CHARS,
    "Reads one Unicode multiline whitespace",
    "Reads a quantified number of Unicode multiline whitespaces"
);

/// Reads a character.
pub fn read_char<'a, C>(character: char) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()> {
    move |reader: &mut Reader<C>| match reader.peek() {
        Some(v) => {
            if v == character {
                reader.read().unwrap();
                Ok(())
            } else {
                Err(ParserResultError::NotFound)
            }
        }
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads a character without taking into account the casing of the text.
pub fn read_char_no_case<'a, C>(
    character: char,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()> {
    move |reader: &mut Reader<C>| match reader.peek() {
        Some(v) => {
            if character
                .to_lowercase()
                .any(|lower_c| lower_c == v.to_lowercase().next().unwrap())
            {
                reader.read().unwrap();
                Ok(())
            } else {
                Err(ParserResultError::NotFound)
            }
        }
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads a text.
pub fn read_text<'a, C>(text: &'a str) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()> {
    move |reader: &mut Reader<C>| {
        let mut chars = text.chars();
        let result = reader.read_while(|i, c| i < text.len() && c == chars.next().unwrap());

        if result.len() == text.len() {
            Ok(())
        } else {
            Err(ParserResultError::NotFound)
        }
    }
}

/// Reads a text without taking into account the casing of the text.
pub fn read_text_no_case<'a, C>(
    text: &'a str,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<()> {
    move |reader: &mut Reader<C>| {
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
            Ok(())
        } else {
            Err(ParserResultError::NotFound)
        }
    }
}

/// Reads one character a quantified number of times.
pub fn read_any_quantified<'a, C>(
    quantifier: impl Into<Quantifier>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<&'a str> {
    let quantifier = quantifier.into();
    move |reader: &mut Reader<C>| match reader.read_quantified(quantifier) {
        Some(v) => Ok(v),
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads one character.
pub fn read_any<'a, C>() -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<char> {
    map_result(read_any_quantified(1), |v| v.chars().next().unwrap())
}

/// Reads one character that is inside `interval` a quantified number of times.
pub fn read_any_of_quantified<'a, C>(
    quantifier: impl Into<Quantifier>,
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<&'a str> {
    let quantifier = quantifier.into();
    move |reader: &mut Reader<C>| match reader
        .read_while_quantified(quantifier, |i, c| verifier(i, c))
    {
        Some(v) => Ok(v),
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads one character that is inside `interval`.
pub fn read_any_of<'a, C>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<char> {
    map_result(read_any_of_quantified(1, verifier), |v| {
        v.chars().next().unwrap()
    })
}

/// Reads one character that is not inside `interval` a quantified number of times.
pub fn read_none_of_quantified<'a, C>(
    quantifier: impl Into<Quantifier>,
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<&'a str> {
    let quantifier = quantifier.into();
    move |reader: &mut Reader<C>| match reader
        .read_while_quantified(quantifier, |i, c| !verifier(i, c))
    {
        Some(v) => Ok(v),
        None => Err(ParserResultError::NotFound),
    }
}

/// Reads one character that are not inside `interval`.
pub fn read_none_of<'a, 'b, C>(
    verifier: impl Fn(usize, char) -> bool,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<char> {
    map_result(read_none_of_quantified(1, verifier), |v| {
        v.chars().next().unwrap()
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::verifier::interval_verifier;

    use super::*;

    #[test]
    fn test_read_char() {
        let mut reader = Reader::new("This is a test");

        let result = read_char('T')(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_char('h')(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_char('T')(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_char_no_case() {
        let mut reader = Reader::new("This is a test");

        let result = read_char_no_case('t')(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_char_no_case('H')(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_char_no_case('T')(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_text() {
        let mut reader = Reader::new("This is a text");

        let result = read_text("This")(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_text(" is ")(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_text(" and")(&mut reader);
        assert_eq!(result, Err(ParserResultError::NotFound));
    }

    #[test]
    fn test_read_text_no_case() {
        let mut reader = Reader::new("This is a text");

        let result = read_text_no_case("tHIS")(&mut reader);
        assert_eq!(result, Ok(()));

        let result = read_text_no_case(" iS ")(&mut reader);
        assert_eq!(result, Ok(()));

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