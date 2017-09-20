extern crate transitfeed;
extern crate quick_csv;

use quick_csv::Csv;
use transitfeed::{GTFSIterator, agencies, calendars, calendar_dates, frequencies, routes, shapes, stops, stop_times, trips};

    #[test]
    fn test_read_agencies() {
        let csv = Csv::from_file("./examples/bench/agency.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "agency.txt".to_string(), agencies::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_calendar_dates() {
        let csv = Csv::from_file("./examples/bench/calendar_dates.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "calendar_dates.txt".to_string(), calendar_dates::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_calendar() {
        let csv = Csv::from_file("./examples/bench/calendar.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "calendar.txt".to_string(), calendars::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_frequencies() {
        let csv = Csv::from_file("./examples/bench/frequencies.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "frequencies.txt".to_string(), frequencies::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_routes() {
        let csv = Csv::from_file("./examples/bench/routes.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "routes.txt".to_string(), routes::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_shapes() {
        let csv = Csv::from_file("./examples/bench/shapes.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "shapes.txt".to_string(), shapes::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_stops() {
        let csv = Csv::from_file("./examples/bench/stops.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "stops.txt".to_string(), stops::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }

    #[test]
    fn test_read_stop_times() {
        let csv = Csv::from_file("./examples/bench/stop_times.txt").unwrap();
        /*let csv = Csv::from_string("trip_id,arrival_time,departure_time,stop_id,stop_sequence,stop_headsign,pickup_type,drop_off_time,shape_dist_traveled
                                    STBA,6:00:00,6:00:00,STAGECOACH,1,,,,
                                    STBA,6:20:00,6:20:00,BEATTY_AIRPORT,2,,,,
                                    CITColumns1,6:00:00,6:00:00,STAGECOACH,1,,,,
                                    CITColumns1,6:05:00,6:07:00,NANAA,2,,,,
                                    CITColumns1,6:12:00,6:14:00,NADAV,3,,,,
                                    CITColumns1,6:19:00,6:21:00,DADAN,4,,,,
                                    CITColumns1,6:26:00,6:28:00,EMSI,5,,,,
                                    CITColumns2,6:28:00,6:30:00,EMSI,1,,,,
                                    CITColumns2,6:35:00,6:37:00,DADAN,2,,,,
                                    CITColumns2,6:42:00,6:44:00,NADAV,3,,,,
                                    CITColumns2,6:49:00,6:51:00,NANAA,4,,,,
                                    CITColumns2,6:56:00,6:58:00,STAGECOACH,5,,,,
                                    AB1,8:00:00,8:00:00,BEATTY_AIRPORT,1,,,,
                                    AB1,8:10:00,8:15:00,BULLFROG,2,,,,
                                    AB2,12:05:00,12:05:00,BULLFROG,1,,,,
                                    AB2,12:15:00,12:15:00,BEATTY_AIRPORT,2
                                    BFC1,8:20:00,8:20:00,BULLFROG,1
                                    BFC1,9:20:00,9:20:00,FUR_CREEK_RES,2
                                    BFC2,11:00:00,11:00:00,FUR_CREEK_RES,1
                                    BFC2,12:00:00,12:00:00,BULLFROG,2
                                    AAMV1,8:00:00,8:00:00,BEATTY_AIRPORT,1
                                    AAMV1,9:00:00,9:00:00,AMV,2");
                                    */
        let mut iterator = GTFSIterator::new(csv, "stop_times.txt".to_string(), stop_times::parse_row).unwrap();
        // example data has bad entries in the bottom
        for entry in iterator.by_ref().take(15) {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
        for entry in iterator.by_ref() {
            assert!(entry.is_err(), format!("{:?}", entry.unwrap()));
        }
    }

    #[test]
    fn test_read_trips() {
        let csv = Csv::from_file("./examples/bench/trips.txt").unwrap();
        let iterator = GTFSIterator::new(csv, "trips.txt".to_string(), trips::parse_row).unwrap();
        for entry in iterator {
            assert!(entry.is_ok(), format!("{}", entry.err().unwrap()));
        }
    }
