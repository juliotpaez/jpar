#[macro_use]
extern crate criterion;

use std::collections::HashMap;

use criterion::Criterion;

use jpar::branch::alternative;
use jpar::characters::{
    read_any, read_any_quantified, read_char, read_none_of, read_text, ucd_whitespace0,
};
use jpar::combinator::verify;
use jpar::helpers::{and_then, map_result};
use jpar::numbers::{read_float, read_integer};
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
        map_result(read_text("false"), |_| false),
        map_result(read_text("true"), |_| true),
    ))(input)
}

fn u16_hex<Err>(input: &mut Reader<Err>) -> ParserResult<u16, Err> {
    map_result(read_any_quantified(4), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}

fn unicode_escape<Err>(input: &mut Reader<Err>) -> ParserResult<char, Err> {
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

fn character<Err>(input: &mut Reader<Err>) -> ParserResult<char, Err> {
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
        |key_values| key_values.into_iter().collect(),
    )(input)
}

fn json_value<Err: From<&'static str>>(input: &mut Reader<Err>) -> ParserResult<JsonValue, Err> {
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

fn json<Err: From<&'static str>>(input: &mut Reader<Err>) -> ParserResult<JsonValue, Err> {
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
            let mut reader = Reader::new_with_error::<&str>(data);
            json(&mut reader).unwrap()
        });
    });
}

fn read_integer_bench(c: &mut Criterion) {
    c.bench_function("read_integer", |b| {
        b.iter(|| {
            let mut reader = Reader::new("-1563718");
            read_integer(&mut reader).unwrap()
        });
    });
}

fn read_float_bench(c: &mut Criterion) {
    c.bench_function("read_float", |b| {
        b.iter(|| {
            let mut reader = Reader::new_with_error::<&str>("-1.234E-12");
            read_float(&mut reader).unwrap()
        });
    });
}

criterion_group!(benches, json_bench, read_integer_bench, read_float_bench,);
criterion_main!(benches);