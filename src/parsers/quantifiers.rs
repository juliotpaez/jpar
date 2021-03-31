use std::ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

/// A quantifier that specify a range of repetitions with both ends included.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Quantifier {
    min: usize,
    max: Option<usize>,
}

impl Quantifier {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn no_repeat() -> Quantifier {
        Quantifier {
            min: 0,
            max: Some(0),
        }
    }

    pub fn zero_or_more() -> Quantifier {
        Quantifier { min: 0, max: None }
    }

    pub fn one_or_more() -> Quantifier {
        Quantifier { min: 1, max: None }
    }

    pub fn exact(value: usize) -> Quantifier {
        Quantifier {
            min: value,
            max: Some(value),
        }
    }

    pub fn at_least(min: usize) -> Quantifier {
        Quantifier { min, max: None }
    }

    pub fn at_most(max: usize) -> Quantifier {
        Quantifier {
            min: 0,
            max: Some(max),
        }
    }

    pub fn min_max(min: usize, max: usize) -> Quantifier {
        Quantifier {
            min,
            max: Some(max),
        }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }

    pub fn is_unbounded(&self) -> bool {
        self.max.is_none()
    }

    // METHODS ----------------------------------------------------------------

    pub fn contains(&self, iteration: usize) -> bool {
        if let Some(max) = self.max {
            self.min <= iteration && iteration <= max
        } else {
            self.min <= iteration
        }
    }

    /// Success when `iteration >= max`.
    pub fn is_finished(&self, iteration: usize) -> bool {
        if let Some(max) = self.max {
            iteration >= max
        } else {
            false
        }
    }
}

impl From<u8> for Quantifier {
    fn from(value: u8) -> Self {
        Quantifier::exact(value as usize)
    }
}

impl From<u16> for Quantifier {
    fn from(value: u16) -> Self {
        Quantifier::exact(value as usize)
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<u32> for Quantifier {
    fn from(value: u32) -> Self {
        Quantifier::exact(value as usize)
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for Quantifier {
    fn from(value: u64) -> Self {
        Quantifier::exact(value as usize)
    }
}

impl From<usize> for Quantifier {
    fn from(value: usize) -> Self {
        Quantifier::exact(value)
    }
}

/// Implemented for simplicity.
///
/// # Safety
///
/// This method will panic if number is negative.
impl From<i8> for Quantifier {
    fn from(value: i8) -> Self {
        assert!(value >= 0, "Cannot make a quantifier of a negative number");
        Quantifier::exact(value as usize)
    }
}

/// Implemented for simplicity.
///
/// # Safety
///
/// This method will panic if number is negative.
impl From<i16> for Quantifier {
    fn from(value: i16) -> Self {
        assert!(value >= 0, "Cannot make a quantifier of a negative number");
        Quantifier::exact(value as usize)
    }
}

/// Implemented for simplicity.
///
/// # Safety
///
/// This method will panic if number is negative.
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<i32> for Quantifier {
    fn from(value: i32) -> Self {
        assert!(value >= 0, "Cannot make a quantifier of a negative number");
        Quantifier::exact(value as usize)
    }
}

/// Implemented for simplicity.
///
/// # Safety
///
/// This method will panic if number is negative.
#[cfg(target_pointer_width = "64")]
impl From<i64> for Quantifier {
    fn from(value: i64) -> Self {
        assert!(value >= 0, "Cannot make a quantifier of a negative number");
        Quantifier::exact(value as usize)
    }
}

impl From<isize> for Quantifier {
    fn from(value: isize) -> Self {
        Quantifier::exact(value as usize)
    }
}

impl From<RangeInclusive<usize>> for Quantifier {
    fn from(value: RangeInclusive<usize>) -> Self {
        Quantifier::min_max(*value.start(), *value.end())
    }
}

impl From<RangeToInclusive<usize>> for Quantifier {
    fn from(value: RangeToInclusive<usize>) -> Self {
        Quantifier::at_most(value.end)
    }
}

impl From<RangeFrom<usize>> for Quantifier {
    fn from(value: RangeFrom<usize>) -> Self {
        Quantifier::at_least(value.start)
    }
}

impl From<RangeFull> for Quantifier {
    fn from(_: RangeFull) -> Self {
        Quantifier::zero_or_more()
    }
}
