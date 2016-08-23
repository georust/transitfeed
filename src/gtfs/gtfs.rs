use std::error::Error;
use std::io::{BufRead, Read, Seek};
use zip::ZipArchive;
use quick_csv::Csv;
use transit::Transit;
use gtfs::error::GtfsError;
use gtfs::{AgencyIterator, RouteIterator, StopIterator};

pub struct GTFS<R: Read+Seek> {
    raw: R,
    archive: ZipArchive<R>,
}


impl<'a, E: Error, R: Read+Seek+'a, B: BufRead> Transit<'a, E> for GTFS<R> {
    type AgencyIterator = AgencyIterator<'a, B>;

    fn agencies(&self) -> AgencyIterator {
        let csv = Csv::from_reader(self.archive.by_name("agency.txt").unwrap());
        AgencyIterator::new(csv)
    }

}
