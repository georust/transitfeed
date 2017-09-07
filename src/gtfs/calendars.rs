use transit::Calendar;
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::ParseError;

use chrono::NaiveDate;
use gtfs::parse::{parse_dow, parse_date};

pub fn parse_row(row: Zip<Iter<String>, Columns>) -> Result<Calendar, ParseError>
{
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

    for (header, column) in row {
        match &header[..] {
            "service_id" => { service_id = String::from(column); },
            "monday" => { monday = parse_try!(parse_dow(column)); },
            "tuesday" => { tuesday = parse_try!(parse_dow(column)); },
            "wednesday" => { wednesday = parse_try!(parse_dow(column)); },
            "thursday" => { thursday = parse_try!(parse_dow(column)); },
            "friday" => { friday = parse_try!(parse_dow(column)); },
            "saturday" => { saturday = parse_try!(parse_dow(column)); },
            "sunday" => { sunday = parse_try!(parse_dow(column)); },
            "start_date" => { start_date = parse_try!(parse_date(column)) }
            "end_date" => { end_date = parse_try!(parse_date(column)) }
            _ => (),
        }
    }
    Ok(Calendar {
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
    })
}
