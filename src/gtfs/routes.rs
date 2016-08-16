use std::io::BufRead;
use quick_csv::Csv;
use transit::{Route, RouteType};

use gtfs::parse::{parse_route_type};
use gtfs::error::{GtfsResult, GtfsError};

/// An decoder which returns as its iterator output a new `Route` struct from
/// a CSV iterator.
pub struct RouteIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> RouteIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<RouteIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from("routes.txt")))
        } else {
            Ok(RouteIterator {
                csv: csv,
                headers: headers,
                line: 0,
            })
        }
    }
}

impl<B: BufRead> Iterator for RouteIterator<B> {
    type Item = GtfsResult<Route>;

    fn next(&mut self) -> Option<GtfsResult<Route>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        let filename = "routes.txt";
                        let mut route_id = String::new();
                        let mut agency_id = None;
                        let mut route_short_name = String::new();
                        let mut route_long_name = String::new();
                        let mut route_desc = None;
                        let mut route_type = RouteType::Bus;
                        let mut route_url = None;
                        let mut route_color = None;
                        let mut route_text_color = None;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "route_id" => { route_id = String::from(column); },
                                "agency_id" => { agency_id = Some(String::from(column)); },
                                "route_short_name" => { route_short_name = String::from(column); },
                                "route_long_name" => { route_long_name = String::from(column); },
                                "route_desc" => { route_desc = Some(String::from(column)); },
                                "route_type" => { route_type = parse_try!(parse_route_type(self.line, filename, column)); },
                                "route_url" => { route_url = Some(String::from(column)); },
                                "route_color" => { route_color = Some(String::from(column)); },
                                "route_text_color" => { route_text_color = Some(String::from(column)); },
                                _ => (),
                            }
                        }
                        let route = Route {
                            route_id: route_id,
                            agency_id: agency_id,
                            route_short_name: route_short_name,
                            route_long_name: route_long_name,
                            route_desc: route_desc,
                            route_type: route_type,
                            route_url: route_url,
                            route_color: route_color,
                            route_text_color: route_text_color,
                        };
                        self.line += 1;
                        Some(Ok(route))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}




