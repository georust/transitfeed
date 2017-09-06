use transit::{CalendarDate, ExceptionType};
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use chrono::NaiveDate;
use gtfs::parse::{parse_date, parse_exceptiontype};
use gtfs::error::GtfsError;

pub fn parse_row(row: Zip<Iter<String>, Columns>, line: usize, filename:&str) -> Result<CalendarDate, GtfsError>
{
    let mut service_id = String::new();
    let mut date = NaiveDate::from_ymd(2017, 1, 1);
    let mut exception_type = ExceptionType::ServiceAdded;

    for (header, column) in row {
        match &header[..] {
            "service_id" => { service_id = String::from(column); },
            "date" => { date = parse_try2!(parse_date(line, &filename, column)) }
            "exception_type" => { exception_type = parse_try2!(parse_exceptiontype(line, &filename, column)) }
            _ => (),
        }
    }
    Ok(CalendarDate {
        service_id: service_id,
        date: date,
        exception_type: exception_type
    })
}
