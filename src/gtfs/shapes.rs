use transit::Shape;
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::GtfsError;

use gtfs::parse::{parse_int, parse_float};

pub fn parse_row(row: Zip<Iter<String>, Columns>, line: usize, filename:&str) -> Result<Shape, GtfsError>
{
    let mut shape_id = String::new();
    let mut shape_pt_lat = 0.0;
    let mut shape_pt_lon = 0.0;
    let mut shape_pt_sequence = 0;
    let mut shape_dist_traveled = 0.0;

    for (header, column) in row {
        match &header[..] {
            "shape_id" => { shape_id = String::from(column); },
            "shape_pt_lat" => { shape_pt_lat = parse_try2!(parse_float(line, filename, column)); },
            "shape_pt_lon" => { shape_pt_lon = parse_try2!(parse_float(line, filename, column)); },
            "shape_pt_sequence" => { shape_pt_sequence = parse_try2!(parse_int(line, filename, column)); },
            "shape_dist_traveled" => { shape_dist_traveled = parse_try2!(parse_float(line, filename, column)); },
            _ => (),
        }
    }
    Ok(Shape {
        shape_id: shape_id,
        shape_pt_lat: shape_pt_lat,
        shape_pt_lon: shape_pt_lon,
        shape_pt_sequence: shape_pt_sequence,
        shape_dist_traveled: shape_dist_traveled,
    })
}
