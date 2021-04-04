use std::option::Option::Some;

use crate::result::{ParserResult, ParserResultError};
use crate::ParserInput;

/// Helper trait for the [alternative()] combinator.
pub trait Alternative<'a, C, R, Err> {
    /// Tests the specified parser if it exist.
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>>;
}

/// Returns the first alternative that matches in order.
pub fn alternative<'a, P, C, R, Err>(
    mut parsers: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>
where
    P: Alternative<'a, C, R, Err>,
{
    move |reader| {
        let mut i = 0;
        while let Some(value) = parsers.choice(i, reader) {
            match value {
                Ok(v) => return Ok(v),
                Err(ParserResultError::NotFound) => {}
                Err(e) => return Err(e),
            }

            i += 1;
        }

        Err(ParserResultError::NotFound)
    }
}

impl<'a, C, R, T1, Err> Alternative<'a, C, R, Err> for (T1,)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, Err> Alternative<'a, C, R, Err> for (T1, T2)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, Err> Alternative<'a, C, R, Err> for (T1, T2, T3)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, Err> Alternative<'a, C, R, Err> for (T1, T2, T3, T4)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, T5, Err> Alternative<'a, C, R, Err> for (T1, T2, T3, T4, T5)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T5: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            4 => Some(self.4(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, T5, T6, Err> Alternative<'a, C, R, Err> for (T1, T2, T3, T4, T5, T6)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T5: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T6: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            4 => Some(self.4(reader)),
            5 => Some(self.5(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, T5, T6, T7, Err> Alternative<'a, C, R, Err>
    for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T5: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T6: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T7: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            4 => Some(self.4(reader)),
            5 => Some(self.5(reader)),
            6 => Some(self.6(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, T5, T6, T7, T8, Err> Alternative<'a, C, R, Err>
    for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T5: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T6: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T7: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T8: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            4 => Some(self.4(reader)),
            5 => Some(self.5(reader)),
            6 => Some(self.6(reader)),
            7 => Some(self.7(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, T5, T6, T7, T8, T9, Err> Alternative<'a, C, R, Err>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T5: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T6: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T7: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T8: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T9: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            4 => Some(self.4(reader)),
            5 => Some(self.5(reader)),
            6 => Some(self.6(reader)),
            7 => Some(self.7(reader)),
            8 => Some(self.8(reader)),
            _ => None,
        }
    }
}

impl<'a, C, R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, Err> Alternative<'a, C, R, Err>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T1: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T2: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T3: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T4: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T5: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T6: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T7: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T8: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T9: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
    T10: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<R, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<R, Err>> {
        match index {
            0 => Some(self.0(reader)),
            1 => Some(self.1(reader)),
            2 => Some(self.2(reader)),
            3 => Some(self.3(reader)),
            4 => Some(self.4(reader)),
            5 => Some(self.5(reader)),
            6 => Some(self.6(reader)),
            7 => Some(self.7(reader)),
            8 => Some(self.8(reader)),
            9 => Some(self.9(reader)),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::read_text;

    use super::*;

    #[test]
    fn test_alternatives_t1() {
        let tuple_size = 1;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((value.remove(0),))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t2() {
        let tuple_size = 2;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((value.remove(0), value.remove(0)))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t3() {
        let tuple_size = 3;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result =
                alternative((value.remove(0), value.remove(0), value.remove(0)))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t4() {
        let tuple_size = 4;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t5() {
        let tuple_size = 5;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t6() {
        let tuple_size = 6;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t7() {
        let tuple_size = 7;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t8() {
        let tuple_size = 8;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t9() {
        let tuple_size = 9;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }

    #[test]
    fn test_alternatives_t10() {
        let tuple_size = 10;
        for i in 0..=tuple_size {
            let texts: Vec<_> = (0..=tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();
            value[i] = read_text("This");

            let mut reader = ParserInput::new("This is a test");
            let result = alternative((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok("This"), "Step: {}", i);
            }
        }
    }
}
