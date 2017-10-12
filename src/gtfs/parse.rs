use std::str::FromStr;
use chrono::NaiveDate;
use transit::{ExceptionType, LocationType, WheelchairBoarding, FrequencyAccuracy, PickupType,
              DropoffType, TimeOffset, RouteType, WheelchairAccessible, BikesAllowed};
use gtfs::error::ParseError;


/// Parse an Integer with line and file numbers given for error reporting
pub fn parse_int<T: FromStr>(val: &str) -> Result<T, ParseError> {
    match val.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::ParseInt(String::from(val))),
    }
}

/// Parse a Float with line and file numbers given for error reporting
pub fn parse_float<T: FromStr>(val: &str) -> Result<T, ParseError> {
    match val.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError::ParseFloat(String::from(val))),
    }
}

/// Parse a day of week service bit
pub fn parse_dow(val: &str) -> Result<bool, ParseError>
{
    match val.parse::<u32>() {
        Ok(0) => Ok(false),
        Ok(1) => Ok(true),
        Ok(_) | Err(_) => Err(ParseError::ParseInt(String::from(val))),
    }
}

/// Parse a CalendarDate ExceptionType
pub fn parse_exceptiontype(val: &str) -> Result<ExceptionType, ParseError>
{
    match val.parse::<u32>() {
        Ok(1) => Ok(ExceptionType::ServiceAdded),
        Ok(2) => Ok(ExceptionType::ServiceRemoved),
        Ok(_) | Err(_) => Err(ParseError::ParseInt(String::from(val))),
    }
}

/// Parse a frequencie exact_times field. Returns true when times are exactly scheduled
pub fn parse_exact_times(val: &str) -> Result<FrequencyAccuracy, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(FrequencyAccuracy::Approximate),
        "1" => Ok(FrequencyAccuracy::Exact),
        _ => Err(ParseError::ParseExactTimes(String::from(val))),
    }
}

/// Parse a &str containing a date in gtfs and return NaiveDate
pub fn parse_date(val: &str) -> Result<NaiveDate, ParseError>
{
    match NaiveDate::parse_from_str(val, "%Y%m%d") {
        Ok(d) => Ok(d),
        Err(_) => Err(ParseError::ParseDate(String::from(val)))
    }
}

/// Takes a &str containing an arrival/departure time for gtfs and returns
/// a naivetime. Chrono's NaiveTime parser is relatively slow and doesn't
/// account for the optional leading zeros in the hour part.
pub fn parse_timeoffset(val: &str) -> Result<TimeOffset, ParseError> {
    let mut parts = val.trim().split(':');
    let parse_part = |part: Option<&str>| -> Result<u32, ParseError> {
        match part {
            Some(val) => Ok(try!(parse_int(val))),
            None => Err(ParseError::ParseTime(String::from(val))),
        }
    };
    let hours = try!(parse_part(parts.next()));
    let minutes = try!(parse_part(parts.next()));
    let seconds = try!(parse_part(parts.next()));
    Ok(TimeOffset::from_hms(hours, minutes, seconds))
}

/// Takes a &str containing an stop time pickup type for gtfs and returns
/// a `PickupType` enum.
pub fn parse_pickup_type(val: &str) -> Result<PickupType, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        "" => Ok(PickupType::RegularlyScheduled),
        "0" => Ok(PickupType::RegularlyScheduled),
        "1" => Ok(PickupType::NoPickupAvailable),
        "2" => Ok(PickupType::MustPhoneAgency),
        "3" => Ok(PickupType::MustCoordinateWithDriver),
        _ => Err(ParseError::ParsePickupType(String::from(val))),
    }
}

/// Takes a &str containing an stop time dropoff type for gtfs and returns
/// a `DropoffType` enum.
pub fn parse_dropoff_type(val: &str) -> Result<DropoffType, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        "" => Ok(DropoffType::RegularlyScheduled),
        "0" => Ok(DropoffType::RegularlyScheduled),
        "1" => Ok(DropoffType::NoDropoffAvailable),
        "2" => Ok(DropoffType::MustPhoneAgency),
        "3" => Ok(DropoffType::MustCoordinateWithDriver),
        _ => Err(ParseError::ParseDropoffType(String::from(val))),
    }
}

/// Takes a &str containing an location type for gtfs and returns
/// a `LocationType` enum.
pub fn parse_location_type(val: &str) -> Result<LocationType, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(LocationType::Stop),
        "1" => Ok(LocationType::Station),
        _ => Err(ParseError::ParseLocationType(String::from(val))),
    }
}

/// Takes a &str containing wheelchair boarding information for gtfs and returns
/// a `WheelchairBoarding` enum.
pub fn parse_wheelchair_boarding(val: &str) -> Result<WheelchairBoarding, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(WheelchairBoarding::NoInformation),
        "1" => Ok(WheelchairBoarding::SomeAccessibility),
        "2" => Ok(WheelchairBoarding::NoAccessibility),
        _ => Err(ParseError::ParseWheelchairBoarding(String::from(val))),
    }
}

/// Takes a &str containing route type information for gtfs and returns a
/// `RouteType` enum.
pub fn parse_route_type(val: &str) -> Result<RouteType, ParseError> {
    let trimmed = val.trim();
    match try!(parse_int(trimmed)) {
        0 => Ok(RouteType::LightRail),
        1 => Ok(RouteType::Subway),
        2 => Ok(RouteType::Rail),
        3 => Ok(RouteType::Bus),
        4 => Ok(RouteType::Ferry),
        5 => Ok(RouteType::CableCar),
        6 => Ok(RouteType::Gondola),
        7 => Ok(RouteType::Funicular),
        _ => Err(ParseError::ParseRouteType(String::from(val))),
    }
}

/// Takes a &str containing wheelchair accessibility information for gtfs and return a
/// `WheelchairAccessible` enum.
pub fn parse_wheelchair_accessible(val: &str) -> Result<WheelchairAccessible, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        _ => Err(ParseError::ParseWheelchairAccessible(String::from(val))),
    }
}

/// Takes a &str containing bikes allowed information for gtfs and return a
/// `BikesAllowed` enum.
pub fn parse_bikes_allowed(val: &str) -> Result<BikesAllowed, ParseError> {
    let trimmed = val.trim();
    match trimmed {
        _ => Err(ParseError::ParseBikesAllowed(String::from(val))),
    }
}


macro_rules! parse_try {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err(e),
        }
    }
}


#[test]
fn parse_timeoffset_test() {
    assert_eq!(parse_timeoffset("01:01:01").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert_eq!(parse_timeoffset("1:01:01").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert_eq!(parse_timeoffset("01:01:01  ").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert_eq!(parse_timeoffset(" 01:01:01  ").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert!(parse_timeoffset(":01:01").is_err());
    assert!(parse_timeoffset("ab:01:01").is_err());
    assert!(parse_timeoffset("01::01").is_err());
}
