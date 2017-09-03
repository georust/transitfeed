use std::io::BufRead;
use quick_csv::Csv;
use transit::{Calendar};

use chrono::NaiveDate;
use gtfs::parse::{parse_dow, parse_date};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Calendar` struct from
/// a CSV iterator.
pub struct CalendarIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> CalendarIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<CalendarIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("calendars.txt")))
        } else {
            Ok(CalendarIterator {
                csv: csv,
                headers: headers,
                line: 0,
            })
        }
    }
}

impl<B: BufRead> Iterator for CalendarIterator<B> {
    type Item = GtfsResult<Calendar>;

    fn next(&mut self) -> Option<GtfsResult<Calendar>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "calendars.txt";
                        let mut service_id = String::new();
                        let mut monday = false;
                        let mut tuesday = false;
                        let mut wednesday = false;
                        let mut thursday = false;
                        let mut friday = false;
                        let mut saturday = false;
                        let mut sunday = false;
                        let mut start_date = NaiveDate::from_ymd(2017, 1, 1);
                        let mut end_date = NaiveDate::from_ymd(2017, 1, 1);

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "service_id" => { service_id = String::from(column); },
                                "monday" => { monday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "tuesday" => { tuesday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "wednesday" => { wednesday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "thursday" => { thursday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "friday" => { friday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "saturday" => { saturday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "sunday" => { sunday = parse_try!(parse_dow(self.line, &filename, column)); },
                                "start_date" => { start_date = parse_try!(parse_date(self.line, &filename, column)) }
                                "end_date" => { end_date = parse_try!(parse_date(self.line, &filename, column)) }
                                _ => (),
                            }
                        }
                        let calendar = Calendar {
                            service_id: service_id,
                            monday: monday,
                            tuesday: tuesday,
                            wednesday: wednesday,
                            thursday: thursday,
                            friday: friday,
                            saturday: saturday,
                            sunday: sunday,
                            start_date: start_date,
                            end_date: end_date
                        };
                        self.line += 1;
                        Some(Ok(calendar))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}
