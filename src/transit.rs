use std::error::Error;
use chrono::{Duration, NaiveDate};
use gtfs::parse::*;

/// Transit trait defines methods for iterating over components of a Transit
/// system
pub trait Transit<'a, E: Error> {
    type AgencyIterator: Iterator<Item=Result<Agency, E>>;

    fn agencies(&'a self) -> Self::AgencyIterator;
}

/// Agency
#[derive(Debug, Deserialize)]
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
#[derive(Debug, PartialEq)]
pub enum LocationType {
    Stop,
    Station,
}

fn default_locationtype() -> LocationType {
    LocationType::Stop
}

/// Wheelchair Boarding
#[derive(Debug, PartialEq)]
pub enum WheelchairBoarding {
    NoInformation,
    SomeAccessibility,
    NoAccessibility,
}

fn default_wheelchairboarding() -> WheelchairBoarding {
    WheelchairBoarding::NoInformation
}

/// Stop
#[derive(Debug, Deserialize, PartialEq)]
pub struct Stop {
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: String,
    pub stop_desc: Option<String>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    #[serde(default = "default_locationtype", deserialize_with = "deserialize_locationtype")]
    pub location_type: LocationType,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    #[serde(default = "default_wheelchairboarding", deserialize_with = "deserialize_wheelchairboarding")]
    pub wheelchair_boarding: WheelchairBoarding,
}

/// RouteType
#[derive(Debug)]
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
#[derive(Debug, Deserialize)]
pub struct Route {
    pub route_id: String,
    pub agency_id: Option<String>,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: Option<String>,
    #[serde(deserialize_with ="deserialize_routetype")]
    pub route_type: RouteType,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
}

/// Wheelchair Accessible
// TODO: merge with WheelchairBoarding
#[derive(Debug)]
pub enum WheelchairAccessible {
    NoInformation,
    SomeAccessibility,
    NoAccessibility,
}

fn default_wheelchairaccessible() -> WheelchairAccessible {
    WheelchairAccessible::NoInformation
}

/// Bikes Allowed
#[derive(Debug)]
pub enum BikesAllowed {
    NoInformation,
    SomeBikes,
    NoBikes,
}

fn default_bikesallowed() -> BikesAllowed {
    BikesAllowed::NoInformation
}

/// Trip
#[derive(Debug, Deserialize)]
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: Option<String>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    #[serde(default = "default_wheelchairaccessible", deserialize_with = "deserialize_wheelchairaccessible")]
    pub wheelchair_accessible: WheelchairAccessible,
    #[serde(default = "default_bikesallowed", deserialize_with = "deserialize_bikesallowed")]
    pub bikes_allowed: BikesAllowed,
}

/// PickupType for `StopTime`
#[derive(Debug, PartialEq)]
pub enum PickupType {
    RegularlyScheduled,
    NoPickupAvailable,
    MustPhoneAgency,
    MustCoordinateWithDriver,
}

fn default_pickuptype() -> PickupType {
    PickupType::RegularlyScheduled
}

/// DropoffType for `StopTime`
#[derive(Debug, PartialEq)]
pub enum DropoffType {
    RegularlyScheduled,
    NoDropoffAvailable,
    MustPhoneAgency,
    MustCoordinateWithDriver,
}

fn default_dropofftype() -> DropoffType {
    DropoffType::RegularlyScheduled
}

/// Timepoint for `StopTime`
#[derive(Debug, PartialEq)]
pub enum Timepoint {
    Approximate,
    Exact,
}

fn default_timepoint() -> Timepoint {
    Timepoint::Exact
}

/// StopTime
#[derive(Debug, Deserialize, PartialEq)]
pub struct StopTime {
    pub trip_id: String,
    #[serde(deserialize_with = "deserialize_timeoffset")]
    pub arrival_time: TimeOffset,
    #[serde(deserialize_with = "deserialize_timeoffset")]
    pub departure_time: TimeOffset,
    pub stop_id: String,
    pub stop_sequence: u64,
    pub stop_headsign: Option<String>,
    #[serde(default = "default_pickuptype", deserialize_with = "deserialize_pickuptype")]
    pub pickup_type: PickupType,
    #[serde(default = "default_dropofftype", deserialize_with = "deserialize_dropofftype")]
    pub dropoff_type: DropoffType,
    pub shape_dist_traveled: Option<f64>,
    #[serde(default = "default_timepoint", deserialize_with = "deserialize_timepoint")]
    pub timepoint: Timepoint,
}

/// Calendar
#[derive(Debug, Deserialize)]
pub struct Calendar {
    pub service_id: String,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub monday: bool,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub tuesday: bool,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub wednesday: bool,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub thursday: bool,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub friday: bool,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub saturday: bool,
    #[serde(deserialize_with = "deserialize_dow_field")]
    pub sunday: bool,
    #[serde(deserialize_with = "deserialize_calendardate")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_calendardate")]
    pub end_date: NaiveDate,
}

/// ExceptionType for `CalendarDate`
#[derive(Debug)]
pub enum ExceptionType {
    ServiceAdded,
    ServiceRemoved,
}

/// CalendarDate
#[derive(Debug, Deserialize)]
pub struct CalendarDate {
    pub service_id: String,
    #[serde(deserialize_with = "deserialize_calendardate")]
    pub date: NaiveDate,
    #[serde(deserialize_with = "deserialize_exceptiontype")]
    pub exception_type: ExceptionType
}

/// PaymentMethod for `FareAttribute`
#[derive(Debug)]
pub enum PaymentMethod {
    PaidOnboard,
    PaidBefore,
}

/// Tranfers for `FareAttribute`
#[derive(Debug)]
pub enum Transfers {
    None,
    TransferOnce,
    TransferTwice,
    Unlimited,
}

/// FareAttribute
#[derive(Debug, Deserialize)]
pub struct FareAttribute {
    pub fare_id: String,
    pub price: f64,
    pub currency_type: String,
    #[serde(deserialize_with = "deserialize_paymentmethod")]
    pub payment_method: PaymentMethod,
    #[serde(deserialize_with = "deserialize_transfers")]
    pub transfers: Transfers,
    #[serde(deserialize_with = "deserialize_transferduration")]
    pub transfer_duration: Option<Duration>,
}

/// FareRule
/// origin, destination, and contains reference a zone_id from stops
#[derive(Debug, Deserialize)]
pub struct FareRule {
    pub fare_id: String,
    pub route_id: Option<String>,
    pub origin_id: Option<String>,
    pub destination_id: Option<String>,
    pub contains_id: Option<String>,
}

/// Shape
#[derive(Debug, Deserialize)]
pub struct Shape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: u64,
    pub shape_dist_traveled: f64,
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

/// exact_times for Frequency
#[derive(Debug)]
pub enum FrequencyAccuracy {
    Approximate,
    Exact
}

fn default_frequencyaccuracy() -> FrequencyAccuracy {
    return FrequencyAccuracy::Approximate;
}

/// Frequency
#[derive(Debug, Deserialize)]
pub struct Frequency {
    pub trip_id: String,
    #[serde(deserialize_with = "deserialize_timeoffset")]
    pub start_time: TimeOffset,
    #[serde(deserialize_with = "deserialize_timeoffset")]
    pub end_time: TimeOffset,
    pub headway_secs: u64,
    #[serde(default = "default_frequencyaccuracy", deserialize_with = "deserialize_frequencyaccuracy")]
    pub exact_times: FrequencyAccuracy,
}

#[derive(Debug)]
pub enum TransferType {
    Recommended,
    Timed,
    MinimumTime,
    NotPossible
}

/// Transfer
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub from_stop_id: String,
    pub to_stop_id: String,
    #[serde(deserialize_with = "deserialize_transfertype")]
    pub transfer_type: TransferType,
    #[serde(deserialize_with = "deserialize_transferduration")]
    pub min_transfer_time: Option<Duration>
}


/// Feed Info
#[derive(Debug, Deserialize)]
pub struct FeedInfo {
    pub feed_publisher_name: String,
    pub feed_publisher_url: String,
    pub feed_lang: String,
    #[serde(default = "default_feed_date", deserialize_with = "deserialize_option_calendardate")]
    pub feed_start_date: Option<NaiveDate>,
    #[serde(default = "default_feed_date", deserialize_with = "deserialize_option_calendardate")]
    pub feed_end_date: Option<NaiveDate>,
    pub feed_version: Option<String>
}

fn default_feed_date() -> Option<NaiveDate> {
    None
}
