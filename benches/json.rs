#[macro_use]
extern crate criterion;

use std::collections::HashMap;

use criterion::Criterion;

use parfet::parsers::branch::alternative;
use parfet::parsers::characters::{
    read_any, read_any_quantified, read_char, read_none_of, read_text, ucd_whitespace0,
};
use parfet::parsers::combinator::verify;
use parfet::parsers::helpers::{and_then, map_result};
use parfet::parsers::numbers::read_float;
use parfet::parsers::sequence::{
    delimited, preceded, repeat_and_fold, repeat_separated, separated_tuple,
};
use parfet::parsers::verifiers::text_verifier;
use parfet::result::{ParserResult, ParserResultError};
use parfet::Reader;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn boolean(input: &mut Reader) -> ParserResult<bool> {
    alternative((
        map_result(read_text("false"), |_| false),
        map_result(read_text("true"), |_| true),
    ))(input)
}

fn u16_hex(input: &mut Reader) -> ParserResult<u16> {
    map_result(read_any_quantified(4), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}

fn unicode_escape(input: &mut Reader) -> ParserResult<char> {
    map_result(
        alternative((
            // Not a surrogate
            map_result(verify(u16_hex, |cp| !(0xD800..0xE000).contains(cp)), |cp| {
                cp as u32
            }),
            // See https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF for details
            map_result(
                verify(
                    separated_tuple((u16_hex, u16_hex), read_text("\\u")),
                    |(high, low)| (0xD800..0xDC00).contains(high) && (0xDC00..0xE000).contains(low),
                ),
                |(high, low)| {
                    let high_ten = (high as u32) - 0xD800;
                    let low_ten = (low as u32) - 0xDC00;
                    (high_ten << 10) + low_ten + 0x10000
                },
            ),
        )),
        |x| std::char::from_u32(x).unwrap(),
    )(input)
}

fn character(input: &mut Reader) -> ParserResult<char> {
    let c = read_none_of(text_verifier("\""))(input)?;
    if c == '\\' {
        alternative((
            and_then(read_any, |c| {
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

fn string(input: &mut Reader) -> ParserResult<String> {
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

fn ws<'a, P, C, R>(content: P) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R>
where
    P: FnMut(&mut Reader<'a, C>) -> ParserResult<R>,
{
    delimited(ucd_whitespace0, content, ucd_whitespace0)
}

fn array(input: &mut Reader) -> ParserResult<Vec<JsonValue>> {
    delimited(
        read_char('['),
        ws(repeat_separated(.., json_value, ws(read_char(',')))),
        read_char(']'),
    )(input)
}

fn object(input: &mut Reader) -> ParserResult<HashMap<String, JsonValue>> {
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
        |key_values| key_values.into_iter().collect(),
    )(input)
}

fn json_value(input: &mut Reader) -> ParserResult<JsonValue> {
    use JsonValue::*;

    alternative((
        map_result(read_text("null"), |_| Null),
        map_result(boolean, Bool),
        map_result(string, Str),
        map_result(read_float, |s| {
            let double = s.parse().unwrap();
            Num(double)
        }),
        map_result(array, Array),
        map_result(object, Object),
    ))(input)
}

fn json(input: &mut Reader) -> ParserResult<JsonValue> {
    ws(json_value)(input)
}

fn json_bench(c: &mut Criterion) {
    let data = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ,\"\\u2014\", \"\\uD83D\\uDE10\"] ,
  \"c\": { \"hello\" : \"world\"
  }
  }  ";

    c.bench_function("json", |b| {
        b.iter(|| {
            let mut reader = Reader::new(data);
            json(&mut reader).unwrap()
        });
    });
}

fn read_integer_bench(c: &mut Criterion) {
    c.bench_function("read_integer", |b| {
        b.iter(|| {
            let mut reader = Reader::new("-1.234E-12");
            read_float(&mut reader).unwrap()
        });
    });
}

fn read_float_bench(c: &mut Criterion) {
    c.bench_function("read_float", |b| {
        b.iter(|| {
            let mut reader = Reader::new("-1.234E-12");
            read_float(&mut reader).unwrap()
        });
    });
}

criterion_group!(benches, json_bench, read_integer_bench, read_float_bench,);
criterion_main!(benches);
