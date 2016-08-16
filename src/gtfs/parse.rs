use std::str::FromStr;
use transit::{LocationType, WheelchairBoarding, PickupType, DropoffType, TimeOffset, RouteType};
use gtfs::error::{GtfsError, GtfsResult};


/// Parse an Integer with line and file numbers given for error reporting
pub fn parse_int<T: FromStr>(line: usize, file: &str, val: &str) -> GtfsResult<T> {
    match val.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_) => Err(GtfsError::ParseInt(line, String::from(file), String::from(val))),
    }
}

/// Parse a Float with line and file numbers given for error reporting
pub fn parse_float<T: FromStr>(line: usize, file: &str, val: &str) -> GtfsResult<T> {
    match val.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_) => Err(GtfsError::ParseFloat(line, String::from(file), String::from(val))),
    }
}


/// Takes a &str containing an arrival/departure time for gtfs and returns
/// a naivetime. Chrono's NaiveTime parser is relatively slow and doesn't
/// account for the optional leading zeros in the hour part.
pub fn parse_timeoffset(line: usize, file: &str, val: &str) -> GtfsResult<TimeOffset> {
    let mut parts = val.trim().split(':');
    let parse_part = |part: Option<&str>| -> GtfsResult<u32> {
        match part {
            Some(val) => Ok(try!(parse_int(line, file, val))),
            None => Err(GtfsError::ParseTime(line, String::from(file), String::from(val))),
        }
    };
    let hours = try!(parse_part(parts.next()));
    let minutes = try!(parse_part(parts.next()));
    let seconds = try!(parse_part(parts.next()));
    Ok(TimeOffset::from_hms(hours, minutes, seconds))
}

/// Takes a &str containing an stop time pickup type for gtfs and returns
/// a `PickupType` enum.
pub fn parse_pickup_type(line: usize, file: &str, val: &str) -> GtfsResult<PickupType> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(PickupType::RegularlyScheduled),
        "1" => Ok(PickupType::NoPickupAvailable),
        "2" => Ok(PickupType::MustPhoneAgency),
        "3" => Ok(PickupType::MustCoordinateWithDriver),
        _ => Err(GtfsError::ParsePickupType(line, String::from(file), String::from(val))),
    }
}

/// Takes a &str containing an stop time dropoff type for gtfs and returns
/// a `DropoffType` enum.
pub fn parse_dropoff_type(line: usize, file: &str, val: &str) -> GtfsResult<DropoffType> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(DropoffType::RegularlyScheduled),
        "1" => Ok(DropoffType::NoDropoffAvailable),
        "2" => Ok(DropoffType::MustPhoneAgency),
        "3" => Ok(DropoffType::MustCoordinateWithDriver),
        _ => Err(GtfsError::ParseDropoffType(line, String::from(file), String::from(val))),
    }
}

/// Takes a &str containing an location type for gtfs and returns
/// a `LocationType` enum.
pub fn parse_location_type(line: usize, file: &str, val: &str) -> GtfsResult<LocationType> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(LocationType::Stop),
        "1" => Ok(LocationType::Station),
        _ => Err(GtfsError::ParseLocationType(line, String::from(file), String::from(val))),
    }
}

/// Takes a &str containing wheelchair boarding information for gtfs and returns
/// a `WheelchairBoarding` enum.
pub fn parse_wheelchair_boarding(line: usize, file: &str, val: &str) -> GtfsResult<WheelchairBoarding> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(WheelchairBoarding::NoInformation),
        "1" => Ok(WheelchairBoarding::SomeAccessibility),
        "2" => Ok(WheelchairBoarding::NoAccessibility),
        _ => Err(GtfsError::ParseWheelchairBoarding(line, String::from(file), String::from(val))),
    }
}

/// Takes a &str containing route type information for gtfs and returns a
/// `RouteType` enum.
pub fn parse_route_type(line: usize, file: &str, val: &str) -> GtfsResult<RouteType> {
    let trimmed = val.trim();
    match trimmed {
        _ => Err(GtfsError::ParseRouteType(line, String::from(file), String::from(val))),
    }
}

macro_rules! parse_try {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        }
    }
}

#[test]
fn parse_timeoffset_test() {
    assert_eq!(parse_timeoffset(0, "bogus", "01:01:01").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert_eq!(parse_timeoffset(0, "bogus", "1:01:01").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert_eq!(parse_timeoffset(0, "bogus", "01:01:01  ").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert_eq!(parse_timeoffset(0, "bogus", " 01:01:01  ").unwrap(), TimeOffset::from_hms(1, 1, 1));
    assert!(parse_timeoffset(0, "bogus", ":01:01").is_err());
    assert!(parse_timeoffset(0, "bogus", "ab:01:01").is_err());
    assert!(parse_timeoffset(0, "bogus", "01::01").is_err());
}
