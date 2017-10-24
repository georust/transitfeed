use serde;
use serde::Deserializer;
use chrono::{Duration, NaiveDate};
use transit::{ExceptionType, LocationType, WheelchairBoarding, FrequencyAccuracy, PickupType,
              DropoffType, TimeOffset, RouteType, Timepoint, WheelchairAccessible,
              BikesAllowed, PaymentMethod, Transfers, TransferType};

pub fn deserialize_dow_field<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>
{
    let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
    match result {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::custom("day of week field was not 0 or 1"))
    }
}

pub fn deserialize_calendardate<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match NaiveDate::parse_from_str(&result, "%Y%m%d") {
        Ok(d) => Ok(d),
        Err(e) => Err(serde::de::Error::custom(format!("Date must be in YYYYMMDD format: {}", e)))
    }
}

pub fn deserialize_exceptiontype<'de, D>(deserializer: D) -> Result<ExceptionType, D::Error>
    where D: Deserializer<'de>
{
    let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
    match result {
        1 => Ok(ExceptionType::ServiceAdded),
        2 => Ok(ExceptionType::ServiceRemoved),
        _ => Err(serde::de::Error::custom("Exception type field was not 1 or 2"))
    }
}

pub fn deserialize_timeoffset<'de, D>(deserializer: D) -> Result<TimeOffset, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    let mut parts = result.trim().split(':');
    let parse_part = |part: Option<&str>| -> Result<u32, D::Error> {
        match part {
            Some(val) => match val.parse() {
                Ok(x) => Ok(x),
                Err(y) => Err(serde::de::Error::custom(y))
            },
            None => Err(serde::de::Error::custom("Unexpected timeoffset part"))
        }
    };
    let hours = try!(parse_part(parts.next()));
    let minutes = try!(parse_part(parts.next()));
    let seconds = try!(parse_part(parts.next()));
    Ok(TimeOffset::from_hms(hours, minutes, seconds))
}

pub fn deserialize_frequencyaccuracy<'de, D>(deserializer: D) -> Result<FrequencyAccuracy, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(FrequencyAccuracy::Approximate),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(FrequencyAccuracy::Approximate),
            Ok(1) => Ok(FrequencyAccuracy::Exact),
            _ => Err(serde::de::Error::custom("Frequency accuracy must be 0 or 1")),
        }
    }
}

pub fn deserialize_routetype<'de, D>(deserializer: D) -> Result<RouteType, D::Error>
    where D: Deserializer<'de>
{
    let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
    match result {
        0 => Ok(RouteType::LightRail),
        1 => Ok(RouteType::Subway),
        2 => Ok(RouteType::Rail),
        3 => Ok(RouteType::Bus),
        4 => Ok(RouteType::Ferry),
        5 => Ok(RouteType::CableCar),
        6 => Ok(RouteType::Gondola),
        7 => Ok(RouteType::Funicular),
        _ => Err(serde::de::Error::custom("Route type must (currently) be 0-7")),
    }
}

pub fn deserialize_locationtype<'de, D>(deserializer: D) -> Result<LocationType, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(LocationType::Stop),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(LocationType::Stop),
            Ok(1) => Ok(LocationType::Station),
            _ => Err(serde::de::Error::custom("Location type must (currently) be 0 or 1")),
        }
    }
}

pub fn deserialize_wheelchairboarding<'de, D>(deserializer: D) -> Result<WheelchairBoarding, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(WheelchairBoarding::NoInformation),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(WheelchairBoarding::NoInformation),
            Ok(1) => Ok(WheelchairBoarding::SomeAccessibility),
            Ok(2) => Ok(WheelchairBoarding::NoAccessibility),
            _ => Err(serde::de::Error::custom("Wheelchair boarding must be between 0 and 2"))
        }
    }
}

pub fn deserialize_wheelchairaccessible<'de, D>(deserializer: D) -> Result<WheelchairAccessible, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(WheelchairAccessible::NoInformation),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(WheelchairAccessible::NoInformation),
            Ok(1) => Ok(WheelchairAccessible::SomeAccessibility),
            Ok(2) => Ok(WheelchairAccessible::NoAccessibility),
            _ => Err(serde::de::Error::custom("Wheelchair accessibility must be between 0 and 2"))
        }
    }
}

pub fn deserialize_bikesallowed<'de, D>(deserializer: D) -> Result<BikesAllowed, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(BikesAllowed::NoInformation),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(BikesAllowed::NoInformation),
            Ok(1) => Ok(BikesAllowed::SomeBikes),
            Ok(2) => Ok(BikesAllowed::NoBikes),
            _ => Err(serde::de::Error::custom("Bikes allowed must be between 0 and 2"))
        }
    }
}

pub fn deserialize_pickuptype<'de, D>(deserializer: D) -> Result<PickupType, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(PickupType::RegularlyScheduled),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(PickupType::RegularlyScheduled),
            Ok(1) => Ok(PickupType::NoPickupAvailable),
            Ok(2) => Ok(PickupType::MustPhoneAgency),
            Ok(3) => Ok(PickupType::MustCoordinateWithDriver),
            _ => Err(serde::de::Error::custom("Pickup type must be between 0 and 3")),
        }
    }
}

pub fn deserialize_dropofftype<'de, D>(deserializer: D) -> Result<DropoffType, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(DropoffType::RegularlyScheduled),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(DropoffType::RegularlyScheduled),
            Ok(1) => Ok(DropoffType::NoDropoffAvailable),
            Ok(2) => Ok(DropoffType::MustPhoneAgency),
            Ok(3) => Ok(DropoffType::MustCoordinateWithDriver),
            _ => Err(serde::de::Error::custom("Dropoff type must be between 0 and 3")),
        }
    }
}

pub fn deserialize_timepoint<'de, D>(deserializer: D) -> Result<Timepoint, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(Timepoint::Exact),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(Timepoint::Approximate),
            Ok(1) => Ok(Timepoint::Exact),
            _ => Err(serde::de::Error::custom("Timepoint must be 0 or 1"))
        }
    }
}

pub fn deserialize_paymentmethod<'de, D>(deserializer: D) -> Result<PaymentMethod, D::Error>
    where D: Deserializer<'de>
{
    let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
    match result {
        0 => Ok(PaymentMethod::PaidOnboard),
        1 => Ok(PaymentMethod::PaidBefore),
        _ => Err(serde::de::Error::custom("payment method must be 0 or 1"))
    }
}

pub fn deserialize_transfers<'de, D>(deserializer: D) -> Result<Transfers, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(Transfers::Unlimited),
        r => match r.parse::<u32>() {
            Ok(0) => Ok(Transfers::None),
            Ok(1) => Ok(Transfers::TransferOnce),
            Ok(2) => Ok(Transfers::TransferTwice),
            _ => Err(serde::de::Error::custom("transfers must be between 0 and 2 or blank"))
        }
    }
}

pub fn deserialize_transferduration<'de, D>(deserializer:D) -> Result<Option<Duration>, D::Error>
    where D: Deserializer<'de>
{
    let result : String = try!(serde::Deserialize::deserialize(deserializer));
    match result.trim() {
        "" => Ok(None),
        r => match r.parse::<i64>() {
            Ok(x) => Ok(Some(Duration::seconds(x))),
            Err(_) => Err(serde::de::Error::custom("transfers duration must be a number or blank"))
        }
    }
}

pub fn deserialize_transfertype<'de, D>(deserializer: D) -> Result<TransferType, D::Error>
    where D: Deserializer<'de>
{
    let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
    match result {
        0 => Ok(TransferType::Recommended),
        1 => Ok(TransferType::Timed),
        2 => Ok(TransferType::MinimumTime),
        3 => Ok(TransferType::NotPossible),
        _ => Err(serde::de::Error::custom("transfer type must be between 0 and 3"))
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
