mod error;
#[macro_use]
mod parse;
mod gtfs;
pub mod agencies;
pub mod calendars;
pub mod calendar_dates;
pub mod routes;
pub mod shapes;
pub mod stops;
pub mod stop_times;
pub mod trips;
pub mod frequencies;

pub use gtfs::gtfs::GTFS;

use std::io::BufRead;
use std::slice::Iter;
use std::iter::Zip;
use std::collections::HashMap;
use quick_csv::Csv;
use quick_csv::columns::Columns;
use gtfs::error::{ParseError, GtfsError};

pub struct GTFSIterator<B, F, T>
    where B: BufRead,
          F: (Fn(Zip<Iter<String>, Columns>) -> Result<T, ParseError>)
{
    csv: Csv<B>,
    filename: String,
    header: Vec<String>,
    line: usize,
    parser: F,
}

impl<B, F, T> GTFSIterator<B, F, T>
    where B: BufRead,
          F: (Fn(Zip<Iter<String>, Columns>) -> Result<T, ParseError>)

{
    pub fn new(csv: Csv<B>, filename: String, parser: F) -> Result<GTFSIterator<B, F, T>, GtfsError> {
        let mut csv = csv.has_header(true);
        let header = csv.headers();
        if header.len() == 0 {
            Err(GtfsError::CsvHeader(filename))
        } else {
            Ok(GTFSIterator {
                csv: csv,
                parser: parser,
                header: header,
                filename: filename,
                line: 1,
            })
        }
    }

//    pub fn from_path(path: String, filename: &str) -> Result<GTFSIterator<B, F, T>, GtfsError> {
//        let csv = Csv::from_file(&path).unwrap();
//        let parsers = match filename {
//            "agency.txt" => agencies::parse_row,
//            "calendar.txt" => calendars::parse_row,
//            "calendar_dates.txt" => calendar_dates::parse_row,
//            "frequencies.txt" => frequencies::parse_row,
//            "routes.txt" => routes::parse_row,
//            "shapes.txt" => shapes::parse_row,
//            "stops.txt" => stops::parse_row,
//            "stop_times.txt" => stop_times::parse_row,
//            "trips.txt" => trips::parse_row,
//            _ => return Err(GtfsError::CsvHeader(format!("No parser for {}", filename))),
//        };
//        GTFSIterator::new(csv, filename.to_string(), parser)
//    }
}

impl<B, F, T> Iterator for GTFSIterator<B, F, T>
    where B: BufRead,
          F: (Fn(Zip<Iter<String>, Columns>) -> Result<T, ParseError>)
{
    type Item = Result<T, GtfsError>;

    fn next(&mut self) -> Option<Result<T, GtfsError>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let result = match (self.parser)(self.header.iter().zip(columns)) {
                            Ok(x) => Some(Ok(x)),
                            Err(y) => Some(Err(GtfsError::LineParseError(y, self.filename.clone(), self.line))),
                        };
                        self.line += 1;
                        result
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err, self.filename.clone(), self.line))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err, self.filename.clone(), self.line))),
            }
        }
    }
}

#[test]
fn test_read_agencies() {
    let csv = Csv::from_file("./examples/agency.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "agency.txt".to_string(), agencies::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_calendar_dates() {
    let csv = Csv::from_file("./examples/calendar_dates.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "calendar_dates.txt".to_string(), calendar_dates::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_calendar() {
    let csv = Csv::from_file("./examples/calendar.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "calendar.txt".to_string(), calendars::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_frequencies() {
    let csv = Csv::from_file("./examples/frequencies.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "frequencies.txt".to_string(), calendars::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_routes() {
    let csv = Csv::from_file("./examples/routes.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "routes.txt".to_string(), routes::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_shapes() {
    let csv = Csv::from_file("./examples/shapes.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "shapes.txt".to_string(), shapes::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_stops() {
    let csv = Csv::from_file("./examples/stops.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "stops.txt".to_string(), stops::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_stop_times() {
    let csv = Csv::from_file("./examples/stop_times.txt").unwrap();
    /*let csv = Csv::from_string("trip_id,arrival_time,departure_time,stop_id,stop_sequence,stop_headsign,pickup_type,drop_off_time,shape_dist_traveled
                                STBA,6:00:00,6:00:00,STAGECOACH,1,,,,
                                STBA,6:20:00,6:20:00,BEATTY_AIRPORT,2,,,,
                                CITColumns1,6:00:00,6:00:00,STAGECOACH,1,,,,
                                CITColumns1,6:05:00,6:07:00,NANAA,2,,,,
                                CITColumns1,6:12:00,6:14:00,NADAV,3,,,,
                                CITColumns1,6:19:00,6:21:00,DADAN,4,,,,
                                CITColumns1,6:26:00,6:28:00,EMSI,5,,,,
                                CITColumns2,6:28:00,6:30:00,EMSI,1,,,,
                                CITColumns2,6:35:00,6:37:00,DADAN,2,,,,
                                CITColumns2,6:42:00,6:44:00,NADAV,3,,,,
                                CITColumns2,6:49:00,6:51:00,NANAA,4,,,,
                                CITColumns2,6:56:00,6:58:00,STAGECOACH,5,,,,
                                AB1,8:00:00,8:00:00,BEATTY_AIRPORT,1,,,,
                                AB1,8:10:00,8:15:00,BULLFROG,2,,,,
                                AB2,12:05:00,12:05:00,BULLFROG,1,,,,
                                AB2,12:15:00,12:15:00,BEATTY_AIRPORT,2
                                BFC1,8:20:00,8:20:00,BULLFROG,1
                                BFC1,9:20:00,9:20:00,FUR_CREEK_RES,2
                                BFC2,11:00:00,11:00:00,FUR_CREEK_RES,1
                                BFC2,12:00:00,12:00:00,BULLFROG,2
                                AAMV1,8:00:00,8:00:00,BEATTY_AIRPORT,1
                                AAMV1,9:00:00,9:00:00,AMV,2");
                                */
    let iterator = GTFSIterator::new(csv, "stop_times.txt".to_string(), stop_times::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}

#[test]
fn test_read_trips() {
    let csv = Csv::from_file("./examples/trips.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "trips.txt".to_string(), trips::parse_row).unwrap();
    for entry in iterator {
        // assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        let _ = entry;
    }
}
