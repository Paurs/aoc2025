use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;

    Ok(lines)
}

pub fn read_example(data: &str) -> Vec<&str> {
    data.lines().collect::<Vec<&str>>()
}
