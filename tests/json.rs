use std::collections::HashMap;

use jpar::branch::alternative;
use jpar::characters::{
    read_any, read_any_quantified, read_char, read_none_of, read_text, ucd_whitespace0,
};
use jpar::combinator::verify;
use jpar::helpers::{and_then, map_result};
use jpar::numbers::read_float;
use jpar::sequence::{delimited, preceded, repeat_and_fold, repeat_separated, separated_tuple};
use jpar::verifiers::text_verifier;
use jpar::Reader;
use jpar::{ParserResult, ParserResultError};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn boolean<Err>(input: &mut Reader<Err>) -> ParserResult<bool, Err> {
    alternative((
        map_result(read_text("false"), |_, _| false),
        map_result(read_text("true"), |_, _| true),
    ))(input)
}

fn u16_hex<Err>(input: &mut Reader<Err>) -> ParserResult<u16, Err> {
    map_result(read_any_quantified(4), |_, s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}

fn unicode_escape<Err>(input: &mut Reader<Err>) -> ParserResult<char, Err> {
    map_result(
        alternative((
            // Not a surrogate
            map_result(
                verify(u16_hex, |_, cp| !(0xD800..0xE000).contains(cp)),
                |_, cp| cp as u32,
            ),
            // See https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF for details
            map_result(
                verify(
                    separated_tuple((u16_hex, u16_hex), read_text("\\u")),
                    |_, (high, low)| {
                        (0xD800..0xDC00).contains(high) && (0xDC00..0xE000).contains(low)
                    },
                ),
                |_, (high, low)| {
                    let high_ten = (high as u32) - 0xD800;
                    let low_ten = (low as u32) - 0xDC00;
                    (high_ten << 10) + low_ten + 0x10000
                },
            ),
        )),
        |_, x| std::char::from_u32(x).unwrap(),
    )(input)
}

fn character<Err>(input: &mut Reader<Err>) -> ParserResult<char, Err> {
    let c = read_none_of(text_verifier("\""))(input)?;
    if c == '\\' {
        alternative((
            and_then(read_any, |_, c| {
                Ok(match c {
                    '"' | '\\' | '/' => c,
                    'b' => '\x08',
                    'f' => '\x0C',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => return Err(ParserResultError::NotFound),
                })
            }),
            preceded(read_char('u'), unicode_escape),
        ))(input)
    } else {
        Ok(c)
    }
}

fn string<Err>(input: &mut Reader<Err>) -> ParserResult<String, Err> {
    delimited(
        read_char('"'),
        repeat_and_fold(
            ..,
            String::new(),
            |mut string, c| {
                string.push(c);
                string
            },
            character,
        ),
        read_char('"'),
    )(input)
}

fn ws<'a, P, C, R, Err>(content: P) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>
where
    P: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R, Err>,
{
    delimited(ucd_whitespace0, content, ucd_whitespace0)
}

fn array<Err: From<&'static str>>(input: &mut Reader<Err>) -> ParserResult<Vec<JsonValue>, Err> {
    delimited(
        read_char('['),
        ws(repeat_separated(.., json_value, ws(read_char(',')))),
        read_char(']'),
    )(input)
}

fn object<Err: From<&'static str>>(
    input: &mut Reader<Err>,
) -> ParserResult<HashMap<String, JsonValue>, Err> {
    map_result(
        delimited(
            read_char('{'),
            ws(repeat_separated(
                ..,
                separated_tuple((string, json_value), ws(read_char(':'))),
                ws(read_char(',')),
            )),
            read_char('}'),
        ),
        |_, key_values| key_values.into_iter().collect(),
    )(input)
}

fn json_value<Err: From<&'static str>>(input: &mut Reader<Err>) -> ParserResult<JsonValue, Err> {
    use JsonValue::*;

    alternative((
        map_result(read_text("null"), |_, _| Null),
        map_result(boolean, |_, v| Bool(v)),
        map_result(string, |_, v| Str(v)),
        map_result(read_float, |_, s| {
            let double = s.parse().unwrap();
            Num(double)
        }),
        map_result(array, |_, v| Array(v)),
        map_result(object, |_, v| Object(v)),
    ))(input)
}

fn json<Err: From<&'static str>>(input: &mut Reader<Err>) -> ParserResult<JsonValue, Err> {
    ws(json_value)(input)
}

#[test]
fn test() {
    let data = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ,\"\\u2014\", \"\\uD83D\\uDE10\"] ,
  \"c\": { \"hello\" : \"world\"
  }
  }  ";

    let mut reader = Reader::new_with_error::<&str>(data);
    json(&mut reader).unwrap();
}
