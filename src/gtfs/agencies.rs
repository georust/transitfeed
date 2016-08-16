use std::io::BufRead;
use quick_csv::Csv;
use transit::Agency;
use gtfs::error::{GtfsResult, GtfsError};

const AGENCY_FILENAME: &'static str = "agency.txt";

/// An decoder which returns as its iterator output a new `Stop` struct from
/// a CSV iterator.
pub struct AgencyIterator<B: BufRead> {
    csv: Csv<B>,
    headers: Vec<String>,
    line: usize,
}

impl<B: BufRead> AgencyIterator<B> {
    pub fn new(csv: Csv<B>) -> GtfsResult<AgencyIterator<B>> {
        let mut csv = csv.has_header(true);
        let headers = csv.headers();
        if headers.len() == 0 {
            Err(GtfsError::CsvHeader(String::from(AGENCY_FILENAME)))
        } else {
            Ok(AgencyIterator{
                csv: csv,
                headers: headers,
                line: 1,
            })
        }
    }
}

impl<B: BufRead> Iterator for AgencyIterator<B> {
    type Item = GtfsResult<Agency>;

    fn next(&mut self) -> Option<GtfsResult<Agency>> {
        match self.csv.next() {
            None => None,
            Some(res) => match res {
                Ok(row) => match row.columns() {
                    Ok(columns) =>  {
                        //let filename = "agency.txt";
                        let mut agency_id = None;
                        let mut agency_name = String::new();
                        let mut agency_url = String::new();
                        let mut agency_timezone = String::new();
                        let mut agency_lang = None;
                        let mut agency_phone = None;
                        let mut agency_fare_url = None;
                        let mut agency_email = None;

                        for (header, column) in self.headers.iter().zip(columns) {
                            match &header[..] {
                                "agency_id" => { agency_id = Some(String::from(column)); },
                                "agency_name" => { agency_name = String::from(column); },
                                "agency_url" => { agency_url= String::from(column); },
                                "agency_timezone" => { agency_timezone = String::from(column); },
                                "agency_lang" => { agency_lang = Some(String::from(column)); },
                                "agency_phone" => { agency_phone = Some(String::from(column)); },
                                "agency_fare_url" => { agency_fare_url = Some(String::from(column)); },
                                "agency_email" => { agency_email = Some(String::from(column)); },
                                _ => (),
                            }
                        }
                        let agency = Agency {
                            agency_id: agency_id,
                            agency_name: agency_name,
                            agency_url: agency_url,
                            agency_timezone: agency_timezone,
                            agency_lang: agency_lang,
                            agency_phone: agency_phone,
                            agency_fare_url: agency_fare_url,
                            agency_email: agency_email,
                        };
                        self.line += 1;
                        //println!("{:?}", stop_time);
                        Some(Ok(agency))
                    },
                    Err(err) => Some(Err(GtfsError::Csv(err))),
                },
                Err(err) => Some(Err(GtfsError::Csv(err))),
            }
        }
    }
}

