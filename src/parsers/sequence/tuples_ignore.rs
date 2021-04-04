use crate::parsers::helpers::not_found_restore;
use crate::result::ParserResult;
use crate::Reader;

/// Helper trait for the [tuple_ignore()] combinator.
pub trait TupleIgnore<'a, C, Err> {
    /// Parses the input and returns a tuple of results of each parser.
    fn parse(&mut self, reader: &mut Reader<'a, Err, C>) -> ParserResult<(), Err>;

    /// Parses the input and returns a tuple of results of each parser.
    /// Between each parser `separator` is executed and its result discarded.
    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut Reader<'a, Err, C>,
        separator: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>;
}

/// Applies a tuple of parsers one by one ignoring their results.
pub fn tuple_ignore<'a, P, C, Err>(
    mut parsers: P,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<(), Err>
where
    P: TupleIgnore<'a, C, Err>,
{
    move |reader| parsers.parse(reader)
}

/// Applies a tuple of parsers one by one ignoring their results.
pub fn separated_tuple_ignore<'a, P, S, C, RSep, Err>(
    mut parsers: P,
    mut separator: S,
) -> impl FnMut(&mut Reader<'a, Err, C>) -> ParserResult<(), Err>
where
    P: TupleIgnore<'a, C, Err>,
    S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>,
{
    move |reader| parsers.parse_separated(reader, |r| separator(r))
}

impl<'a, C, T1, R1, Err> TupleIgnore<'a, C, Err> for (T1,)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
{
    fn parse(&mut self, reader: &mut Reader<'a, Err, C>) -> ParserResult<(), Err> {
        not_found_restore(move |reader| {
            self.0(reader)?;

            Ok(())
        })(reader)
    }

    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut Reader<'a, Err, C>,
        _: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>,
    {
        self.parse(reader)
    }
}

impl<'a, C, T1, T2, R1, R2, Err> TupleIgnore<'a, C, Err> for (T1, T2)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
{
    fn parse(&mut self, reader: &mut Reader<'a, Err, C>) -> ParserResult<(), Err> {
        not_found_restore(move |reader| {
            self.0(reader)?;
            self.1(reader)?;

            Ok(())
        })(reader)
    }

    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut Reader<'a, Err, C>,
        mut separator: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>,
    {
        not_found_restore(move |reader| {
            self.0(reader)?;
            separator(reader)?;
            self.1(reader)?;

            Ok(())
        })(reader)
    }
}

impl<'a, C, T1, T2, T3, R1, R2, R3, Err> TupleIgnore<'a, C, Err> for (T1, T2, T3)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
{
    fn parse(&mut self, reader: &mut Reader<'a, Err, C>) -> ParserResult<(), Err> {
        not_found_restore(move |reader| {
            self.0(reader)?;
            self.1(reader)?;
            self.2(reader)?;

            Ok(())
        })(reader)
    }

    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut Reader<'a, Err, C>,
        mut separator: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>,
    {
        not_found_restore(move |reader| {
            self.0(reader)?;
            separator(reader)?;
            self.1(reader)?;
            separator(reader)?;
            self.2(reader)?;

            Ok(())
        })(reader)
    }
}

impl<'a, C, T1, T2, T3, T4, R1, R2, R3, R4, Err> TupleIgnore<'a, C, Err> for (T1, T2, T3, T4)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
{
    fn parse(&mut self, reader: &mut Reader<'a, Err, C>) -> ParserResult<(), Err> {
        not_found_restore(move |reader| {
            self.0(reader)?;
            self.1(reader)?;
            self.2(reader)?;
            self.3(reader)?;

            Ok(())
        })(reader)
    }

    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut Reader<'a, Err, C>,
        mut separator: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>,
    {
        not_found_restore(move |reader| {
            self.0(reader)?;
            separator(reader)?;
            self.1(reader)?;
            separator(reader)?;
            self.2(reader)?;
            separator(reader)?;
            self.3(reader)?;

            Ok(())
        })(reader)
    }
}

impl<'a, C, T1, T2, T3, T4, T5, R1, R2, R3, R4, R5, Err> TupleIgnore<'a, C, Err>
    for (T1, T2, T3, T4, T5)
where
    T1: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R1, Err>,
    T2: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R2, Err>,
    T3: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R3, Err>,
    T4: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R4, Err>,
    T5: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<R5, Err>,
{
    fn parse(&mut self, reader: &mut Reader<'a, Err, C>) -> ParserResult<(), Err> {
        not_found_restore(move |reader| {
            self.0(reader)?;
            self.1(reader)?;
            self.2(reader)?;
            self.3(reader)?;
            self.4(reader)?;

            Ok(())
        })(reader)
    }

    fn parse_separated<S, RSep>(
        &mut self,
        reader: &mut Reader<'a, Err, C>,
        mut separator: S,
    ) -> ParserResult<(), Err>
    where
        S: FnMut(&mut Reader<'a, Err, C>) -> ParserResult<RSep, Err>,
    {
        not_found_restore(move |reader| {
            self.0(reader)?;
            separator(reader)?;
            self.1(reader)?;
            separator(reader)?;
            self.2(reader)?;
            separator(reader)?;
            self.3(reader)?;
            separator(reader)?;
            self.4(reader)?;

            Ok(())
        })(reader)
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
    fn test_tuple1() {
        let tuple_size = 1;
        for i in 0..tuple_size {
            let texts: Vec<_> = (0..tuple_size)
                .into_iter()
                .map(|i| format!("{}", i))
                .collect();
            let input: String = texts.join("");
            let mut value: Vec<_> = texts.iter().map(|t| read_text(t.as_str())).collect();

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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

            let mut reader = Reader::new(input.as_str());
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
