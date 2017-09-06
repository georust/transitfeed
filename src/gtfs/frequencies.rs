use transit::{Frequency, FrequencyAccuracy, TimeOffset};
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::GtfsError;

use gtfs::parse::{parse_int, parse_exact_times, parse_timeoffset};

pub fn parse_row(row: Zip<Iter<String>, Columns>, line: usize, filename:&str) -> Result<Frequency, GtfsError>
{
    let mut trip_id = String::new();
    let mut start_time = TimeOffset::from_hms(0, 0, 0);
    let mut end_time = TimeOffset::from_hms(0, 0, 0);
    let mut headway_secs = 0;
    let mut exact_times = FrequencyAccuracy::Approximate;

    for (header, column) in row {
        match &header[..] {
            "trip_id" => { trip_id = String::from(column); },
            "start_time" => { start_time = parse_try2!(parse_timeoffset(line, &filename, column)); },
            "end_time" => { end_time = parse_try2!(parse_timeoffset(line, &filename, column)); },
            "headway_secs" => { headway_secs = parse_try2!(parse_int(line, filename, column)); },
            "exact_times" => { exact_times = parse_try2!(parse_exact_times(line, filename, column)); },
            _ => (),
        }
    }
    Ok(Frequency {
        trip_id: trip_id,
        start_time: start_time,
        end_time: end_time,
        headway_secs: headway_secs,
        exact_times: exact_times
    })
}
