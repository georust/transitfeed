#![feature(test)]
extern crate transitfeed;
extern crate test;
extern crate csv;

use std::fmt::{Debug, Display};
use std::fs;
use std::io::Read;
use test::Bencher;
use csv::{ReaderBuilder, Reader};
use transitfeed::{GTFSIterator, Agency, Calendar, CalendarDate, Frequency, Route, Shape, Stop, StopTime, Trip, FareRule, FareAttribute};

static AGENCY_DATA: &'static str = include_str!("../examples/bench/agency.txt");
static CALENDAR_DATA: &'static str = include_str!("../examples/bench/calendar.txt");
static CALENDAR_DATE_DATA: &'static str = include_str!("../examples/bench/calendar_dates.txt");
static ROUTE_DATA: &'static str = include_str!("../examples/bench/routes.txt");
static SHAPE_DATA: &'static str = include_str!("../examples/bench/shapes.txt");
static STOP_DATA: &'static str = include_str!("../examples/bench/stops.txt");
static STOP_TIMES_DATA: &'static str = include_str!("../examples/bench/stop_times.txt");
static TRIP_DATA: &'static str = include_str!("../examples/bench/trips.txt");
static FREQUENCY_DATA: &'static str = include_str!("../examples/bench/frequencies.txt");
static FARE_RULES_DATA: &'static str = include_str!("../examples/bench/fare_rules.txt");
static FARE_ATTRIBUTES_DATA: &'static str = include_str!("../examples/bench/fare_attributes.txt");

macro_rules! bench_deserialize {
    ($name:ident, $data:ident, $typ:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let data = $data.as_bytes();
            b.bytes = data.len() as u64;
            b.iter(|| {
                let mut csv = ReaderBuilder::new()
                    .has_headers(true)
                    .from_reader(data);
                for row in csv.deserialize() {
                    let _row: $typ = row.unwrap();
                }
            })
        }
    };
}

bench_deserialize!(agency, AGENCY_DATA, Agency);
bench_deserialize!(calendar, CALENDAR_DATA, Calendar);
bench_deserialize!(calendar_date, CALENDAR_DATE_DATA, CalendarDate);
bench_deserialize!(route, ROUTE_DATA, Route);
bench_deserialize!(shape, SHAPE_DATA, Shape);
bench_deserialize!(stop, STOP_DATA, Stop);
//bench_deserialize!(stop_times, STOP_TIMES_DATA, StopTime);
bench_deserialize!(trip, TRIP_DATA, Trip);
bench_deserialize!(frequency, FREQUENCY_DATA, Frequency);
bench_deserialize!(fare_rules, FARE_RULES_DATA, FareRule);
bench_deserialize!(fare_attributes, FARE_ATTRIBUTES_DATA, FareAttribute);
