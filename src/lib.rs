//! Transit Feed provides a consistent set of data structures, parsers,
//! and API clients for obtaining usable transit related information
//! such as routes, stop, trips, stop times, and more.

extern crate chrono;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate zip;

mod transit;
mod gtfs;

pub use transit::*;
pub use gtfs::{Error, GTFSIterator};
