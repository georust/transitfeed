use std::error::Error;
use chrono::{Duration, NaiveDate};

/// Transit trait defines methods for iterating over components of a Transit
/// system
pub trait Transit<E: Error, AgencyIterator: Iterator<Item=Result<Agency, E>>> {
    fn agencies() -> AgencyIterator;
}

/// Agency
pub struct Agency {
    pub agency_id: Option<String>,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
    pub agency_fare_url: Option<String>,
    pub agency_email: Option<String>,
}

/// Location Type
pub enum LocationType {
    Stop,
    Station,
}

/// Wheelchair Boarding
pub enum WheelchairBoarding {
    NoInformation,
    SomeAccessibility,
    NoAccessibility,
}

/// Stop
pub struct Stop {
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: String,
    pub stop_desc: Option<String>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: LocationType,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: WheelchairBoarding,
}

/// RouteType
pub enum RouteType {
    LightRail,
    Subway,
    Rail,
    Bus,
    Ferry,
    CableCar,
    Gondola,
    Funicular,
}

/// Route
pub struct Route {
    pub route_id: String,
    pub agency_id: Option<String>,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: Option<String>,
    pub route_type: RouteType,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
}

/// Wheelchair Accessible
pub enum WheelchairAccessible {
    NoInformation,
    SomeAccessibility,
    NoAccessibility,
}

/// Bikes Allowed
pub enum BikesAllowed {
    NoInformation,
    SomeBikes,
    NoBikes,
}

/// Trip
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: Option<String>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: WheelchairAccessible,
    pub bikes_allowed: BikesAllowed,
}

/// PickupType for `StopTime`
#[derive(Debug)]
pub enum PickupType {
    RegularlyScheduled,
    NoPickupAvailable,
    MustPhoneAgency,
    MustCoordinateWithDriver,
}

/// DropoffType for `StopTime`
#[derive(Debug)]
pub enum DropoffType {
    RegularlyScheduled,
    NoDropoffAvailable,
    MustPhoneAgency,
    MustCoordinateWithDriver,
}

/// Timepoint for `StopTime`
#[derive(Debug)]
pub enum Timepoint {
    Approximate,
    Exact,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TimeOffset {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl TimeOffset {
    pub fn from_hms(hours: u32, minutes: u32, seconds: u32) -> TimeOffset {
        TimeOffset {
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::hours(self.hours as i64)
            + Duration::minutes(self.minutes as i64)
            + Duration::seconds(self.seconds as i64)
    }
}


/// StopTime
#[derive(Debug)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: TimeOffset,
    pub departure_time: TimeOffset,
    pub stop_id: String,
    pub stop_sequence: u64,
    pub stop_headsign: Option<String>,
    pub pickup_type: PickupType,
    pub dropoff_type: DropoffType,
    pub shape_dist_traveled: Option<f64>,
    pub timepoint: Timepoint,
}

/// Calendar
pub struct Calendar {
    pub service_id: String,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

/// ExceptionType for `CalendarDate`
pub enum ExceptionType {
    ServiceAdded,
    ServiceRemoved,
}

/// CalendarDate
pub struct CalendarDate {
    pub service_id: String,
    pub date: NaiveDate,
    pub exception_type: ExceptionType
}

/// PaymentMethod for `FareAttribute`
pub enum PaymentMethod {
    PaidOnboard,
    PaidBefore,
}

/// Tranfers for `FareAttribute`
pub enum Transfers {
    None,
    TransferOnce,
    TransferTwice,
    Unlimited,
}


/// FareAttribute
pub struct FareAttribute {
    pub fare_id: String,
    pub price: f64,
    pub currency_type: String,
    pub payment_method: PaymentMethod,
    pub transfers: Transfers,
    pub transfer_duration: Duration,
}

/// FareRule
/// origin, destination, and contains reference a zone_id from stops
pub struct FareRule {
    pub fare_id: String,
    pub route_id: Option<String>,
    pub origin_id: Option<String>,
    pub destination_id: Option<String>,
    pub contains_id: Option<String>,
}

/// Shape
pub struct Shape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: u64,
    pub shape_dist_traveled: f64,
}
