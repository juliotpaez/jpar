use parfet::parsers::branch::alternative;
use parfet::parsers::characters::{
    decimal_digit1, read_any_of, read_char, ucd_single_line_whitespace0,
};
use parfet::parsers::helpers::map_result;
use parfet::parsers::sequence::{delimited, repeat_and_fold, tuple};
use parfet::parsers::verifiers::text_verifier;
use parfet::result::ParserResult;
use parfet::Reader;

// Parser definition

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If there are no digits, we look for a parenthesized expression.
fn factor(input: &mut Reader) -> ParserResult<i64> {
    delimited(
        ucd_single_line_whitespace0,
        alternative((
            map_result(decimal_digit1, |digits| {
                unsafe { std::str::from_utf8_unchecked(digits.as_bytes()) }
                    .parse()
                    .unwrap()
            }),
            delimited(read_char('('), expr, read_char(')')),
        )),
        ucd_single_line_whitespace0,
    )(input)
}

// We read an initial factor and for each time we find
// a * or / operator followed by another factor, we do
// the math by folding everything
fn term(input: &mut Reader) -> ParserResult<i64> {
    let init = factor(input)?;
    repeat_and_fold(
        ..,
        init,
        |acc, (op, val)| {
            if op == '*' {
                acc * val
            } else {
                acc / val
            }
        },
        tuple((read_any_of(text_verifier("*/")), factor)),
    )(input)
}

fn expr(input: &mut Reader) -> ParserResult<i64> {
    let init = term(input)?;
    repeat_and_fold(
        ..,
        init,
        |acc, (op, val)| {
            if op == '+' {
                acc + val
            } else {
                acc - val
            }
        },
        tuple((read_any_of(text_verifier("+-")), term)),
    )(input)
}

#[test]
fn test() {
    let data = "  2*2 / ( 5 - 1) + 3 / 4 * (2 - 7 + 567 *12 /2) + 3*(1+2*( 45 /2));";
    let mut reader = Reader::new(data);

    let result = expr(&mut reader);
    assert_eq!(
        result,
        Ok(2 * 2 / (5 - 1) + 3 / 4 * (2 - 7 + 567 * 12 / 2) + 3 * (1 + 2 * (45 / 2)))
    );

    assert_eq!(reader.remaining_content(), ";");
}
