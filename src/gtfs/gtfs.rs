use std::io::{Read, Seek};
use zip::ZipArchive;

pub struct GTFS<R: Read+Seek> {
    raw: R,
    archive: ZipArchive<R>,
}


//impl<'a, E: Error, R: Read+Seek+'a, B: BufRead> Transit<'a, E> for GTFS<R> {
//    type AgencyIterator = AgencyIterator<'a, B>;
//
//    fn agencies(&self) -> AgencyIterator {
//        let csv = Csv::from_reader(self.archive.by_name("agency.txt").unwrap());
//        AgencyIterator::new(csv)
//    }
//
//}
