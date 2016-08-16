use std::io::BufRead;
use quick_csv::Csv;
use transit::{Stop, WheelchairBoarding, LocationType};

use gtfs::parse::{parse_float, parse_location_type, parse_wheelchair_boarding};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Stop` struct from
/// a CSV iterator.
pub struct StopIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> StopIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<StopIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("stops.txt")))
        } else {
            Ok(StopIterator {
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for StopIterator<B> {
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




