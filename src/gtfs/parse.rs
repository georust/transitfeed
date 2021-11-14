use chrono::{Duration, NaiveDate};
use serde::Deserializer;

pub fn deserialize_dow_field<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let result: u32 = serde::Deserialize::deserialize(deserializer)?;
    match result {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::custom("day of week field was not 0 or 1")),
    }
}

pub fn deserialize_calendardate<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let result: String = serde::Deserialize::deserialize(deserializer)?;
    match NaiveDate::parse_from_str(&result, "%Y%m%d") {
        Ok(d) => Ok(d),
        Err(e) => Err(serde::de::Error::custom(format!(
            "Date must be in YYYYMMDD format: {}",
            e
        ))),
    }
}

pub fn deserialize_option_calendardate<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: String = serde::Deserialize::deserialize(deserializer)?;
    match result.as_ref() {
        "" => Ok(None),
        s => match NaiveDate::parse_from_str(s, "%Y%m%d") {
            Ok(d) => Ok(Some(d)),
            Err(e) => Err(serde::de::Error::custom(format!(
                "Date must be in YYYYMMDD format: {}",
                e
            ))),
        },
    }
}

pub fn deserialize_transferduration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: String = serde::Deserialize::deserialize(deserializer)?;
    match result.trim() {
        "" => Ok(None),
        r => match r.parse::<i64>() {
            Ok(x) => Ok(Some(Duration::seconds(x))),
            Err(_) => Err(serde::de::Error::custom(
                "transfers duration must be a number or blank",
            )),
        },
    }
}

//#[test]
//fn parse_timeoffset_test() {
//    assert_eq!(parse_timeoffset("01:01:01").unwrap(), TimeOffset::from_hms(1, 1, 1));
//    assert_eq!(parse_timeoffset("1:01:01").unwrap(), TimeOffset::from_hms(1, 1, 1));
//    assert_eq!(parse_timeoffset("01:01:01  ").unwrap(), TimeOffset::from_hms(1, 1, 1));
//    assert_eq!(parse_timeoffset(" 01:01:01  ").unwrap(), TimeOffset::from_hms(1, 1, 1));
//    assert!(parse_timeoffset(":01:01").is_err());
//    assert!(parse_timeoffset("ab:01:01").is_err());
//    assert!(parse_timeoffset("01::01").is_err());
//}
