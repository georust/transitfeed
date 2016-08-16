#![feature(test)]
extern crate transitfeed;
extern crate test;
extern crate quick_csv;

use std::fmt::{Debug, Display};
use std::fs;
use std::io::Read;
use test::Bencher;
use quick_csv::Csv;
use transitfeed::{AgencyIterator, StopIterator, StopTimeIterator};

static AGENCY_DATA: &'static str = "./examples/agency.txt";
static STOP_DATA: &'static str = "./examples/stop_times.txt";
static STOP_TIMES_DATA: &'static str = "./examples/stop_times.txt";

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
fn bench_agency_decoder(b: &mut Bencher) {
    let data = file_to_mem(AGENCY_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Csv::from_reader(&*data);
        let decoder = AgencyIterator::new(csv).unwrap();
        for agency in decoder {
            let _ = agency;
        }
    })
}

#[bench]
fn bench_stop_decoder(b: &mut Bencher) {
    let data = file_to_mem(STOP_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Csv::from_reader(&*data);
        let decoder = StopIterator::new(csv).unwrap();
        for stop in decoder {
            let _ = stop;
        }
    })
}

#[bench]
fn bench_stop_time_decoder(b: &mut Bencher) {
    let data = file_to_mem(STOP_TIMES_DATA);
    b.bytes = data.len() as u64;
    b.iter(|| {
        let csv = Csv::from_reader(&*data);
        let decoder = StopTimeIterator::new(csv).unwrap();
        for stop_time in decoder {
            let _ = stop_time;
        }
    })
}
