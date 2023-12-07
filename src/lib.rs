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

pub fn read_blocks<T: FromStr + Default>(filename: &str) -> Result<Vec<Vec<T>>, ParseError> {
    let file = File::open(filename).expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut whole: Vec<Vec<T>> = Vec::from(Vec::new());
    let mut current: Vec<T> = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if &l == "" {
                whole.push(current);
                current = vec![];
            } else {
                current.push(l.clone().parse::<T>().unwrap_or_default());
            }
        } else {
            return Ok(vec![vec![]]);
        }
    }

    return Ok(whole);
}
