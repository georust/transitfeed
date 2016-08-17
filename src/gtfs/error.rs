use std::error;
use std::fmt;
use quick_csv::error::{Error as CsvError};


#[derive(Debug)]
pub enum GtfsError {
    Csv(CsvError),
    CsvHeader(String),
    ParseInt(usize, String, String),
    ParseFloat(usize, String, String),
    ParseTime(usize, String, String),
    ParseLocationType(usize, String, String),
    ParseWheelchairBoarding(usize, String, String),
    ParsePickupType(usize, String, String),
    ParseDropoffType(usize, String, String),
    ParseRouteType(usize, String, String),
    ParseWheelchairAccessible(usize, String, String),
    ParseBikesAllowed(usize, String, String),
}

impl fmt::Display for GtfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GtfsError")
    }
}

impl error::Error for GtfsError {
    fn description(&self) -> &str {
       "gtfs bad stuff happened"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// Type alias for Gtfs Results
pub type GtfsResult<T> = Result<T, GtfsError>;
