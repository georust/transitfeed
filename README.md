# Transit

Fast transit library for Rust that provides GTFS serialization/deserialization,
validation, and manipulation.

## Usage

For files on your file system just us `GTFSIterator::from_path`

```rust
extern crate transitfeed;
use transitfeed::{GTFSIterator, Agency};

fn read_agencies() {
    let iterator : GTFSIterator<_, Agency> = GTFSIterator::from_path("~/Downloads/gtfs/agency.txt").unwrap();
    for result in iterator {
        match result {
            Ok(entry) => println!("{:?}", entry),
            Err(err) => println!("{}", err),
        };
    }
}
```

If you have your own `csv::Reader` then just give `GTFSIterator::new` a meaningful name
```rust
let iterator : GTFSIterator<_, Agency> = GTFSIterator::new(reader, "example_data").unwrap();
for result in iterator {
    match result {
        Ok(entry) => println!("{:?}", entry),
        Err(err) => println!("{}", err),
    };
}
```
