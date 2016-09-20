use std::io::BufRead;
use quick_csv::Csv;
use transit::Shape;

use gtfs::parse::{parse_int, parse_float};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Stop` struct from
/// a CSV iterator.
pub struct ShapeIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> ShapeIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<ShapeIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("shapes.txt")))
        } else {
            Ok(ShapeIterator {
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for ShapeIterator<B> {
    type Item = GtfsResult<Shape>;

    fn next(&mut self) -> Option<GtfsResult<Shape>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "shapes.txt";
                        let mut shape_id = String::new();
                        let mut shape_pt_lat = 0.0;
                        let mut shape_pt_lon = 0.0;
                        let mut shape_pt_sequence = 0;
                        let mut shape_dist_traveled = 0.0;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "shape_id" => { shape_id = String::from(column); },
                                "shape_pt_lat" => { shape_pt_lat = parse_try!(parse_float(self.line, filename, column)); },
                                "shape_pt_lon" => { shape_pt_lon = parse_try!(parse_float(self.line, filename, column)); },
                                "shape_pt_sequence" => { shape_pt_sequence = parse_try!(parse_int(self.line, filename, column)); },
                                "shape_dist_traveled" => { shape_dist_traveled = parse_try!(parse_float(self.line, filename, column)); },
                                _ => (),
                            }
                        }
                        let shape = Shape {
                            shape_id: shape_id,
                            shape_pt_lat: shape_pt_lat,
                            shape_pt_lon: shape_pt_lon,
                            shape_pt_sequence: shape_pt_sequence,
                            shape_dist_traveled: shape_dist_traveled,
                        };
                        self.line += 1;
                        Some(Ok(shape))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}
