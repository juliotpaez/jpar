use std::option::Option::Some;

use crate::result::{ParserResult, ParserResultError};
use crate::ParserInput;

/// Helper trait for the [alternative_ignore()] combinator.
pub trait AlternativeIgnore<'a, C, Err> {
    /// Tests the specified parser if it exist.
    fn choice(
        &mut self,
        index: usize,
        reader: &mut ParserInput<'a, Err, C>,
    ) -> Option<ParserResult<(), Err>>;
}

/// Returns the first alternative that matches in order.
pub fn alternative_ignore<'a, P, C, Err>(
    mut parsers: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<(), Err>
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

macro_rules! impl_alternative_body (
    // Origin
    ($_self:tt, $index:tt, $reader:tt, $($list:ident)+) => {{
        impl_alternative_body!(0 $_self $index $reader $($list)+);
    }};

    // Internal to build the tuple recursively.
    ($idx:tt $_self:tt $index:tt $reader:tt $list_first:ident $($list:ident)+) => {
        if $index == $idx {
            return Some($_self.$idx($reader).map(|_| ()));
        }

        $crate::successor!($idx impl_alternative_body $_self $index $reader $($list)+);
    };
    ($idx:tt $_self:tt $index:tt $reader:tt $list_first:ident) => {
        if $index == $idx {
            return Some($_self.$idx($reader).map(|_| ()));
        }
    };
);

macro_rules! impl_alternative_for_tuples (
    // The actual implementation.
    (__impl $($input:ident: $output:ident)+) => {
        impl<'a, C, $($input),+,$($output),+,Err> AlternativeIgnore<'a, C, Err> for ($($input),+,)
        where
            $($input: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<$output, Err>),+
        {
            fn choice(
                &mut self,
                index: usize,
                reader: &mut ParserInput<'a, Err, C>,
            ) -> Option<ParserResult<(), Err>> {
                impl_alternative_body!(self, index, reader, $($input)+);

                None
            }
        }
    };

    // Last implementation.
    ($input_last:ident: $output_last:ident) => {
        impl_alternative_for_tuples!(__impl $input_last: $output_last);
    };

    // Origin.
    ($($input:ident: $output:ident),+) => {
        impl_alternative_for_tuples!(__impl $($input: $output)+);
        impl_alternative_for_tuples!([$($input: $output)+]);
    };

    // To remove last -> last
    ([$input_last:ident: $output_last:ident] $($input_rev:ident: $output_rev:ident)+) => {
        impl_alternative_for_tuples!($($input_rev: $output_rev),*);
    };

    // To remove last -> middle steps
    ([$input_last:ident: $output_last:ident $($input_rest:ident: $output_rest:ident)+] $($input_rev:ident: $output_rev:ident)*) => {
        impl_alternative_for_tuples!([$($input_rest: $output_rest)*] $($input_rev: $output_rev)* $input_last: $output_last);  // recursion
    };
);

crate::execute_for_tuples!(impl_alternative_for_tuples);

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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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

            let mut reader = ParserInput::new("This is a test");
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
