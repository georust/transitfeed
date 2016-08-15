use std::str::FromStr;
use std::io::BufRead;
use quick_csv::Csv;
use transit::{Agency, LocationType, WheelchairBoarding, Stop, PickupType, DropoffType, Timepoint, StopTime, TimeOffset};
use error::GtfsError;

/// Type alias for Gtfs Results
pub type GtfsResult<T> = Result<T, GtfsError>;

/// Parse an Integer with line and file numbers given for error reporting
fn parse_int<T: FromStr>(line: usize, file: &str, val: &str) -> GtfsResult<T> {
    match val.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_) => Err(GtfsError::ParseInt(line, String::from(file), String::from(val))),
    }
}

/// Parse a Float with line and file numbers given for error reporting
fn parse_float<T: FromStr>(line: usize, file: &str, val: &str) -> GtfsResult<T> {
    match val.parse::<T>() {
        Ok(n) => Ok(n),
        Err(_) => Err(GtfsError::ParseFloat(line, String::from(file), String::from(val))),
    }
}


/// Takes a &str containing an arrival/departure time for gtfs and returns
/// a naivetime. Chrono's NaiveTime parser is relatively slow and doesn't
/// account for the optional leading zeros in the hour part.
fn parse_timeoffset(line: usize, file: &str, val: &str) -> GtfsResult<TimeOffset> {
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
fn parse_pickup_type(line: usize, file: &str, val: &str) -> GtfsResult<PickupType> {
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
fn parse_dropoff_type(line: usize, file: &str, val: &str) -> GtfsResult<DropoffType> {
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
fn parse_location_type(line: usize, file: &str, val: &str) -> GtfsResult<LocationType> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(LocationType::Stop),
        "1" => Ok(LocationType::Station),
        _ => Err(GtfsError::ParseLocationType(line, String::from(file), String::from(val))),
    }
}

/// Takes a &str containing wheelchair boarding information for gtfs and returns
/// a `WheelchairBoarding` enum.
fn parse_wheelchair_boarding(line: usize, file: &str, val: &str) -> GtfsResult<WheelchairBoarding> {
    let trimmed = val.trim();
    match trimmed {
        "0" => Ok(WheelchairBoarding::NoInformation),
        "1" => Ok(WheelchairBoarding::SomeAccessibility),
        "2" => Ok(WheelchairBoarding::NoAccessibility),
        _ => Err(GtfsError::ParseWheelchairBoarding(line, String::from(file), String::from(val))),
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

/// An decoder which returns as its iterator output a new `Stop` struct from
/// a CSV iterator.
pub struct AgencyDecoder<B: BufRead> {
    transit_id: i64,
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> AgencyDecoder<B> {
    pub fn new(transit_id: i64, csv: Csv<B>) -> GtfsResult<AgencyDecoder<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("stops.txt")))
        } else {
            Ok(AgencyDecoder {
                transit_id: transit_id,
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for AgencyDecoder<B> {
    type Item = GtfsResult<Agency>;

    fn next(&mut self) -> Option<GtfsResult<Agency>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        //let filename = "agency.txt";
                        let mut agency_id = None;
                        let mut agency_name = String::new();
                        let mut agency_url = String::new();
                        let mut agency_timezone = String::new();
                        let mut agency_lang = None;
                        let mut agency_phone = None;
                        let mut agency_fare_url = None;
                        let mut agency_email = None;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "agency_id" => { agency_id = Some(String::from(column)); },
                                "agency_name" => { agency_name = String::from(column); },
                                "agency_url" => { agency_url= String::from(column); },
                                "agency_timezone" => { agency_timezone = String::from(column); },
                                "agency_lang" => { agency_lang = Some(String::from(column)); },
                                "agency_phone" => { agency_phone = Some(String::from(column)); },
                                "agency_fare_url" => { agency_fare_url = Some(String::from(column)); },
                                "agency_email" => { agency_email = Some(String::from(column)); },
                                _ => (),
                            }
                        }
                        let agency = Agency {
                            transit_id: self.transit_id,
                            agency_id: agency_id,
                            agency_name: agency_name,
                            agency_url: agency_url,
                            agency_timezone: agency_timezone,
                            agency_lang: agency_lang,
                            agency_phone: agency_phone,
                            agency_fare_url: agency_fare_url,
                            agency_email: agency_email,
                        };
                        self.line += 1;
                        //println!("{:?}", stop_time);
                        Some(Ok(agency))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}


/// An decoder which returns as its iterator output a new `Stop` struct from
/// a CSV iterator.
pub struct StopDecoder<B: BufRead> {
    transit_id: i64,
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> StopDecoder<B> {
    pub fn new(transit_id: i64, csv: Csv<B>) -> GtfsResult<StopDecoder<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("stops.txt")))
        } else {
            Ok(StopDecoder {
                transit_id: transit_id,
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for StopDecoder<B> {
    type Item = GtfsResult<Stop>;

    fn next(&mut self) -> Option<GtfsResult<Stop>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "stops.txt";
                        let mut stop_id = String::new();
                        let mut stop_code = None;
                        let mut stop_name = String::new();
                        let mut stop_desc = None;
                        let mut stop_lat = 0.0;
                        let mut stop_lon = 0.0;
                        let mut zone_id = None;
                        let mut stop_url = None;
                        let mut location_type = LocationType::Stop;
                        let mut parent_station = None;
                        let mut stop_timezone = None;
                        let mut wheelchair_boarding = WheelchairBoarding::NoInformation;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "stop_id" => { stop_id = String::from(column); },
                                "stop_code" => { stop_code = Some(String::from(column)); },
                                "stop_name" => { stop_name = String::from(column); },
                                "stop_desc" => { stop_desc = Some(String::from(column)); },
                                "stop_lat" => { stop_lat = parse_try!(parse_float(self.line, filename, column)); },
                                "stop_lon" => { stop_lon = parse_try!(parse_float(self.line, filename, column)); },
                                "zone_id" => { zone_id = Some(String::from(column)); },
                                "stop_url" => { stop_url = Some(String::from(column)); },
                                "location_type" => { location_type = parse_try!(parse_location_type(self.line, filename, column)); },
                                "parent_station" => { parent_station= Some(String::from(column)); },
                                "stop_timezone" => { stop_timezone = Some(String::from(column)); },
                                "wheelchair_boarding" => { wheelchair_boarding = parse_try!(parse_wheelchair_boarding(self.line, filename, column)); },
                                _ => (),
                            }
                        }
                        let stop = Stop {
                            transit_id: self.transit_id,
                            stop_id: stop_id,
                            stop_code: stop_code,
                            stop_name: stop_name,
                            stop_desc: stop_desc,
                            stop_lat: stop_lat,
                            stop_lon: stop_lon,
                            zone_id: zone_id,
                            stop_url: stop_url,
                            location_type: location_type,
                            parent_station: parent_station,
                            stop_timezone: stop_timezone,
                            wheelchair_boarding: wheelchair_boarding,
                        };
                        self.line += 1;
                        //println!("{:?}", stop_time);
                        Some(Ok(stop))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}



/// An decoder which returns as its iterator output a new `StopTime` struct from
/// a CSV iterator.
pub struct StopTimeDecoder<B: BufRead> {
    transit_id: i64,
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> StopTimeDecoder<B> {
    pub fn new(transit_id: i64, csv: Csv<B>) -> GtfsResult<StopTimeDecoder<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("stop_times.txt")))
        } else {
            Ok(StopTimeDecoder {
                transit_id: transit_id,
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for StopTimeDecoder<B> {
    type Item = GtfsResult<StopTime>;

    fn next(&mut self) -> Option<GtfsResult<StopTime>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let mut trip_id = String::new();
                        let mut departure_time = TimeOffset::from_hms(0, 0, 0);
                        let mut arrival_time = TimeOffset::from_hms(0, 0, 0);
                        let mut stop_id = String::new();
                        let mut stop_sequence = 0;
                        let mut stop_headsign = None;
                        let mut pickup_type = PickupType::RegularlyScheduled;
                        let mut dropoff_type = DropoffType::RegularlyScheduled;
                        let mut shape_dist_traveled = None;
                        let timepoint = Timepoint::Exact;
                        let filename = "stop_times.txt";
                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "trip_id" => { trip_id = String::from(column); },
                                "departure_time" => { departure_time = parse_try!(parse_timeoffset(self.line, &filename, column)) },
                                "arrival_time" => { arrival_time = parse_try!(parse_timeoffset(self.line, &filename, column)) },
                                "stop_id" => { stop_id = String::from(column); },
                                "stop_sequence" => { stop_sequence = parse_try!(parse_int(self.line, &filename, column)); },
                                "stop_headsign" => { stop_headsign = Some(String::from(column)); },
                                "pickup_type" => { pickup_type = parse_try!(parse_pickup_type(self.line, &filename, column)); },
                                "dropoff_type" => { dropoff_type = parse_try!(parse_dropoff_type(self.line, &filename, column)); },
                                "shape_dist_traveled" => { shape_dist_traveled = Some(parse_try!(parse_float(self.line, &filename, column))); },
                                _ => (),
                            }
                        }
                        let stop_time = StopTime {
                            transit_id: self.transit_id,
                            trip_id: trip_id,
                            departure_time: departure_time,
                            arrival_time: arrival_time,
                            stop_id: stop_id,
                            stop_sequence: stop_sequence,
                            stop_headsign: stop_headsign,
                            pickup_type: pickup_type,
                            dropoff_type: dropoff_type,
                            shape_dist_traveled: shape_dist_traveled,
                            timepoint: timepoint,
                        };
                        self.line += 1;
                        //println!("{:?}", stop_time);
                        Some(Ok(stop_time))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
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
