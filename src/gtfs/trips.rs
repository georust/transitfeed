use transit::{Trip, BikesAllowed, WheelchairAccessible};
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::GtfsError;

use gtfs::parse::{parse_wheelchair_accessible, parse_bikes_allowed};

pub fn parse_row(row: Zip<Iter<String>, Columns>, line: usize, filename:&str) -> Result<Trip, GtfsError>
{
    let mut route_id = String::new();
    let mut service_id = String::new();
    let mut trip_id = String::new();
    let mut trip_headsign = None;
    let mut trip_short_name = None;
    let mut direction_id = None;
    let mut block_id = None;
    let mut shape_id = None;
    let mut wheelchair_accessible = WheelchairAccessible::NoInformation;
    let mut bikes_allowed = BikesAllowed::NoInformation;

    for (header, column) in row {
        match &header[..] {
            "route_id" => { route_id = String::from(column); },
            "service_id" => { service_id = String::from(column); },
            "trip_id" => { trip_id = String::from(column); },
            "trip_headsign" => { trip_headsign = Some(String::from(column)); },
            "trip_short_name" => { trip_short_name = Some(String::from(column)); },
            "direction_id" => { direction_id = Some(String::from(column)); },
            "block_id" => { block_id = Some(String::from(column)); },
            "shape_id" => { shape_id = Some(String::from(column)); },
            "wheelchair_accessible" => { wheelchair_accessible = parse_try2!(parse_wheelchair_accessible(line, filename, column)); },
            "bikes_allowed" => { bikes_allowed = parse_try2!(parse_bikes_allowed(line, filename, column)); },
            _ => (),
        }
    }
    Ok(Trip {
        route_id: route_id,
        service_id: service_id,
        trip_id: trip_id,
        trip_headsign: trip_headsign,
        trip_short_name: trip_short_name,
        direction_id: direction_id,
        block_id: block_id,
        shape_id: shape_id,
        wheelchair_accessible: wheelchair_accessible,
        bikes_allowed: bikes_allowed,
    })
}
