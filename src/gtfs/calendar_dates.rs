use std::io::BufRead;
use quick_csv::Csv;
use transit::{CalendarDate, ExceptionType};

use chrono::NaiveDate;
use gtfs::parse::{parse_date, parse_exceptiontype};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Calendar` struct from
/// a CSV iterator.
pub struct CalendarDateIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> CalendarDateIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<CalendarDateIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("calendar_dates.txt")))
        } else {
            Ok(CalendarDateIterator {
                csv: csv,
                headers: headers,
                line: 0,
            })
        }
    }
}

impl<B: BufRead> Iterator for CalendarDateIterator<B> {
    type Item = GtfsResult<CalendarDate>;

    fn next(&mut self) -> Option<GtfsResult<CalendarDate>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "calendar_dates.txt";
                        let mut service_id = String::new();
                        let mut date = NaiveDate::from_ymd(2017, 1, 1);
                        let mut exception_type = ExceptionType::ServiceAdded;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "service_id" => { service_id = String::from(column); },
                                "date" => { date = parse_try!(parse_date(self.line, &filename, column)) }
                                "exception_type" => { exception_type = parse_try!(parse_exceptiontype(self.line, &filename, column)) }
                                _ => (),
                            }
                        }
                        let calendar_date = CalendarDate {
                            service_id: service_id,
                            date: date,
                            exception_type: exception_type
                        };
                        self.line += 1;
                        Some(Ok(calendar_date))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}
