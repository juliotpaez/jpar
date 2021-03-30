use std::ops::RangeInclusive;

pub use ranges::*;

use crate::result::{ParserResult, ParserResultError};
use crate::Reader;

mod ranges;

pub fn ascii_alpha0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, ASCII_ALPHA_CHARS, 0)
}

pub fn ascii_alpha1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, ASCII_ALPHA_CHARS, 1)
}

pub fn ascii_alphanumeric0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, ASCII_ALPHANUMERIC_CHARS, 0)
}

pub fn ascii_alphanumeric1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, ASCII_ALPHANUMERIC_CHARS, 1)
}

pub fn space0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, UCD_WHITESPACE_CHARS, 0)
}

pub fn space1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, UCD_WHITESPACE_CHARS, 1)
}

pub fn multiline_whitespace0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, UCD_MULTILINE_WHITESPACE_CHARS, 0)
}

pub fn multiline_whitespace1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, UCD_MULTILINE_WHITESPACE_CHARS, 1)
}

/// Reads a single "\r\n".
pub fn crlf<'a>(reader: &'a mut Reader<'a>) -> ParserResult<()> {
    if reader.read("\r\n") {
        Ok(())
    } else {
        ParserResult::Err(ParserResultError::NotFound)
    }
}

/// Reads a single "\n".
pub fn newline<'a>(reader: &'a mut Reader<'a>) -> ParserResult<()> {
    if reader.read("\n") {
        Ok(())
    } else {
        ParserResult::Err(ParserResultError::NotFound)
    }
}

/// Reads a single "\n" or "\r\n".
pub fn line_ending<'a>(reader: &'a mut Reader<'a>) -> ParserResult<()> {
    if reader.read("\r\n") || reader.read("\n") {
        Ok(())
    } else {
        ParserResult::Err(ParserResultError::NotFound)
    }
}

pub fn binary_digits0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, BINARY_DIGITS_CHARS, 0)
}

pub fn binary_digits1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, BINARY_DIGITS_CHARS, 1)
}

pub fn octal_digits0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, OCTAL_DIGITS_CHARS, 0)
}

pub fn octal_digits1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, OCTAL_DIGITS_CHARS, 1)
}

pub fn decimal_digits0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, DECIMAL_DIGITS_CHARS, 0)
}

pub fn decimal_digits1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, DECIMAL_DIGITS_CHARS, 1)
}

pub fn hexadecimal_digits0<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, HEXADECIMAL_DIGITS_CHARS, 0)
}

pub fn hexadecimal_digits1<'a>(reader: &'a mut Reader<'a>) -> ParserResult<&'a str> {
    read_one_of_min(reader, HEXADECIMAL_DIGITS_CHARS, 1)
}

pub fn not_consume<'a, F, R>(reader: &'a mut Reader<'a>, f: F) -> ParserResult<R>
where
    F: FnOnce(&mut Reader<'a>) -> ParserResult<R>,
{
    let init_cursor = reader.save_cursor();
    let result = f(reader);
    reader.restore(init_cursor);
    result
}

pub(crate) fn read_one_of_min<'a>(
    reader: &'a mut Reader<'a>,
    range: &[RangeInclusive<char>],
    min: usize,
) -> ParserResult<&'a str> {
    match reader.read_many_of(range) {
        Some(v) => {
            if v.len() >= min {
                Ok(v)
            } else {
                Err(ParserResultError::NotFound)
            }
        }
        None => {
            if min == 0 {
                Ok("")
            } else {
                Err(ParserResultError::NotFound)
            }
        }
    }
}
