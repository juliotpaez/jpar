use std::ops::RangeInclusive;

pub fn char_verifier(value: char) -> impl Fn(usize, char) -> bool {
    move |_, c| value == c
}

pub fn text_verifier(value: &str) -> impl Fn(usize, char) -> bool + '_ {
    move |_, c| value.contains(c)
}

pub fn range_verifier(value: RangeInclusive<char>) -> impl Fn(usize, char) -> bool {
    move |_, c| value.contains(&c)
}

/// # Safety
///
/// This implementation considers the interval is sorted to speed up,
/// therefore it can cause undefined behaviour otherwise.
pub fn interval_verifier(value: &'static [RangeInclusive<char>]) -> impl Fn(usize, char) -> bool {
    move |_, c| {
        for range in value.iter() {
            // Exit early to optimize searching.
            if &c < range.start() {
                break;
            }

            if range.contains(&c) {
                return true;
            }
        }

        false
    }
}
