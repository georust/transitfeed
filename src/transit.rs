use chrono::{Duration, NaiveDate};
use gtfs::parse::*;
use serde;

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

impl Default for LocationType {
    fn default() -> Self {
        LocationType::Stop
    }
}

impl<'de> serde::Deserialize<'de> for LocationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}
/// Wheelchair Boarding
#[derive(Debug, PartialEq)]
pub enum WheelchairBoarding {
    NoInformation,
    SomeAccessibility,
    NoAccessibility,
}

impl Default for WheelchairBoarding {
    fn default() -> Self {
        WheelchairBoarding::NoInformation
    }
}

impl<'de> serde::Deserialize<'de> for WheelchairBoarding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
    #[serde(default)]
    pub location_type: LocationType,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    #[serde(default)]
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

impl<'de> serde::Deserialize<'de> for RouteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// Route
#[derive(Debug, Deserialize)]
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
// TODO: merge with WheelchairBoarding
#[derive(Debug)]
pub enum WheelchairAccessible {
    NoInformation,
    SomeAccessibility,
    NoAccessibility,
}

impl Default for WheelchairAccessible {
    fn default() -> Self {
        WheelchairAccessible::NoInformation
    }
}

impl<'de> serde::Deserialize<'de> for WheelchairAccessible {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// Bikes Allowed
#[derive(Debug)]
pub enum BikesAllowed {
    NoInformation,
    SomeBikes,
    NoBikes,
}

impl Default for BikesAllowed {
    fn default() -> Self {
        BikesAllowed::NoInformation
    }
}

impl<'de> serde::Deserialize<'de> for BikesAllowed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
    #[serde(default)]
    pub wheelchair_accessible: WheelchairAccessible,
    #[serde(default)]
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

impl Default for PickupType {
    fn default() -> Self {
        PickupType::RegularlyScheduled
    }
}

impl<'de> serde::Deserialize<'de> for PickupType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// DropoffType for `StopTime`
#[derive(Debug, PartialEq)]
pub enum DropoffType {
    RegularlyScheduled,
    NoDropoffAvailable,
    MustPhoneAgency,
    MustCoordinateWithDriver,
}

impl Default for DropoffType {
    fn default() -> Self {
        DropoffType::RegularlyScheduled
    }
}

impl<'de> serde::Deserialize<'de> for DropoffType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}
/// Timepoint for `StopTime`
#[derive(Debug, PartialEq)]
pub enum Timepoint {
    Approximate,
    Exact,
}

impl Default for Timepoint {
    fn default() -> Self {
        Timepoint::Exact
    }
}

impl<'de> serde::Deserialize<'de> for Timepoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// StopTime
#[derive(Debug, Deserialize, PartialEq)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: TimeOffset,
    pub departure_time: TimeOffset,
    pub stop_id: String,
    pub stop_sequence: u64,
    pub stop_headsign: Option<String>,
    #[serde(default)]
    pub pickup_type: PickupType,
    #[serde(default)]
    pub dropoff_type: DropoffType,
    pub shape_dist_traveled: Option<f64>,
    #[serde(default)]
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

impl<'de> serde::Deserialize<'de> for ExceptionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
        match result {
            1 => Ok(ExceptionType::ServiceAdded),
            2 => Ok(ExceptionType::ServiceRemoved),
            _ => Err(serde::de::Error::custom("Exception type field was not 1 or 2"))
        }
    }
}

/// CalendarDate
#[derive(Debug, Deserialize)]
pub struct CalendarDate {
    pub service_id: String,
    #[serde(deserialize_with = "deserialize_calendardate")]
    pub date: NaiveDate,
    pub exception_type: ExceptionType
}

/// PaymentMethod for `FareAttribute`
#[derive(Debug)]
pub enum PaymentMethod {
    PaidOnboard,
    PaidBefore,
}

impl<'de> serde::Deserialize<'de> for PaymentMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let result : u32 = try!(serde::Deserialize::deserialize(deserializer));
        match result {
            0 => Ok(PaymentMethod::PaidOnboard),
            1 => Ok(PaymentMethod::PaidBefore),
            _ => Err(serde::de::Error::custom("payment method must be 0 or 1"))
        }
    }
}

/// Tranfers for `FareAttribute`
#[derive(Debug)]
pub enum Transfers {
    None,
    TransferOnce,
    TransferTwice,
    Unlimited,
}

impl<'de> serde::Deserialize<'de> for Transfers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// FareAttribute
#[derive(Debug, Deserialize)]
pub struct FareAttribute {
    pub fare_id: String,
    pub price: f64,
    pub currency_type: String,
    pub payment_method: PaymentMethod,
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
    pub shape_dist_traveled: Option<f64>,
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

impl<'de> serde::Deserialize<'de> for TimeOffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// exact_times for Frequency
#[derive(Debug)]
pub enum FrequencyAccuracy {
    Approximate,
    Exact
}

impl Default for FrequencyAccuracy {
    fn default() -> Self {
        FrequencyAccuracy::Approximate
    }
}

impl<'de> serde::Deserialize<'de> for FrequencyAccuracy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// Frequency
#[derive(Debug, Deserialize)]
pub struct Frequency {
    pub trip_id: String,
    pub start_time: TimeOffset,
    pub end_time: TimeOffset,
    pub headway_secs: u64,
    #[serde(default)]
    pub exact_times: FrequencyAccuracy,
}

#[derive(Debug)]
pub enum TransferType {
    Recommended,
    Timed,
    MinimumTime,
    NotPossible
}

impl<'de> serde::Deserialize<'de> for TransferType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
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
}

/// Transfer
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub from_stop_id: String,
    pub to_stop_id: String,
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
