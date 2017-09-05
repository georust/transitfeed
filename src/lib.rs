//! Transit Feed provides a consistent set of data structures, parsers,
//! and API clients for obtaining usable transit related information
//! such as routes, stop, trips, stop times, and more.

extern crate quick_csv;
extern crate chrono;
extern crate zip;

mod transit;
mod gtfs;

pub use transit::*;
pub use gtfs::{AgencyIterator, CalendarIterator, CalendarDateIterator, RouteIterator, ShapeIterator, StopIterator, StopTimeIterator, TripIterator, FrequencyIterator};
