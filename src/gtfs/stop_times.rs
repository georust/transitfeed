use std::io::BufRead;
use quick_csv::Csv;
use transit::{StopTime, DropoffType, PickupType, Timepoint, TimeOffset};
use gtfs::parse::{parse_timeoffset, parse_float, parse_pickup_type, parse_dropoff_type, parse_int};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `StopTime` struct from
/// a CSV iterator.
pub struct StopTimeIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> StopTimeIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<StopTimeIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("stop_times.txt")))
        } else {
            Ok(StopTimeIterator {
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for StopTimeIterator<B> {
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


