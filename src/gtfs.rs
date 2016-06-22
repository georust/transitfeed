use std::io::{BufReader, Read, Seek};
use std::result;
use std::num::ParseIntError;
use zip::ZipArchive;
use zip::result::ZipError;
use quick_csv::{Csv, Row};
use quick_csv::error::{Error as CsvError};
use chrono::{UTC, ParseError as ParseDateTimeError};

use transit::{Transit, Agency, Stop, StopTime, TimeOffset, Route, Trip, Shape};

/// Result and Error types
error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    links {
    }

    foreign_links {
        ZipError, Zip, "Zip file error";
        CsvError, Csv, "CSV parsing error";
        ParseDateTimeError, ParseDateTime, "DateTime parsing error";
        ParseIntError, ParseInt, "Int parsing error";
    }

    errors {
        EmptyCsv {
            description("empty csv file")
            display("empty csv file")
        }
        ParseTimeError {
            description("invalid time string")
            display("invalid time string")
        }
    }
}

/// GTFS represents a decoded GTFS data set. This implements the Transit interface.
pub struct GTFS {
    transit: Transit,
    agencies: Vec<Agency>,
    stops: Vec<Stop>,
    stop_times: Vec<StopTime>,
    routes: Vec<Route>,
    trips: Vec<Trip>,
    shapes: Vec<Shape>,
}


impl GTFS {
    /// Decode a GTFS data set from a Zip Archive
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<GTFS> {
        // TODO calculate the hash for the raw data
        let mut archive = try!(ZipArchive::new(reader));
        let agencies_reader = try!(archive.by_name("agencies.txt"));
        let agencies = try!(GTFS::decode_agencies(agencies_reader));
        Ok(GTFS {
            transit: Transit {
                id: 0,
                sha512: String::from(""),
                name: String::from("unnamed"),
                created: UTC::now(),
            },
            agencies: agencies,
            stops: Vec::new(),
            stop_times: Vec::new(),
            routes: Vec::new(),
            trips: Vec::new(),
            shapes: Vec::new(),
        })
    }

    fn decode_agencies<R: Read>(reader: R) -> Result<Vec<Agency>> {
         Ok(vec!())
    }

    fn decode_stops<R: Read>(reader: R) -> Result<Vec<Stop>> {
        Ok(vec!())
    }

    fn decode_stop_times<R: Read>(reader: R) -> Result<Vec<StopTime>> {
        let buf_reader = BufReader::new(reader);
        let mut csv = Csv::from_reader(buf_reader);
        let decoder = try!(StopTimeDecoder::new(&mut csv));
        let mut stop_times: Vec<StopTime> = Vec::new();
        for result in decoder {
            let stop_time = try!(result);
            stop_times.push(stop_time);
        }
        Ok(stop_times)
    }

    fn decode_routes<R: Read>(reader: R) -> Result<Vec<Route>> {
        Ok(vec!())
    }

    fn decode_trips<R: Read>(reader: R) -> Result<Vec<Trip>> {
        Ok(vec!())
    }

    fn decode_shapes<R: Read>(reader: R) -> Result<Vec<Shape>> {
        Ok(vec!())
    }
}

/// Takes a csv iterator and produces the headers row as a Vec<String>
fn csv_headers(csv: &mut Iterator<Item=result::Result<Row, CsvError>>) -> Result<Vec<String>> {
    match csv.next() {
        Some(row_result) => match row_result {
            Ok(row) => {
                let columns = try!(row.columns());
                Ok(columns.into_iter().map(|header| String::from(header)).collect())
            }
            Err(err) => Err(err.into()),
        },
        None => Err(ErrorKind::EmptyCsv.into()),
    }
}

/// Takes a &str containing an arrival/departure time for gtfs and returns
/// a naivetime. Chrono's NaiveTime parser is relatively slow and doesn't
/// account for the optional leading zeros in the hour part.
fn decode_timeoffset(tm: &str) -> Result<TimeOffset> {
    let mut parts = tm.trim().split(':');
    let parse_part = |part: Option<&str>| -> Result<u32> {
        match part {
            Some(val) => Ok(try!(val.parse::<u32>())),
            None => Err(ErrorKind::ParseTimeError.into()),
        }
    };
    let hours = try!(parse_part(parts.next()));
    let minutes = try!(parse_part(parts.next()));
    let seconds = try!(parse_part(parts.next()));
    Ok(TimeOffset::from_hms(hours, minutes, seconds))
}

/// An decoder which returns as its iterator output a new `StopTime` struct
pub struct StopTimeDecoder<'a> {
    csv: &'a mut Iterator<Item=result::Result<Row, CsvError>>,
    headers: Vec<String>,
}


impl<'a> StopTimeDecoder<'a> {
    pub fn new(csv: &'a mut Iterator<Item=result::Result<Row, CsvError>>) -> Result<StopTimeDecoder<'a>> {
        let headers = { try!(csv_headers(csv)) };
        Ok(StopTimeDecoder {
            csv: csv,
            headers: headers,
        })
    }

}

impl<'a> Iterator for StopTimeDecoder<'a> {
    type Item = Result<StopTime>;

    fn next(&mut self) -> Option<Result<StopTime>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let mut transit_id = 0;
                        let mut trip_id = String::new();
                        let mut departure_time = TimeOffset::from_hms(0, 0, 0);
                        let mut arrival_time = TimeOffset::from_hms(0, 0, 0);
                        let mut stop_id = String::new();
                        let mut stop_sequence = 0;
                        let mut stop_headsign = None;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "trip_id" => { trip_id = String::from(column); },
                                "departure_time" => { departure_time = match decode_timeoffset(column) { Ok(naive_time) => naive_time, Err(err) => return Some(Err(err.into())) }; },
                                "arrival_time" => { arrival_time = match decode_timeoffset(column) { Ok(naive_time) => naive_time, Err(err) => return Some(Err(err.into())) }; },
                                "stop_id" => { stop_id = String::from(column); },
                                "stop_sequence" => { let parse_res: result::Result<u64, ParseIntError> = column.parse(); stop_sequence = match parse_res { Ok(stop_seq) => stop_seq, Err(err) => return Some(Err(err.into())) }; },
                                "stop_headsign" => { stop_headsign = Some(String::from(column)); },
                                _ => (),
                            }
                        }
                        let stop_time = StopTime {
                            transit_id: transit_id,
                            trip_id: trip_id,
                            departure_time: departure_time,
                            arrival_time: arrival_time,
                            stop_id: stop_id,
                            stop_sequence: stop_sequence,
                            stop_headsign: stop_headsign,
                        };
                        //println!("{:?}", stop_time);
                        Some(Ok(stop_time))
                    },
                    Err(err) => Some(Err(err.into())),
                },
                Err(err) => Some(Err(err.into())),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Display};
    use std::fs;
    use std::io::Read;
    use test::Bencher;
    use quick_csv::Csv;
    use super::StopTimeDecoder;

    static STOP_TIMES_DATA: &'static str = "./examples/stop_times.txt";

    fn or_die<T, E: Debug+Display>(r: Result<T, E>) -> T {
        r.or_else(|e: E| -> Result<T, E> { panic!(format!("{:?}", e)) }).unwrap()
    }

    fn file_to_mem(fp: &str) -> Vec<u8> {
        let mut f = or_die(fs::File::open(fp));
        let mut bs = vec![];
        or_die(f.read_to_end(&mut bs));
        bs
    }

    #[bench]
    fn bench_stop_time_decoder(b: &mut Bencher) {
        let data = file_to_mem(STOP_TIMES_DATA);
        b.bytes = data.len() as u64;
        b.iter(|| {
            let mut csv = Csv::from_reader(&*data);
            let decoder = StopTimeDecoder::new(&mut csv).unwrap();
            for stop_time in decoder {
                let _ = stop_time;
            }
        })
    }
}
