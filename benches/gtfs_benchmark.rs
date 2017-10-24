#![feature(test)]
extern crate transitfeed;
extern crate test;
extern crate csv;

use std::fmt::{Debug, Display};
use std::fs;
use std::io::Read;
use test::Bencher;
use csv::Reader;
use transitfeed::{GTFSIterator, Agency, Calendar, CalendarDate, Frequency, Route, Shape, Stop, StopTime, Trip, FareRule, FareAttribute, Transfer};

const AGENCY_DATA: &'static str = "./examples/bench/agency.txt";
const CALENDAR_DATA: &'static str = "./examples/bench/calendar.txt";
const CALENDAR_DATE_DATA: &'static str = "./examples/bench/calendar_dates.txt";
const ROUTE_DATA: &'static str = "./examples/bench/routes.txt";
const SHAPE_DATA: &'static str = "./examples/bench/shapes.txt";
const STOP_DATA: &'static str = "./examples/bench/stops.txt";
const STOP_TIMES_DATA: &'static str = "./examples/bench/stop_times.txt";
const TRIP_DATA: &'static str = "./examples/bench/trips.txt";
const FREQUENCY_DATA: &'static str = "./examples/bench/frequencies.txt";
const FARE_RULES_DATA: &'static str = "./examples/bench/fare_rules.txt";
const FARE_ATTRIBUTES_DATA: &'static str = "./examples/bench/fare_attributes.txt";
const TRANSFERS_DATA: &'static str = "./examples/good_feed/transfers.txt";

fn or_die<T, E: Debug+Display>(r: Result<T, E>) -> T {
    r.or_else(|e: E| -> Result<T, E> { panic!(format!("{:?}", e)) }).unwrap()
}

fn file_to_mem(fp: &str) -> Vec<u8> {
    let mut f = or_die(fs::File::open(fp));
    let mut bs = vec![];
    or_die(f.read_to_end(&mut bs));
    bs
}

#[bench]
fn bench_agency_iterator(b: &mut Bencher) {
    let data = file_to_mem(AGENCY_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Agency> = GTFSIterator::new(csv, "agency.txt").unwrap();
        for agency in iterator {
            let _ = agency;
        }
    })
}

#[bench]
fn bench_calendar_iterator(b: &mut Bencher) {
    let data = file_to_mem(CALENDAR_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Calendar> = GTFSIterator::new(csv, "calendar.txt").unwrap();
        for calendar in iterator {
            let _ = calendar;
        }
    })
}

#[bench]
fn bench_calendar_date_iterator(b: &mut Bencher) {
    let data = file_to_mem(CALENDAR_DATE_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, CalendarDate> = GTFSIterator::new(csv, "calendar_dates.txt").unwrap();
        for calendar_date in iterator {
            let _ = calendar_date;
        }
    })
}

#[bench]
fn bench_frequency_iterator(b: &mut Bencher) {
    let data = file_to_mem(FREQUENCY_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Frequency> = GTFSIterator::new(csv, "frequencies.txt").unwrap();
        for freq in iterator {
            let _ = freq;
        }
    })
}

#[bench]
fn bench_route_iterator(b: &mut Bencher) {
    let data = file_to_mem(ROUTE_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Route> = GTFSIterator::new(csv, "routes.txt").unwrap();
        for route in iterator {
            let _ = route;
        }
    })
}

#[bench]
fn bench_shape_iterator(b: &mut Bencher) {
    let data = file_to_mem(SHAPE_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Shape> = GTFSIterator::new(csv, "shapes.txt").unwrap();
        for shape in iterator {
            let _ = shape;
        }
    })
}

#[bench]
fn bench_stop_iterator(b: &mut Bencher) {
    let data = file_to_mem(STOP_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Stop> = GTFSIterator::new(csv, "stops.txt").unwrap();
        for stop in iterator {
            let _ = stop;
        }
    })
}

#[bench]
fn bench_stop_time_iterator(b: &mut Bencher) {
    let data = file_to_mem(STOP_TIMES_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, StopTime> = GTFSIterator::new(csv, "stop_times.txt").unwrap();
        for stop_time in iterator {
            let _ = stop_time;
        }
    })
}

#[bench]
fn bench_trip_iterator(b: &mut Bencher) {
    let data = file_to_mem(TRIP_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Trip> = GTFSIterator::new(csv, "trips.txt").unwrap();
        for trip in iterator {
            let _ = trip;
        }
    })
}

#[bench]
fn bench_fare_rules_iterator(b: &mut Bencher) {
    let data = file_to_mem(FARE_RULES_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, FareRule> = GTFSIterator::new(csv, "fare_rules.txt").unwrap();
        for rule in iterator {
            let _ = rule;
        }
    })
}

#[bench]
fn bench_fare_attributes_iterator(b: &mut Bencher) {
    let data = file_to_mem(FARE_ATTRIBUTES_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, FareAttribute> = GTFSIterator::new(csv, "fare_attributes.txt").unwrap();
        for attribute in iterator {
            let _ = attribute;
        }
    })
}

#[bench]
fn bench_transfers_iterator(b: &mut Bencher) {
    let data = file_to_mem(TRANSFERS_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Reader::from_reader(&*data);
        let iterator : GTFSIterator<_, Transfer> = GTFSIterator::new(csv, "transfers.txt").unwrap();
        for attribute in iterator {
            let _ = attribute;
        }
    })
}
