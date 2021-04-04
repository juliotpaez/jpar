use std::option::Option::Some;

use crate::result::{ParserResult, ParserResultError};
use crate::Reader;

/// Helper trait for the [alternative_ignore()] combinator.
pub trait AlternativeIgnore<'a, C, Err> {
    /// Tests the specified parser if it exist.
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>>;
}

/// Returns the first alternative that matches in order.
pub fn alternative_ignore<'a, P, C, Err>(
    mut parsers: P,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<(), Err>
where
    P: AlternativeIgnore<'a, C, Err>,
{
    move |reader| {
        let mut i = 0;
        while let Some(value) = parsers.choice(i, reader) {
            match value {
                Ok(_) => return Ok(()),
                Err(ParserResultError::NotFound) => {}
                Err(e) => return Err(e),
            }

            i += 1;
        }

        Err(ParserResultError::NotFound)
    }
}

impl<'a, C, T1, R1, Err> AlternativeIgnore<'a, C, Err> for (T1,)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, R1, R2, Err> AlternativeIgnore<'a, C, Err> for (T1, T2)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, R1, R2, R3, Err> AlternativeIgnore<'a, C, Err> for (T1, T2, T3)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, T4, R1, R2, R3, R4, Err> AlternativeIgnore<'a, C, Err> for (T1, T2, T3, T4)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, T4, T5, R1, R2, R3, R4, R5, Err> AlternativeIgnore<'a, C, Err>
    for (T1, T2, T3, T4, T5)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            4 => Some(self.4(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, T4, T5, T6, R1, R2, R3, R4, R5, R6, Err> AlternativeIgnore<'a, C, Err>
    for (T1, T2, T3, T4, T5, T6)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
    T6: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R6, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            4 => Some(self.4(reader).map(|_| ())),
            5 => Some(self.5(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, T4, T5, T6, T7, R1, R2, R3, R4, R5, R6, R7, Err>
    AlternativeIgnore<'a, C, Err> for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
    T6: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R6, Err>,
    T7: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R7, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            4 => Some(self.4(reader).map(|_| ())),
            5 => Some(self.5(reader).map(|_| ())),
            6 => Some(self.6(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, T4, T5, T6, T7, T8, R1, R2, R3, R4, R5, R6, R7, R8, Err>
    AlternativeIgnore<'a, C, Err> for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
    T6: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R6, Err>,
    T7: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R7, Err>,
    T8: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R8, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            4 => Some(self.4(reader).map(|_| ())),
            5 => Some(self.5(reader).map(|_| ())),
            6 => Some(self.6(reader).map(|_| ())),
            7 => Some(self.7(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<'a, C, T1, T2, T3, T4, T5, T6, T7, T8, T9, R1, R2, R3, R4, R5, R6, R7, R8, R9, Err>
    AlternativeIgnore<'a, C, Err> for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
    T6: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R6, Err>,
    T7: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R7, Err>,
    T8: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R8, Err>,
    T9: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R9, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            4 => Some(self.4(reader).map(|_| ())),
            5 => Some(self.5(reader).map(|_| ())),
            6 => Some(self.6(reader).map(|_| ())),
            7 => Some(self.7(reader).map(|_| ())),
            8 => Some(self.8(reader).map(|_| ())),
            _ => None,
        }
    }
}

impl<
        'a,
        C,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        R8,
        R9,
        R10,
        Err,
    > AlternativeIgnore<'a, C, Err> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
    T6: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R6, Err>,
    T7: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R7, Err>,
    T8: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R8, Err>,
    T9: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R9, Err>,
    T10: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R10, Err>,
{
    fn choice(
        &mut self,
        index: usize,
        reader: &mut Reader<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>> {
        match index {
            0 => Some(self.0(reader).map(|_| ())),
            1 => Some(self.1(reader).map(|_| ())),
            2 => Some(self.2(reader).map(|_| ())),
            3 => Some(self.3(reader).map(|_| ())),
            4 => Some(self.4(reader).map(|_| ())),
            5 => Some(self.5(reader).map(|_| ())),
            6 => Some(self.6(reader).map(|_| ())),
            7 => Some(self.7(reader).map(|_| ())),
            8 => Some(self.8(reader).map(|_| ())),
            9 => Some(self.9(reader).map(|_| ())),
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((value.remove(0),))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((value.remove(0), value.remove(0)))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((value.remove(0), value.remove(0), value.remove(0)))(
                &mut reader,
            );

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ))(&mut reader);

            if tuple_size == i {
                assert_eq!(result, Err(ParserResultError::NotFound));
            } else {
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
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
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
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
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
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
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
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
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
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

            let mut reader = Reader::new("This is a test");
            let result = alternative_ignore((
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
                assert_eq!(result, Ok(()), "Step: {}", i);
                assert_eq!(reader.byte_offset(), 4, "Step: {} - byte_offset", i);
            }
        }
    }
}
