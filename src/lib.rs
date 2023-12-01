use std::str::FromStr;
use std::string::ParseError;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_lines<T: FromStr + Default>(filename: &str) -> Result<Vec<T>, ParseError> {
    let file = File::open(filename).expect("couldn't read file!");
    let reader = BufReader::new(file);

    Ok(reader.lines()
             .map(|line| line.expect("error parsing file").parse::<T>().unwrap_or_default())
             .collect::<Vec<T>>()
    )
}
