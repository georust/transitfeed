extern crate csv;
extern crate transitfeed;

use transitfeed::{FeedReader, Trim};

#[test]
fn test_read_feed_with_reader_options() {
    let mut feed = FeedReader::new("./examples/invalid_csv");
    feed.builder().delimiter(b';').trim(Trim::All);

    for result in feed.stops().unwrap() {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_read_zipfiles_with_feed() {
    let feed = FeedReader::from_zip("./examples/good_feed.zip").unwrap();

    for result in feed.stops().unwrap() {
        assert!(result.is_ok(), format!("{}", result.err().unwrap()));
    }
}

#[test]
fn test_fail_bad_zipfiles() {
    let result = FeedReader::from_zip("./examples/unknown_format.zip");
    assert!(result.is_err());
    assert_eq!(
        "error in feed - Invalid Zip archive: Invalid zip header",
        format!("{}", result.err().unwrap()),
        "Error didn't match"
    );
}
