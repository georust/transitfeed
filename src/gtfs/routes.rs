use transit::{Route, RouteType};
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::ParseError;

use gtfs::parse::{parse_route_type};

pub fn parse_row(row: Zip<Iter<String>, Columns>) -> Result<Route, ParseError>
{
    let mut route_id = String::new();
    let mut agency_id = None;
    let mut route_short_name = String::new();
    let mut route_long_name = String::new();
    let mut route_desc = None;
    let mut route_type = RouteType::Bus;
    let mut route_url = None;
    let mut route_color = None;
    let mut route_text_color = None;

    for (header, column) in row {
        match &header[..] {
            "route_id" => { route_id = String::from(column); },
            "agency_id" => { agency_id = Some(String::from(column)); },
            "route_short_name" => { route_short_name = String::from(column); },
            "route_long_name" => { route_long_name = String::from(column); },
            "route_desc" => { route_desc = Some(String::from(column)); },
            "route_type" => { route_type = parse_try!(parse_route_type(column)); },
            "route_url" => { route_url = Some(String::from(column)); },
            "route_color" => { route_color = Some(String::from(column)); },
            "route_text_color" => { route_text_color = Some(String::from(column)); },
            _ => (),
        }
    }
    Ok(Route {
        route_id: route_id,
        agency_id: agency_id,
        route_short_name: route_short_name,
        route_long_name: route_long_name,
        route_desc: route_desc,
        route_type: route_type,
        route_url: route_url,
        route_color: route_color,
        route_text_color: route_text_color,
    })
}
