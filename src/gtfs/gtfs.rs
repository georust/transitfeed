use gtfs::error::Error;
use csv::{DeserializeRecordsIntoIter, StringRecord, Reader, ErrorKind, DeserializeError, Position};
use serde;
use std;

pub struct GTFSIterator<R, T>
    where R: std::io::Read,
          T: serde::de::DeserializeOwned
{
    iter: DeserializeRecordsIntoIter<R, T>,
    headers: StringRecord,
    filename: String,
}

impl<T> GTFSIterator<std::fs::File, T>
    where T: serde::de::DeserializeOwned
{
    pub fn from_path(filename: &str) -> Result<GTFSIterator<std::fs::File, T>, Error>
    {
        let csv = match Reader::from_path(filename) {
            Ok(c) => c,
            Err(e) => return Err(Error::Csv(filename.to_string(), e))
        };
        GTFSIterator::new(csv, filename)
    }
}

impl<R, T> GTFSIterator<R, T>
    where R: std::io::Read,
          T: serde::de::DeserializeOwned
{
    pub fn new(mut reader: Reader<R>, filename: &str) -> Result<GTFSIterator<R, T>, Error> {
        let headers = match reader.headers() {
            Ok(r) => r.clone(),
            Err(e) => return Err(Error::Csv(filename.to_string(), e))
        };
        Ok(GTFSIterator{
            iter: reader.into_deserialize(),
            headers: headers,
            filename: filename.to_string()
        })
    }

    fn wrap_fielderror(&self, err: &DeserializeError, position: &Option<Position>) -> Error {
        let fieldname = match err.field() {
            Some(field_pos) => Some(match self.headers.get(field_pos as usize) {
                Some(field) => field.to_string(),
                None => format!("field {}", field_pos).to_string()
            }),
            None => None
        };
        // TODO:: What if position.line() is None?
        Error::FieldError(String::clone(&self.filename), position.as_ref().unwrap().line(), err.kind().clone(), fieldname)
    }
}

impl<R, T> Iterator for GTFSIterator<R, T>
    where R: std::io::Read,
          T: serde::de::DeserializeOwned
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Result<T, Error>> {
        match self.iter.next() {
            Some(r) => Some(match r {
                Err(e) => match e.into_kind() {
                    ErrorKind::Deserialize{ref pos, ref err} => Err(self.wrap_fielderror(err, pos)),
                    k => Err(Error::LineError(String::clone(&self.filename), k))
                },
                Ok(s) => Ok(s)
            }),
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use csv;
    use super::*;
    use gtfs::parse::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        foo: String,
        bar: f64,
        #[serde(deserialize_with = "deserialize_dow_field")]  // makes 0 or 1 into bool
        baz: bool
    }

    #[test]
    fn test_parse_records() {

        let data = "\
foo,bar,baz
Foo,1.0,0
";
        let expected = Test { foo: "Foo".to_string(), bar: 1.0, baz: false };

        let reader = csv::Reader::from_reader(data.as_bytes());
        let mut iter : GTFSIterator<_, Test> = GTFSIterator::new(reader, "test.txt").unwrap();
        assert_eq!(&expected, iter.next().unwrap().as_ref().unwrap());
    }

    #[test]
    fn test_error_parsing_primitives() {

        let data = "\
foo,bar,baz
Foo,w,0
";
        let expected = Test { foo: "Foo".to_string(), bar: 1.0, baz: false };

        let reader = csv::Reader::from_reader(data.as_bytes());
        let mut iter : GTFSIterator<_, Test> = GTFSIterator::new(reader, "test.txt").unwrap();
        assert_eq!(&expected, iter.next().unwrap().as_ref().unwrap());
    }

    #[test]
    fn test_error_file_missing() {
        let result : Result<GTFSIterator<_, Test>, Error> = GTFSIterator::from_path("./examples/definitelynothere");
        // TODO: make sure its an IO error
        assert!(result.is_err());
    }

    // TODO: raw csv serde errors are kind of obscure, try to make them better
}
