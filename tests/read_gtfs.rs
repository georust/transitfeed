extern crate transitfeed;

use transitfeed::{GTFSIterator, Agency, Calendar, CalendarDate, Frequency, Route, Shape, Stop, StopTime, Trip, FareRule, FareAttribute};

#[test]
fn test_read_agencies() {
    let iter : GTFSIterator<_, Agency> = GTFSIterator::from_path("./examples/bench/agency.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_calendar_dates() {
    let iter : GTFSIterator<_, CalendarDate> = GTFSIterator::from_path("./examples/bench/calendar_dates.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_calendar() {
    let iter : GTFSIterator<_, Calendar> = GTFSIterator::from_path("./examples/bench/calendar.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_frequencies() {
    let iter : GTFSIterator<_, Frequency> = GTFSIterator::from_path("./examples/bench/frequencies.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_routes() {
    let iter : GTFSIterator<_, Route> = GTFSIterator::from_path("./examples/bench/routes.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_shapes() {
    let iter : GTFSIterator<_, Shape> = GTFSIterator::from_path("./examples/bench/shapes.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_stops() {
    let iter : GTFSIterator<_, Stop> = GTFSIterator::from_path("./examples/bench/stops.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_stop_times() {
    let mut iter : GTFSIterator<_, StopTime> = GTFSIterator::from_path("./examples/bench/stop_times.txt").unwrap();
    for result in iter.by_ref().take(15) {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
    for result in iter.by_ref() {
        assert!(result.is_err(), format!("{:?}", result.unwrap()));
    }
}

#[test]
fn test_read_trips() {
    let iter : GTFSIterator<_, Trip> = GTFSIterator::from_path("./examples/bench/trips.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_fare_rules() {
    let iter : GTFSIterator<_, FareRule> = GTFSIterator::from_path("./examples/bench/fare_rules.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_fare_attributes() {
    let iter : GTFSIterator<_, FareAttribute> = GTFSIterator::from_path("./examples/bench/fare_attributes.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}
