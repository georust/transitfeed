//! TransitFeed provides a consistent set of datastructures, parsers,
//! and API clients for obtaining usable transit related information
//! such as routes, stop, trips, stop times, and more.

#[macro_use]
extern crate zip;
extern crate quick_csv;
extern crate chrono;

mod transit;
mod gtfs;
mod error;

pub use transit::*;
pub use gtfs::{AgencyDecoder, StopDecoder, StopTimeDecoder};
