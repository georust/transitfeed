use transit::Agency;
use std::iter::Zip;
use std::slice::Iter;
use quick_csv::columns::Columns;
use gtfs::error::ParseError;

pub fn parse_row(row: Zip<Iter<String>, Columns>) -> Result<Agency, ParseError>
{
    let mut agency_id = None;
    let mut agency_name = String::new();
    let mut agency_url = String::new();
    let mut agency_timezone = String::new();
    let mut agency_lang = None;
    let mut agency_phone = None;
    let mut agency_fare_url = None;
    let mut agency_email = None;

    for (header_item, column) in row {
        match &header_item[..] {
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
    Ok(Agency {
        agency_id: agency_id,
        agency_name: agency_name,
        agency_url: agency_url,
        agency_timezone: agency_timezone,
        agency_lang: agency_lang,
        agency_phone: agency_phone,
        agency_fare_url: agency_fare_url,
        agency_email: agency_email,
    })
}
