use crate::parsers::helpers::not_found_restore;
use crate::result::ParserResult;
use crate::ParserInput;

/// Helper trait for the [tuple_ignore()] combinator.
pub trait TupleIgnore<'a, C, Err> {
    /// Parses the input and returns a tuple of results of each parser.
    fn parse(&mut self, reader: &mut ParserInput<'a, Err, C>) -> ParserResult<(), Err>;

    /// Parses the input and returns a tuple of results of each parser.
    /// Between each parser `separator` is executed and its result discarded.
    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut ParserInput<'a, Err, C>,
        separator: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<RSep, Err>;
}

/// Applies a tuple of parsers one by one ignoring their results.
pub fn tuple_ignore<'a, P, C, Err>(
    mut parsers: P,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<(), Err>
where
    P: TupleIgnore<'a, C, Err>,
{
    move |reader| parsers.parse(reader)
}

/// Applies a tuple of parsers one by one ignoring their results.
pub fn separated_tuple_ignore<'a, P, S, C, RSep, Err>(
    mut parsers: P,
    mut separator: S,
) -> impl FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<(), Err>
where
    P: TupleIgnore<'a, C, Err>,
    S: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<RSep, Err>,
{
    move |reader| parsers.parse_separated(reader, |r| separator(r))
}

macro_rules! impl_tuple_body (
    // Origin
    ($_self:tt, $reader:tt, $($list:ident)+) => {{
        impl_tuple_body!(0 $_self $reader $($list)+);
    }};

    // Internal to build the tuple recursively.
    ($idx:tt $_self:tt $reader:tt $list_first:ident $($list:ident)+) => {
        $_self.$idx($reader)?;
        $crate::successor!($idx impl_tuple_body $_self $reader $($list)+);
    };
    ($idx:tt $_self:tt $reader:tt $list_first:ident) => {
        $_self.$idx($reader)?;
    };
);

macro_rules! impl_tuple_body_separated (
    // Origin
    ($_self:tt, $reader:tt, $separator:tt, $($list:ident)+) => {{
        impl_tuple_body_separated!(0 $_self $reader $separator $($list)+);
    }};

    // Internal to build the tuple recursively.
    (0 $_self:tt $reader:tt $separator:tt $list_first:ident $($list:ident)+) => {
        $_self.0($reader)?;
        $crate::successor!(0 impl_tuple_body_separated $_self $reader $separator $($list)+);
    };
    ($idx:tt $_self:tt $reader:tt $separator:tt $list_first:ident $($list:ident)+) => {
        $separator($reader)?;
        $_self.$idx($reader)?;
        $crate::successor!($idx impl_tuple_body_separated $_self $reader $separator $($list)+);
    };
    (0 $_self:tt $reader:tt $separator:tt $list_first:ident) => {
        $_self.0($reader)?;
    };
    ($idx:tt $_self:tt $reader:tt $separator:tt $list_first:ident) => {
        $separator($reader)?;
        $_self.$idx($reader)?;
    };
);

macro_rules! impl_tuple_for_tuples (
    // The actual implementation.
    (__impl $($input:ident: $output:ident)+) => {
        impl<'a, C, $($input),+, $($output),+,Err> TupleIgnore<'a, C, Err> for ($($input),+,)
        where
            $($input: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<$output, Err>),+
        {
            fn parse(&mut self, reader: &mut ParserInput<'a, Err, C>) -> ParserResult<(), Err> {
                not_found_restore(move |reader| {
                    impl_tuple_body!(self, reader, $($input)+);
                    Ok(())
                })(reader)
            }

            fn parse_separated<S, RSep>(
                &mut self,
                reader: &mut ParserInput<'a, Err, C>,
                #[allow(unused_variables, unused_mut)]
                mut separator: S,
            ) -> ParserResult<(), Err>
                where
                    S: FnMut(&mut ParserInput<'a, Err, C>) -> ParserResult<RSep, Err>,
            {
                not_found_restore(move |reader| {
                    impl_tuple_body_separated!(self, reader, separator, $($input)+);
                    Ok(())
                })(reader)
            }
        }
    };

    // Last implementation.
    ($input_last:ident: $output_last:ident) => {
        impl_tuple_for_tuples!(__impl $input_last: $output_last);
    };

    // Origin.
    ($($input:ident: $output:ident),+) => {
        impl_tuple_for_tuples!(__impl $($input: $output)+);
        impl_tuple_for_tuples!([$($input: $output)+]);
    };

    // To remove last -> last
    ([$input_last:ident: $output_last:ident] $($input_rev:ident: $output_rev:ident)+) => {
        impl_tuple_for_tuples!($($input_rev: $output_rev),*);
    };

    // To remove last -> middle steps
    ([$input_last:ident: $output_last:ident $($input_rest:ident: $output_rest:ident)+] $($input_rev:ident: $output_rev:ident)*) => {
        impl_tuple_for_tuples!([$($input_rest: $output_rest)*] $($input_rev: $output_rev)* $input_last: $output_last);  // recursion
    };
);

crate::execute_for_tuples!(impl_tuple_for_tuples);

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::characters::read_text;

    use super::*;

    #[test]
    fn test_tuple1() {
        let tuple_size = 1;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = tuple_ignore((value.remove(0),));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_tuple2() {
        let tuple_size = 2;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = tuple_ignore((value.remove(0), value.remove(0)));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_tuple3() {
        let tuple_size = 3;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = tuple_ignore((value.remove(0), value.remove(0), value.remove(0)));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_tuple4() {
        let tuple_size = 4;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = tuple_ignore((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_tuple5() {
        let tuple_size = 5;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = tuple_ignore((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_separated_tuple1() {
        let tuple_size = 1;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("|");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = separated_tuple_ignore((value.remove(0),), read_text("|"));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_separated_tuple2() {
        let tuple_size = 2;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("|");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser =
                separated_tuple_ignore((value.remove(0), value.remove(0)), read_text("|"));
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_separated_tuple3() {
        let tuple_size = 3;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("|");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = separated_tuple_ignore(
                (value.remove(0), value.remove(0), value.remove(0)),
                read_text("|"),
            );
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_separated_tuple4() {
        let tuple_size = 4;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("|");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = separated_tuple_ignore(
                (
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                ),
                read_text("|"),
            );
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }

    #[test]
    fn test_separated_tuple5() {
        let tuple_size = 5;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("|");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = ParserInput::new(input.as_str());
            let mut parser = separated_tuple_ignore(
                (
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                ),
                read_text("|"),
            );
            let result = parser(&mut reader);
            assert_eq!(result, Ok(()), "Step: {}", i);
            assert_eq!(
                reader.byte_offset(),
                input.len(),
                "Step: {} - byte_offset",
                i
            );
        }
    }
}
