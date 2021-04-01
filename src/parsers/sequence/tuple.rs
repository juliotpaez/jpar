use crate::parsers::helpers::not_found_restore;
use crate::result::ParserResult;
use crate::Reader;

/// Helper trait for the [tuple()] combinator.
pub trait Tuple<'a, C, R> {
    /// Parses the input and returns a tuple of results of each parser.
    fn parse(&mut self, reader: &mut Reader<'a, C>) -> ParserResult<R>;

    /// Parses the input and returns a tuple of results of each parser.
    /// Between each parser `separator` is executed and its result discarded.
    fn parse_separated<RSep>(
        &mut self,
        reader: &mut Reader<'a, C>,
        separator: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
    ) -> ParserResult<R>;
}

/// Applies a tuple of parsers one by one and returns their results as a tuple.
pub fn tuple<'a, C, R>(
    mut parsers: impl Tuple<'a, C, R>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |reader| parsers.parse(reader)
}

/// Applies a tuple of parsers one by one and returns their results as a tuple.
pub fn tuple_separated<'a, C, R, RSep>(
    mut parsers: impl Tuple<'a, C, R>,
    mut separator: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
) -> impl FnMut(&mut Reader<'a, C>) -> ParserResult<R> {
    move |reader| parsers.parse_separated(reader, |r| separator(r))
}

impl<'a, C, T1, R1> Tuple<'a, C, (R1,)> for (T1,)
where
    T1: FnMut(&mut Reader<'a, C>) -> ParserResult<R1>,
{
    fn parse(&mut self, reader: &mut Reader<'a, C>) -> ParserResult<(R1,)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;

            Ok((r0,))
        })(reader)
    }

    fn parse_separated<RSep>(
        &mut self,
        reader: &mut Reader<'a, C>,
        _: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
    ) -> ParserResult<(R1,)> {
        self.parse(reader)
    }
}

impl<'a, C, T1, T2, R1, R2> Tuple<'a, C, (R1, R2)> for (T1, T2)
where
    T1: FnMut(&mut Reader<'a, C>) -> ParserResult<R1>,
    T2: FnMut(&mut Reader<'a, C>) -> ParserResult<R2>,
{
    fn parse(&mut self, reader: &mut Reader<'a, C>) -> ParserResult<(R1, R2)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            let r1 = self.1(reader)?;

            Ok((r0, r1))
        })(reader)
    }

    fn parse_separated<RSep>(
        &mut self,
        reader: &mut Reader<'a, C>,
        mut separator: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
    ) -> ParserResult<(R1, R2)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            separator(reader)?;
            let r1 = self.1(reader)?;

            Ok((r0, r1))
        })(reader)
    }
}

impl<'a, C, T1, T2, T3, R1, R2, R3> Tuple<'a, C, (R1, R2, R3)> for (T1, T2, T3)
where
    T1: FnMut(&mut Reader<'a, C>) -> ParserResult<R1>,
    T2: FnMut(&mut Reader<'a, C>) -> ParserResult<R2>,
    T3: FnMut(&mut Reader<'a, C>) -> ParserResult<R3>,
{
    fn parse(&mut self, reader: &mut Reader<'a, C>) -> ParserResult<(R1, R2, R3)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            let r1 = self.1(reader)?;
            let r2 = self.2(reader)?;

            Ok((r0, r1, r2))
        })(reader)
    }

    fn parse_separated<RSep>(
        &mut self,
        reader: &mut Reader<'a, C>,
        mut separator: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
    ) -> ParserResult<(R1, R2, R3)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            separator(reader)?;
            let r1 = self.1(reader)?;
            separator(reader)?;
            let r2 = self.2(reader)?;

            Ok((r0, r1, r2))
        })(reader)
    }
}

impl<'a, C, T1, T2, T3, T4, R1, R2, R3, R4> Tuple<'a, C, (R1, R2, R3, R4)> for (T1, T2, T3, T4)
where
    T1: FnMut(&mut Reader<'a, C>) -> ParserResult<R1>,
    T2: FnMut(&mut Reader<'a, C>) -> ParserResult<R2>,
    T3: FnMut(&mut Reader<'a, C>) -> ParserResult<R3>,
    T4: FnMut(&mut Reader<'a, C>) -> ParserResult<R4>,
{
    fn parse(&mut self, reader: &mut Reader<'a, C>) -> ParserResult<(R1, R2, R3, R4)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            let r1 = self.1(reader)?;
            let r2 = self.2(reader)?;
            let r3 = self.3(reader)?;

            Ok((r0, r1, r2, r3))
        })(reader)
    }

    fn parse_separated<RSep>(
        &mut self,
        reader: &mut Reader<'a, C>,
        mut separator: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
    ) -> ParserResult<(R1, R2, R3, R4)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            separator(reader)?;
            let r1 = self.1(reader)?;
            separator(reader)?;
            let r2 = self.2(reader)?;
            separator(reader)?;
            let r3 = self.3(reader)?;

            Ok((r0, r1, r2, r3))
        })(reader)
    }
}

impl<'a, C, T1, T2, T3, T4, T5, R1, R2, R3, R4, R5> Tuple<'a, C, (R1, R2, R3, R4, R5)>
    for (T1, T2, T3, T4, T5)
where
    T1: FnMut(&mut Reader<'a, C>) -> ParserResult<R1>,
    T2: FnMut(&mut Reader<'a, C>) -> ParserResult<R2>,
    T3: FnMut(&mut Reader<'a, C>) -> ParserResult<R3>,
    T4: FnMut(&mut Reader<'a, C>) -> ParserResult<R4>,
    T5: FnMut(&mut Reader<'a, C>) -> ParserResult<R5>,
{
    fn parse(&mut self, reader: &mut Reader<'a, C>) -> ParserResult<(R1, R2, R3, R4, R5)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            let r1 = self.1(reader)?;
            let r2 = self.2(reader)?;
            let r3 = self.3(reader)?;
            let r4 = self.4(reader)?;

            Ok((r0, r1, r2, r3, r4))
        })(reader)
    }

    fn parse_separated<RSep>(
        &mut self,
        reader: &mut Reader<'a, C>,
        mut separator: impl FnMut(&mut Reader<'a, C>) -> ParserResult<RSep>,
    ) -> ParserResult<(R1, R2, R3, R4, R5)> {
        not_found_restore(move |reader| {
            let r0 = self.0(reader)?;
            separator(reader)?;
            let r1 = self.1(reader)?;
            separator(reader)?;
            let r2 = self.2(reader)?;
            separator(reader)?;
            let r3 = self.3(reader)?;
            separator(reader)?;
            let r4 = self.4(reader)?;

            Ok((r0, r1, r2, r3, r4))
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
            let mut parser = tuple((value.remove(0),));
            let result = parser(&mut reader);
            assert_eq!(result, Ok((texts[0].as_str(),)), "Step: {}", i);
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
            let mut parser = tuple((value.remove(0), value.remove(0)));
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((texts[0].as_str(), texts[1].as_str())),
                "Step: {}",
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
            let mut parser = tuple((value.remove(0), value.remove(0), value.remove(0)));
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((texts[0].as_str(), texts[1].as_str(), texts[2].as_str())),
                "Step: {}",
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
            let mut parser = tuple((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ));
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((
                    texts[0].as_str(),
                    texts[1].as_str(),
                    texts[2].as_str(),
                    texts[3].as_str()
                )),
                "Step: {}",
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
            let mut parser = tuple((
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
                value.remove(0),
            ));
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((
                    texts[0].as_str(),
                    texts[1].as_str(),
                    texts[2].as_str(),
                    texts[3].as_str(),
                    texts[4].as_str()
                )),
                "Step: {}",
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
            let mut parser = tuple_separated((value.remove(0),), read_text("|"));
            let result = parser(&mut reader);
            assert_eq!(result, Ok((texts[0].as_str(),)), "Step: {}", i);
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
            let mut parser = tuple_separated((value.remove(0), value.remove(0)), read_text("|"));
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((texts[0].as_str(), texts[1].as_str())),
                "Step: {}",
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
            let mut parser = tuple_separated(
                (value.remove(0), value.remove(0), value.remove(0)),
                read_text("|"),
            );
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((texts[0].as_str(), texts[1].as_str(), texts[2].as_str())),
                "Step: {}",
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
            let mut parser = tuple_separated(
                (
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                    value.remove(0),
                ),
                read_text("|"),
            );
            let result = parser(&mut reader);
            assert_eq!(
                result,
                Ok((
                    texts[0].as_str(),
                    texts[1].as_str(),
                    texts[2].as_str(),
                    texts[3].as_str()
                )),
                "Step: {}",
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
            let mut parser = tuple_separated(
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
            assert_eq!(
                result,
                Ok((
                    texts[0].as_str(),
                    texts[1].as_str(),
                    texts[2].as_str(),
                    texts[3].as_str(),
                    texts[4].as_str()
                )),
                "Step: {}",
                i
            );
        }
    }
}
