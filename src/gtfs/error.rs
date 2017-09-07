use std::error;
use std::fmt;
use quick_csv::error::{Error as CsvError};
use std::error::Error;


#[derive(Debug)]
pub enum GtfsError {
    Csv(CsvError, String, usize),
    CsvHeader(String),
    LineParseError(ParseError, String, usize),
}

impl fmt::Display for GtfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GtfsError::Csv(_, ref file, ref line) => write!(f, "GtfsError: {} ({}:{}) - {:?}", self.description(), file, line, self.cause()),
            GtfsError::CsvHeader(ref file) => write!(f, "GtfsError: {}, ({})", self.description(), file),
            GtfsError::LineParseError(_, ref file, ref line) => write!(f, "GtfsError: {} ({}:{}) - {:?}", self.description(), file, line, self.cause()),
        }
    }
}

impl error::Error for GtfsError {
    fn description(&self) -> &str {
        match *self {
            GtfsError::Csv(ref err, _, _) => err.description(),
            GtfsError::CsvHeader(_) => "error reading headers",
            GtfsError::LineParseError(_, _, _) => "error reading line",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GtfsError::Csv(ref err, _, _) => Some(err),
            GtfsError::CsvHeader(_) => None,
            GtfsError::LineParseError(ref err, _, _) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    ParseInt(String),
    ParseFloat(String),
    ParseDate(String),
    ParseTime(String),
    ParseLocationType(String),
    ParseWheelchairBoarding(String),
    ParseExactTimes(String),
    ParsePickupType(String),
    ParseDropoffType(String),
    ParseRouteType(String),
    ParseWheelchairAccessible(String),
    ParseBikesAllowed(String),
}

// TODO: This is ugly as hell, also look into improving ParseError.description too
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::ParseInt(ref value) | ParseError::ParseFloat(ref value) | ParseError::ParseDate(ref value) | ParseError::ParseTime(ref value) | ParseError::ParseLocationType(ref value) | ParseError::ParseWheelchairBoarding(ref value) | ParseError::ParseExactTimes(ref value) | ParseError::ParsePickupType(ref value) | ParseError::ParseDropoffType(ref value) | ParseError::ParseRouteType(ref value) | ParseError::ParseWheelchairAccessible(ref value) | ParseError::ParseBikesAllowed(ref value) => write!(f, "ParseError: {} - '{}'", self.description(), value),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::ParseInt(_) => "error parsing int",
            ParseError::ParseFloat(_) => "error parsing float",
            ParseError::ParseDate(_) => "error parsing date",
            ParseError::ParseTime(_) => "error parsing time",
            ParseError::ParseLocationType(_) => "error parsing location_type",
            ParseError::ParseWheelchairBoarding(_) => "error parsing wheelchair_boarding",
            ParseError::ParseExactTimes(_) => "error parsing exact_times",
            ParseError::ParsePickupType(_) => "error parsing pickup_type",
            ParseError::ParseDropoffType(_) => "error parsing dropoff_type",
            ParseError::ParseRouteType(_) => "error parsing route_type",
            ParseError::ParseWheelchairAccessible(_) => "error parsing wheelchair_accessible",
            ParseError::ParseBikesAllowed(_) => "error parsing bikes_allowed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
