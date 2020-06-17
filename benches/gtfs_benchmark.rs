use csv::Reader;
use std::fmt::{Debug, Display};
use std::fs;
use std::io::Read;
use transitfeed::{
    Agency, Calendar, CalendarDate, FareAttribute, FareRule, FeedInfo, Frequency, GTFSIterator,
    Route, ShapePoint, Stop, StopTime, Transfer, Trip,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT_FOLDER: &'static str = "examples/bench";

criterion_group!(gtfs, bench_feed_throughput,);
criterion_main!(gtfs);

fn bench_feed_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    bench_file_throughput::<Agency>("agency.txt", &mut group);
    bench_file_throughput::<Stop>("stops.txt", &mut group);
    bench_file_throughput::<Route>("routes.txt", &mut group);
    bench_file_throughput::<Trip>("trips.txt", &mut group);
    bench_file_throughput::<StopTime>("stop_times.txt", &mut group);
    bench_file_throughput::<Calendar>("calendar.txt", &mut group);
    bench_file_throughput::<CalendarDate>("calendar_dates.txt", &mut group);
    bench_file_throughput::<FareAttribute>("fare_attributes.txt", &mut group);
    bench_file_throughput::<ShapePoint>("shapes.txt", &mut group);
    //bench_file_throughput::<Frequency>("frequencies.txt", &mut group);
    //bench_file_throughput::<FeedInfo>("feed_info.txt", &mut group);
}

fn bench_file_throughput<T: for<'de> serde::Deserialize<'de>>(
    file: &str,
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
) {
    // TODO: this makes the benchmark SLOWER somehow?!
    // let data = file_to_mem(AGENCY_DATA); was noticeably faster
    let data = file_to_mem(&format!("{}/{}", INPUT_FOLDER, file));
    group.throughput(criterion::Throughput::Bytes(data.len() as u64));
    group.bench_with_input(
        criterion::BenchmarkId::new(INPUT_FOLDER, file),
        &data,
        |b, i| {
            b.iter(|| {
                let csv = Reader::from_reader(&*data);
                let iterator: GTFSIterator<_, T> = GTFSIterator::new(csv, file).unwrap();
                for thing in iterator {
                    let _ = thing;
                }
            })
        },
    );
}

fn file_to_mem(fp: &str) -> Vec<u8> {
    let mut f = or_die(fs::File::open(fp));
    let mut bs = vec![];
    or_die(f.read_to_end(&mut bs));
    bs
}

fn or_die<T, E: Debug + Display>(r: Result<T, E>) -> T {
    r.or_else(|e: E| -> Result<T, E> { panic!(format!("{:?}", e)) })
        .unwrap()
}
