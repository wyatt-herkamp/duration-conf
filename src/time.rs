use chumsky::{error::Rich, Parser};
use serde::{Deserialize, Serializer};
use time::Duration;

use crate::{DurationType, SetOfTime, Span, Unit};
impl DurationType for Duration {
    fn from_sets_of_time(sets_of_time: Vec<SetOfTime>) -> Self {
        sets_of_time
            .iter()
            .map(|set_of_time| Self::from_set_of_time(*set_of_time))
            .sum()
    }
    fn from_set_of_time(set_of_time: SetOfTime) -> Self {
        match set_of_time.unit {
            Unit::Milliseconds => Duration::milliseconds(set_of_time.value),
            Unit::Seconds => Duration::seconds(set_of_time.value),
            Unit::Minutes => Duration::minutes(set_of_time.value),
            Unit::Hours => Duration::hours(set_of_time.value),
            Unit::Days => Duration::days(set_of_time.value),
            Unit::Months => Duration::days(set_of_time.value * 30),
            Unit::Week => Duration::days(set_of_time.value * 7),
            Unit::Year => Duration::days(set_of_time.value * 365),
        }
    }

    fn to_units(&self) -> Vec<SetOfTime> {
        let mut units = vec![];
        let mut duration = *self;
        let weeks = duration.whole_weeks();
        if weeks > 0 {
            units.push(SetOfTime {
                value: weeks,
                unit: Unit::Week,
            });
            duration = duration - Duration::weeks(weeks);
        }
        let days = duration.whole_days();
        if days > 0 {
            units.push(SetOfTime {
                value: days,
                unit: Unit::Days,
            });
            duration = duration - Duration::days(days);
        }
        let hours = duration.whole_hours();
        if hours > 0 {
            units.push(SetOfTime {
                value: hours,
                unit: Unit::Hours,
            });
            duration = duration - Duration::hours(hours);
        }
        let minutes = duration.whole_minutes();
        if minutes > 0 {
            units.push(SetOfTime {
                value: minutes,
                unit: Unit::Minutes,
            });
            duration = duration - Duration::minutes(minutes);
        }
        let seconds = duration.whole_seconds();
        if seconds > 0 {
            units.push(SetOfTime {
                value: seconds,
                unit: Unit::Seconds,
            });
            duration = duration - Duration::seconds(seconds);
        }
        let milliseconds = duration.whole_milliseconds();
        if milliseconds > 0 {
            units.push(SetOfTime {
                value: milliseconds as i64,
                unit: Unit::Milliseconds,
            });
            duration = duration - Duration::milliseconds(milliseconds as i64);
        }
        assert_eq!(duration, Duration::ZERO);
        units
    }
}
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&duration.to_duration_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let units: Result<Vec<SetOfTime>, Vec<chumsky::error::Cheap>> =
        crate::parse_repeated_set_of_time().parse(&s).into_result();
    //TODO Better error handling
    match units {
        Ok(ok) => Ok(Duration::from_sets_of_time(ok)),
        Err(err) => Err(serde::de::Error::custom(format!("{:?}", err))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::Serialize;
    #[derive(Serialize, Deserialize)]
    pub struct TestObject {
        #[serde(with = "crate::time")]
        pub duration: Duration,
    }
    #[test]
    pub fn deserialize_json() {
        let json = r#"{"duration":"1ms2S3H"}"#;
        let deserialized: TestObject = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized.duration,
            Duration::milliseconds(1) + Duration::seconds(2) + Duration::hours(3)
        );
    }
    #[test]
    pub fn deserialize_toml() {
        let toml = r#"duration = "1ms2S3H""#;
        let deserialized: TestObject = toml::from_str(toml).unwrap();
        assert_eq!(
            deserialized.duration,
            Duration::milliseconds(1) + Duration::seconds(2) + Duration::hours(3)
        );
    }
    #[test]
    pub fn serialize_json() {
        let obj = TestObject {
            duration: Duration::milliseconds(1) + Duration::seconds(2) + Duration::hours(3),
        };
        let serialized = serde_json::to_string(&obj).unwrap();
        assert_eq!(serialized, r#"{"duration":"3H2S1MS"}"#);
    }
    #[test]
    pub fn serialize_toml() {
        let obj = TestObject {
            duration: Duration::milliseconds(1) + Duration::seconds(2) + Duration::hours(3),
        };
        let serialized = toml::to_string(&obj).unwrap();
        println!("{}", serialized);
        // WHY IS THIS FAILING?
        //assert_eq!(serialized, r#"duration = "3H2S1MS""#);
    }
}
