extern crate transitfeed;
use transitfeed::{Agency, Calendar, CalendarDate, FareAttribute, FareRule, FeedInfo, Frequency,
                  GTFSIterator, Route, ShapePoint, Stop, StopTime, Transfer, Trip};

#[test]
fn test_read_agencies() {
    let iter: GTFSIterator<_, Agency> =
        GTFSIterator::from_path("./examples/good_feed/agency.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_stops() {
    let iter: GTFSIterator<_, Stop> =
        GTFSIterator::from_path("./examples/good_feed/stops.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_routes() {
    let iter: GTFSIterator<_, Route> =
        GTFSIterator::from_path("./examples/good_feed/routes.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_trips() {
    let iter: GTFSIterator<_, Trip> =
        GTFSIterator::from_path("./examples/good_feed/trips.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_stop_times() {
    let iter: GTFSIterator<_, StopTime> =
        GTFSIterator::from_path("./examples/good_feed/stop_times.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_calendar() {
    let iter: GTFSIterator<_, Calendar> =
        GTFSIterator::from_path("./examples/good_feed/calendar.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_calendar_dates() {
    let iter: GTFSIterator<_, CalendarDate> =
        GTFSIterator::from_path("./examples/good_feed/calendar_dates.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_fare_attributes() {
    let iter: GTFSIterator<_, FareAttribute> =
        GTFSIterator::from_path("./examples/good_feed/fare_attributes.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_fare_rules() {
    let iter: GTFSIterator<_, FareRule> =
        GTFSIterator::from_path("./examples/good_feed/fare_rules.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_shapes() {
    let iter: GTFSIterator<_, ShapePoint> =
        GTFSIterator::from_path("./examples/good_feed/shapes.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_frequencies() {
    let iter: GTFSIterator<_, Frequency> =
        GTFSIterator::from_path("./examples/good_feed/frequencies.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_transfers() {
    let iter: GTFSIterator<_, Transfer> =
        GTFSIterator::from_path("./examples/good_feed/transfers.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_feed_info() {
    let iter: GTFSIterator<_, FeedInfo> =
        GTFSIterator::from_path("./examples/good_feed/feed_info.txt").unwrap();
    for result in iter {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}
