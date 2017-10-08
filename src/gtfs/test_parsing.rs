#[cfg(test)]
pub mod test {
    use std::iter::Zip;
    use std::slice::Iter;
    use quick_csv::columns::Columns;
    use gtfs::error::ParseError;

    pub fn parse_row_harness<T, F: (Fn(Zip<Iter<String>, Columns>) -> Result<T, ParseError>)>(headers: Vec<&str>, values: Vec<&str>, parse_fn : F) -> Result<T, ParseError> {
        let header = make_header(headers);
        let (line, indices) = make_values(values);
        let column = Columns::new(&line, &indices);
        let row = make_row(&header, column);
        parse_fn(row)
    }


    fn make_row<'a, 'b>(headers: &'a Vec<String>, columns: Columns<'b>) -> Zip<Iter<'a, String>, Columns<'b>> {
        headers.iter().zip(columns)
    }

    fn make_header(headers: Vec<&str>) -> Vec<String> {
        headers.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    }

    fn make_values(values: Vec<&str>) -> (String, Vec<usize>) {
        let line = values.join(",").to_string();
        // indices to the delimiter positions
        let mut indices : Vec<usize> = vec!();
        for value in values.iter() {
            let last_index = match indices.last() {
                Some(x) => x.clone() as i32,
                None => -1,
            };
            indices.push((last_index + value.len() as i32 + 1) as usize)
        }
        (line, indices)
    }
}
