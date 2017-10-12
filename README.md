# Transit

Fast transit library for Rust that provides GTFS serialization/deserialization,
validation, and manipulation.

## Usage

```rust
extern crate transitfeed;
extern crate quick_csv;

use quick_csv::Csv;
use transitfeed::{GTFSIterator, agencies};

fn read_agencies() {
    let csv = Csv::from_file("/tmp/my_feed/agency.txt").unwrap();
    let iterator = GTFSIterator::new(csv, "agency.txt".to_string(), agencies::parse_row).unwrap();
    for result in iterator {
        match result {
            Ok(entry) => println!("{:?}", entry),
            Err(err) => println!("{}", err),
        };
    }
}
```
