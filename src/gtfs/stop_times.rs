use transit::{StopTime, DropoffType, PickupType, Timepoint, TimeOffset};
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::parse::{parse_timeoffset, parse_float, parse_pickup_type, parse_dropoff_type, parse_int};
use gtfs::error::GtfsError;

pub fn parse_row(row: Zip<Iter<String>, Columns>, line: usize, filename:&str) -> Result<StopTime, GtfsError>
{
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
    for (header_item, column) in row {
        match &header_item[..] {
            "trip_id" => { trip_id = String::from(column); },
            "departure_time" => { departure_time = parse_try2!(parse_timeoffset(line, filename, column)); },
            "arrival_time" => { arrival_time = parse_try2!(parse_timeoffset(line, filename, column)); },
            "stop_id" => { stop_id = String::from(column); },
            "stop_sequence" => { stop_sequence = parse_try2!(parse_int(line, filename, column)); },
            "stop_headsign" => { stop_headsign = Some(String::from(column)) },
            "pickup_type" => { pickup_type = parse_try2!(parse_pickup_type(line, filename, column)); },
            "dropoff_type" => { dropoff_type = parse_try2!(parse_dropoff_type(line, filename, column)); },
            "shape_dist_traveled" => { shape_dist_traveled = Some(parse_float(line, filename, column).unwrap_or(0.0)); }, // # TODO: needs to be None if empty
            _ => (),
        }
    }
    Ok(StopTime {
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
    })
}
