use std::str::FromStr; 
use std::string::ParseError;

use aoc_tools;

struct Mapping {
    seed_start: u64,
    dest_start: u64,
    length: u64,
}

impl FromStr for Mapping {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(" ").collect::<Vec<&str>>();
        let seed_start = tokens[1].parse::<u64>().unwrap();
        let dest_start = tokens[0].parse::<u64>().unwrap();
        let length = tokens[2].parse::<u64>().unwrap();

        return Ok(Mapping { seed_start, dest_start, length });
    }
}

impl Default for Mapping {
    fn default() -> Self {
        return Mapping { seed_start: 0, dest_start: 0, length: 0 };
    }
}

impl Mapping {
    fn map(&self, n: u64) -> u64 {
        if n >= self.seed_start && n < (self.seed_start + self.length) {
            return n - self.seed_start + self.dest_start;
        } else {
            return n;
        }
    }
}

struct Map {
    maps: Vec<Mapping>,
}

impl Map {
    fn from_vec(lines: Vec<String>) -> Self {
        let mut maps: Vec<Mapping> = Vec::new();

        for line in lines[1..].iter() {
            maps.push(Mapping::from_str(line).unwrap()); 
        }

        return Map {maps};
    }

    fn map(&self, n: u64) -> u64 {
        let mut num = n;
        for mapping in self.maps.iter() {
            num = mapping.map(num); 
        }

        return num;
    }
}

fn main() {
    let data = aoc_tools::read_lines::<String>("data/day5_map.txt").unwrap();
    let map = Map::from_vec(data);

    println!("{}", map.map(237273108));
}
