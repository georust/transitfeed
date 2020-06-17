use csv;
use serde;
use std::fs::File;
use std::path::Path;
use tempfile::{Builder, TempDir};
use zip;

use super::archive::extract_zip;
use super::{
    Agency, Calendar, CalendarDate, FareAttribute, FareRule, FeedInfo, Frequency, Route,
    ShapePoint, Stop, StopTime, Transfer, Trip,
};
use super::{Error, GTFSIterator};

pub use csv::{Terminator, Trim};

#[derive(Debug)]
pub struct FeedReader<P>
where
    P: FeedProvider,
{
    provider: P,
    builder: csv::ReaderBuilder,
}

pub trait FeedProvider {
    fn path(&self) -> &str;
}

pub struct LocalFeedProvider {
    path: String,
}

impl LocalFeedProvider {
    fn new(path: &str) -> LocalFeedProvider {
        LocalFeedProvider {
            path: path.to_string(),
        }
    }
}

impl FeedProvider for LocalFeedProvider {
    fn path(&self) -> &str {
        return &self.path;
    }
}

#[derive(Debug)]
pub struct ZipFeedProvider {
    dir: TempDir,
}

impl ZipFeedProvider {
    fn new(zipfile: &str) -> Result<ZipFeedProvider, Error> {
        let dir = Builder::new()
            .prefix("transitfeed")
            .tempdir()
            .map_err(|e| Error::Feed(format!("{}", e)))?;
        let mut zip =
            zip::ZipArchive::new(File::open(zipfile).map_err(|e| Error::Feed(format!("{}", e)))?)
                .map_err(|e| Error::Feed(format!("{}", e)))?;
        extract_zip(&mut zip, dir.path()).map_err(|e| Error::Feed(format!("{}", e)))?;
        Ok(ZipFeedProvider { dir: dir })
    }
}

impl FeedProvider for ZipFeedProvider {
    fn path(&self) -> &str {
        self.dir.path().to_str().unwrap()
    }
}

impl FeedReader<LocalFeedProvider> {
    pub fn new(path: &str) -> Self {
        FeedReader::from_provider(LocalFeedProvider::new(path))
    }
}

impl FeedReader<ZipFeedProvider> {
    pub fn from_zip(zipfile: &str) -> Result<Self, Error> {
        Ok(FeedReader::from_provider(ZipFeedProvider::new(zipfile)?))
    }
}

impl<P: FeedProvider> FeedReader<P> {
    pub fn from_provider(provider: P) -> Self {
        FeedReader {
            provider: provider,
            builder: csv::ReaderBuilder::new(),
        }
    }

    pub fn builder(&mut self) -> &mut csv::ReaderBuilder {
        &mut self.builder
    }

    pub fn agencies(&self) -> Result<GTFSIterator<File, Agency>, Error> {
        self.make_iterator("agency.txt")
    }

    pub fn stops(&self) -> Result<GTFSIterator<File, Stop>, Error> {
        self.make_iterator("stops.txt")
    }

    pub fn routes(&self) -> Result<GTFSIterator<File, Route>, Error> {
        self.make_iterator("routes.txt")
    }

    pub fn trips(&self) -> Result<GTFSIterator<File, Trip>, Error> {
        self.make_iterator("trips.txt")
    }

    pub fn stop_times(&self) -> Result<GTFSIterator<File, StopTime>, Error> {
        self.make_iterator("stop_times.txt")
    }

    pub fn calendars(&self) -> Result<GTFSIterator<File, Calendar>, Error> {
        self.make_iterator("calendar.txt")
    }

    pub fn calendar_dates(&self) -> Result<GTFSIterator<File, CalendarDate>, Error> {
        self.make_iterator("calendar_dates.txt")
    }

    pub fn fare_attributes(&self) -> Result<GTFSIterator<File, FareAttribute>, Error> {
        self.make_iterator("fare_attributes.txt")
    }

    pub fn fare_rules(&self) -> Result<GTFSIterator<File, FareRule>, Error> {
        self.make_iterator("fare_rules.txt")
    }

    pub fn shapes(&self) -> Result<GTFSIterator<File, ShapePoint>, Error> {
        self.make_iterator("shapes.txt")
    }

    pub fn frequencies(&self) -> Result<GTFSIterator<File, Frequency>, Error> {
        self.make_iterator("frequencies.txt")
    }

    pub fn transfers(&self) -> Result<GTFSIterator<File, Transfer>, Error> {
        self.make_iterator("transfers.txt")
    }

    pub fn feed_info(&self) -> Result<GTFSIterator<File, FeedInfo>, Error> {
        self.make_iterator("feed_info.txt")
    }

    fn make_iterator<T>(&self, filename: &str) -> Result<GTFSIterator<File, T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = match Path::new(&self.provider.path()).join(filename).to_str() {
            Some(path_str) => path_str.to_string(),
            None => {
                return Err(Error::Feed(format!(
                    "failed to construct path from {} and {}",
                    self.provider.path(),
                    filename
                )))
            }
        };
        let reader = match self.builder.from_path(&path) {
            Ok(reader) => reader,
            Err(e) => return Err(Error::Csv(path, e)),
        };
        Ok(GTFSIterator::new(reader, &path)?)
    }
}
