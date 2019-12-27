use super::error::Error;
use csv::{
    DeserializeError, DeserializeRecordsIntoIter, ErrorKind, Position, Reader, StringRecord,
};
use serde;
use std;

pub struct GTFSIterator<R, T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    iter: DeserializeRecordsIntoIter<R, T>,
    headers: StringRecord,
    filename: String,
}

impl<T> GTFSIterator<std::fs::File, T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn from_path(filename: &str) -> Result<GTFSIterator<std::fs::File, T>, Error> {
        let csv = match Reader::from_path(filename) {
            Ok(c) => c,
            Err(e) => return Err(Error::Csv(filename.to_string(), e)),
        };
        GTFSIterator::new(csv, filename)
    }
}

impl<R, T> GTFSIterator<R, T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    pub fn new(mut reader: Reader<R>, filename: &str) -> Result<GTFSIterator<R, T>, Error> {
        let headers = match reader.headers() {
            Ok(r) => r.clone(),
            Err(e) => return Err(Error::Csv(filename.to_string(), e)),
        };
        Ok(GTFSIterator {
            iter: reader.into_deserialize(),
            headers: headers,
            filename: filename.to_string(),
        })
    }

    fn wrap_fielderror(&self, err: &DeserializeError, position: &Option<Position>) -> Error {
        let fieldname = match err.field() {
            Some(field_pos) => Some(match self.headers.get(field_pos as usize) {
                Some(field) => field.to_string(),
                None => format!("field {}", field_pos).to_string(),
            }),
            None => None,
        };
        // TODO:: What if position.line() is None?
        Error::FieldError(
            String::clone(&self.filename),
            position.as_ref().unwrap().line(),
            err.kind().clone(),
            fieldname,
        )
    }
}

impl<R, T> Iterator for GTFSIterator<R, T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Result<T, Error>> {
        match self.iter.next() {
            Some(r) => Some(match r {
                Err(e) => match e.into_kind() {
                    ErrorKind::Deserialize { ref pos, ref err } => {
                        Err(self.wrap_fielderror(err, pos))
                    }
                    k => Err(Error::LineError(String::clone(&self.filename), k)),
                },
                Ok(s) => Ok(s),
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gtfs::parse::*;
    use csv;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        foo: String,
        bar: f64,
        #[serde(deserialize_with = "deserialize_dow_field")] // makes 0 or 1 into bool
        baz: bool,
    }

    #[test]
    fn test_parse_records() {
        let data = "\
foo,bar,baz
Foo,1.0,0
";
        let expected = Test {
            foo: "Foo".to_string(),
            bar: 1.0,
            baz: false,
        };

        let reader = csv::Reader::from_reader(data.as_bytes());
        let mut iter: GTFSIterator<_, Test> = GTFSIterator::new(reader, "test.txt").unwrap();
        assert_eq!(&expected, iter.next().unwrap().as_ref().unwrap());
    }

    #[test]
    fn test_error_parsing_primitive_fields() {
        let data = "\
foo,bar,baz
Foo,w,0
";
        let expected = "error parsing bar in test.txt:2 - invalid float literal";

        let reader = csv::Reader::from_reader(data.as_bytes());
        let mut iter: GTFSIterator<_, Test> = GTFSIterator::new(reader, "test.txt").unwrap();

        let result = iter.next().unwrap().err().unwrap();
        assert_eq!(expected, format!("{}", result));
    }

    #[test]
    fn test_error_parsing_custom_fields() {
        let data = "\
foo,bar,baz
Foo,1,3
";
        // ugly: Can deserialize_dow_field take a DeRecordWrap instead to add field info?
        let expected = "error parsing test.txt:2 - day of week field was not 0 or 1";

        let reader = csv::Reader::from_reader(data.as_bytes());
        let mut iter: GTFSIterator<_, Test> = GTFSIterator::new(reader, "test.txt").unwrap();

        let result = iter.next().unwrap().err().unwrap();
        assert_eq!(expected, format!("{}", result));
    }

    #[test]
    fn test_error_parsing_bad_lines() {
        let data = "\
foo,bar,baz
Foo,1
";
        // ugly: Can deserialize_dow_field take a DeRecordWrap instead to add field info?
        let expected = "error parsing test.txt:2 - expected 3 fields but got 2 fields";

        let reader = csv::Reader::from_reader(data.as_bytes());
        let mut iter: GTFSIterator<_, Test> = GTFSIterator::new(reader, "test.txt").unwrap();
        let result = iter.next().unwrap().err().unwrap();
        assert_eq!(expected, format!("{}", result));
    }

    #[test]
    fn test_error_file_missing() {
        let result: Result<GTFSIterator<_, Test>, Error> =
            GTFSIterator::from_path("./examples/definitelynothere");
        assert_eq!(
            "error parsing ./examples/definitelynothere - No such file or directory (os error 2)",
            format!("{}", result.err().unwrap())
        )
    }
}
