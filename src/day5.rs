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
    mappings: Vec<Mapping>,
}

impl Map {
    fn from_vec(lines: Vec<String>) -> Self {
        let mut mappings: Vec<Mapping> = Vec::new();

        for line in lines[1..].iter() {
            mappings.push(Mapping::from_str(line).unwrap()); 
        }

        return Map {mappings};
    }

    fn map(&self, n: u64) -> u64 {
        let mut num = n;
        for mapping in self.mappings.iter() {
            num = mapping.map(num); 
        }

        return num;
    }
}

struct Chain {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Chain {
    fn from_blocks(blocks: Vec<Vec<String>>) -> Self {
        let seeds_string = blocks[0][0].clone(); 
        let seeds_strs = &seeds_string.split(" ").collect::<Vec<&str>>()[1..];
        let mut seeds: Vec<u64> = Vec::new();
        for seed in seeds_strs.iter() {
            seeds.push(seed.parse::<u64>().unwrap()); 
        }

        return Chain { seeds, maps: vec![] };
    }
}

fn main() {
    let data = aoc_tools::read_blocks::<String>("data/day5.txt").unwrap();
    let chain = Chain::from_blocks(data);

    println!("{:?}", chain.seeds);
}
