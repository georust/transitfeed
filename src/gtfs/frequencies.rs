use std::io::BufRead;
use quick_csv::Csv;
use transit::{Frequency, FrequencyAccuracy, TimeOffset};

use gtfs::parse::{parse_int, parse_exact_times, parse_timeoffset};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Frequency` struct from
/// a CSV iterator.
pub struct FrequencyIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> FrequencyIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<FrequencyIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("frequencies.txt")))
        } else {
            Ok(FrequencyIterator {
                csv: csv,
                headers: headers,
                line: 0,
            })
        }
    }
}

impl<B: BufRead> Iterator for FrequencyIterator<B> {
    type Item = GtfsResult<Frequency>;

    fn next(&mut self) -> Option<GtfsResult<Frequency>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "frequencies.txt";
                        let mut trip_id = String::new();
                        let mut start_time = TimeOffset::from_hms(0, 0, 0);
                        let mut end_time = TimeOffset::from_hms(0, 0, 0);
                        let mut headway_secs = 0;
                        let mut exact_times = FrequencyAccuracy::Approximate;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "trip_id" => { trip_id = String::from(column); },
                                "start_time" => { start_time = parse_try!(parse_timeoffset(self.line, &filename, column)); },
                                "end_time" => { end_time = parse_try!(parse_timeoffset(self.line, &filename, column)); },
                                "headway_secs" => { headway_secs = parse_try!(parse_int(self.line, filename, column)); },
                                "exact_times" => { exact_times = parse_try!(parse_exact_times(self.line, filename, column)); },
                                _ => (),
                            }
                        }
                        let frequency = Frequency {
                            trip_id: trip_id,
                            start_time: start_time,
                            end_time: end_time,
                            headway_secs: headway_secs,
                            exact_times: exact_times
                        };
                        self.line += 1;
                        Some(Ok(frequency))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}
