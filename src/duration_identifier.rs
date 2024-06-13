use chumsky::prelude::*;
use strum::*;

use crate::ErrType;
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    EnumCount,
    IntoStaticStr,
    EnumIter,
    EnumIs,
)]
#[repr(usize)]
#[non_exhaustive]
pub enum Unit {
    #[default]
    #[strum(serialize = "MS")]
    Milliseconds,
    #[strum(serialize = "S")]
    Seconds,
    #[strum(serialize = "m")]
    Minutes,
    #[strum(serialize = "H")]
    Hours,
    #[strum(serialize = "D")]
    Days,
    #[strum(serialize = "M")]
    Months,
    #[strum(serialize = "W")]
    Week,
    #[strum(serialize = "Y")]
    Year,
}
pub fn parse_unit<'src>() -> impl Parser<'src, &'src str, Unit, ErrType<'src>> {
    choice((
        parse_ms(),
        parse_seconds(),
        parse_minutes(),
        parse_hours(),
        parse_days(),
        parse_weeks(),
        parse_months(),
        parse_years(),
    ))
}
macro_rules! parser {
    ($fn:ident => $into:ident, [$($y:literal),+]) => {
        fn $fn<'src>() -> impl Parser<'src, &'src str, Unit, ErrType<'src>> {
            choice((
                $(just($y)),+
            )).map(|_| Unit::$into)
        }
    };
    ($fn:ident => $into:ident, $y:literal) => {
        fn $fn<'src>() -> impl Parser<'src, &'src str, Unit, ErrType<'src>> {
            just($y).map(|_| Unit::$into)
        }
    };
}
parser!(parse_ms => Milliseconds, ["MS", "ms"]);
parser!(parse_seconds => Seconds, "S");
parser!(parse_minutes => Minutes, "m");
parser!(parse_hours => Hours, "H");
parser!(parse_days => Days, "D");
parser!(parse_months => Months, "M");
parser!(parse_weeks => Week, "W");
parser!(parse_years => Year, "Y");

#[cfg(test)]
mod tests {
    use crate::{parse_unit, Unit};
    use chumsky::Parser;
    use pretty_assertions::assert_eq;
    fn check(raw: &str, unit: Unit) {
        let parse = parse_unit().parse(raw).into_result();
        if let Err(e) = parse {
            panic!("{:?}: {:?}", raw, e);
        }
        let parse = parse.unwrap();
        assert_eq!(parse, unit);
    }
    #[test]
    pub fn parse_milis() {
        check("MS", Unit::Milliseconds);
        check("ms", Unit::Milliseconds)
    }
    #[test]
    pub fn parse_seconds(){
        check("S", Unit::Seconds)
    }
    #[test]
    pub fn parse_minutes(){
        check("m", Unit::Minutes)
    }
    #[test]
    pub fn parse_hours(){
        check("H", Unit::Hours)
    }
    #[test]
    pub fn parse_days(){
        check("D", Unit::Days)
    }
    #[test]
    pub fn parse_months(){
        check("M", Unit::Months)
    }
    #[test]
    pub fn parse_weeks(){
        check("W", Unit::Week)
    }
    #[test]
    pub fn parse_years(){
        check("Y", Unit::Year)
    }
    
}
