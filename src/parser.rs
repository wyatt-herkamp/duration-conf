use crate::{parse_unit, ErrType, Unit};
use chumsky::prelude::*;
use std::{fmt::Display, str::FromStr};

pub fn parse_number<'src>() -> impl Parser<'src, &'src str, i64, ErrType<'src>> {
    text::int(10).map(|v| i64::from_str(v).unwrap())
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SetOfTime {
    pub value: i64,
    pub unit: Unit,
}
impl Display for SetOfTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}
pub fn parse_set_of_time<'src>() -> impl Parser<'src, &'src str, SetOfTime, ErrType<'src>> {
    parse_number()
        .then(parse_unit())
        .map(|(value, unit)| SetOfTime { value, unit })
}
pub fn parse_repeated_set_of_time<'src>(
) -> impl Parser<'src, &'src str, Vec<SetOfTime>, ErrType<'src>> {
    parse_set_of_time()
        .then_ignore(just(' ').or_not())
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
}
#[cfg(test)]
mod repeated_test {
    use super::*;
    use crate::{parse_repeated_set_of_time, Unit};
    use pretty_assertions::assert_eq;
    fn check(raw: &str, expected: &[(i64, Unit)]) {
        let parse = parse_repeated_set_of_time().parse(raw).into_result();
        if let Err(e) = parse {
            panic!("{:?}: {:?}", raw, e);
        }
        let parse = parse.unwrap();
        assert_eq!(parse.len(), expected.len());
        for (i, (value, unit)) in expected.iter().enumerate() {
            assert_eq!(parse[i].value, *value);
            assert_eq!(parse[i].unit, *unit);
        }
    }
    #[test]
    pub fn parse_milis() {
        check("1MS", &[(1, Unit::Milliseconds)]);
        check("1MS 2S", &[(1, Unit::Milliseconds), (2, Unit::Seconds)]);
        check(
            "1ms2S3H",
            &[
                (1, Unit::Milliseconds),
                (2, Unit::Seconds),
                (3, Unit::Hours),
            ],
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    fn check(raw: &str, unit: Unit, value: i64) {
        let parse = parse_set_of_time().parse(raw).into_result();
        if let Err(e) = parse {
            panic!("{:?}: {:?}", raw, e);
        }
        let parse = parse.unwrap();
        assert_eq!(parse.value, value);
        assert_eq!(parse.unit, unit);
    }
    #[test]
    pub fn parse_milis() {
        check("1MS", Unit::Milliseconds, 1);
        check("1ms", Unit::Milliseconds, 1)
    }
    #[test]
    pub fn parse_seconds() {
        check("1S", Unit::Seconds, 1)
    }
    #[test]
    pub fn parse_minutes() {
        check("1m", Unit::Minutes, 1)
    }
    #[test]
    pub fn parse_hours() {
        check("1H", Unit::Hours, 1)
    }
    #[test]
    pub fn parse_days() {
        check("1D", Unit::Days, 1)
    }
    #[test]
    pub fn parse_months() {
        check("1M", Unit::Months, 1)
    }
    #[test]
    pub fn parse_weeks() {
        check("1W", Unit::Week, 1)
    }
    #[test]
    pub fn parse_years() {
        check("1Y", Unit::Year, 1)
    }
}
