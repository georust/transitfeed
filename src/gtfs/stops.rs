use transit::{Stop, WheelchairBoarding, LocationType};
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::GtfsError;

use gtfs::parse::{parse_float, parse_location_type, parse_wheelchair_boarding};

pub fn parse_row(row: Zip<Iter<String>, Columns>, line: usize, filename:&str) -> Result<Stop, GtfsError>
{
    let mut stop_id = String::new();
    let mut stop_code = None;
    let mut stop_name = String::new();
    let mut stop_desc = None;
    let mut stop_lat = 0.0;
    let mut stop_lon = 0.0;
    let mut zone_id = None;
    let mut stop_url = None;
    let mut location_type = LocationType::Stop;
    let mut parent_station = None;
    let mut stop_timezone = None;
    let mut wheelchair_boarding = WheelchairBoarding::NoInformation;

    for (header, column) in row {
        match &header[..] {
            "stop_id" => { stop_id = String::from(column); },
            "stop_code" => { stop_code = Some(String::from(column)); },
            "stop_name" => { stop_name = String::from(column); },
            "stop_desc" => { stop_desc = Some(String::from(column)); },
            "stop_lat" => { stop_lat = parse_try2!(parse_float(line, filename, column)); },
            "stop_lon" => { stop_lon = parse_try2!(parse_float(line, filename, column)); },
            "zone_id" => { zone_id = Some(String::from(column)); },
            "stop_url" => { stop_url = Some(String::from(column)); },
            "location_type" => { location_type = parse_try2!(parse_location_type(line, filename, column)); },
            "parent_station" => { parent_station= Some(String::from(column)); },
            "stop_timezone" => { stop_timezone = Some(String::from(column)); },
            "wheelchair_boarding" => { wheelchair_boarding = parse_try2!(parse_wheelchair_boarding(line, filename, column)); },
            _ => (),
        }
    }
    Ok(Stop {
        stop_id: stop_id,
        stop_code: stop_code,
        stop_name: stop_name,
        stop_desc: stop_desc,
        stop_lat: stop_lat,
        stop_lon: stop_lon,
        zone_id: zone_id,
        stop_url: stop_url,
        location_type: location_type,
        parent_station: parent_station,
        stop_timezone: stop_timezone,
        wheelchair_boarding: wheelchair_boarding,
    })
}
