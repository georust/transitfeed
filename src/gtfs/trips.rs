use std::io::BufRead;
use quick_csv::Csv;
use transit::{Trip, BikesAllowed, WheelchairAccessible};

use gtfs::parse::{parse_wheelchair_accessible, parse_bikes_allowed};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Trip` struct from
/// a CSV iterator.
pub struct TripIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> TripIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<TripIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("trips.txt")))
        } else {
            Ok(TripIterator {
                csv: csv,
                headers: headers,
                line: 0,
            })
        }
    }
}

impl<B: BufRead> Iterator for TripIterator<B> {
    type Item = GtfsResult<Trip>;

    fn next(&mut self) -> Option<GtfsResult<Trip>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "trips.txt";
                        let mut route_id = String::new();
                        let mut service_id = String::new();
                        let mut trip_id = String::new();
                        let mut trip_headsign = None;
                        let mut trip_short_name = None;
                        let mut direction_id = None;
                        let mut block_id = None;
                        let mut shape_id = None;
                        let mut wheelchair_accessible = WheelchairAccessible::NoInformation;
                        let mut bikes_allowed = BikesAllowed::NoInformation;
                        
                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "route_id" => { route_id = String::from(column); },
                                "service_id" => { service_id = String::from(column); },
                                "trip_id" => { trip_id = String::from(column); },
                                "trip_headsign" => { trip_headsign = Some(String::from(column)); },
                                "trip_short_name" => { trip_short_name = Some(String::from(column)); },
                                "direction_id" => { direction_id = Some(String::from(column)); },
                                "block_id" => { block_id = Some(String::from(column)); },
                                "shape_id" => { shape_id = Some(String::from(column)); },
                                "wheelchair_accessible" => { wheelchair_accessible = parse_try!(parse_wheelchair_accessible(self.line, filename, column)); },
                                "bikes_allowed" => { bikes_allowed = parse_try!(parse_bikes_allowed(self.line, filename, column)); },
                                _ => (),
                            }
                        }
                        let trip = Trip {
                            route_id: route_id,
                            service_id: service_id,
                            trip_id: trip_id,
                            trip_headsign: trip_headsign,
                            trip_short_name: trip_short_name,
                            direction_id: direction_id,
                            block_id: block_id,
                            shape_id: shape_id,
                            wheelchair_accessible: wheelchair_accessible,
                            bikes_allowed: bikes_allowed,
                        };
                        self.line += 1;
                        Some(Ok(trip))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}




