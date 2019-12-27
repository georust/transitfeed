//! Transit Feed provides a consistent set of data structures, parsers,
//! and API clients for obtaining usable transit related information
//! such as routes, stop, trips, stop times, and more.
mod archive;
pub mod feed;
mod gtfs;
mod transit;

pub use feed::{FeedReader, Terminator, Trim};
pub use gtfs::{Error, GTFSIterator};
pub use transit::*;
