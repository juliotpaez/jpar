use std::ops::RangeInclusive;

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
