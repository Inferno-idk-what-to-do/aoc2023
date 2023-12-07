use std::str::FromStr; 
use std::string::ParseError;

use aoc_tools;

#[derive(Debug)]
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

#[derive(Debug)]
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
        for mapping in self.mappings.iter() {
            if mapping.map(n) != n {
                return mapping.map(n);
            }
        }

        return n;
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

        let map_blocks = &blocks[1..];
        let mut maps: Vec<Map> = Vec::new();
        for map in map_blocks {
            maps.push(Map::from_vec(map.to_vec())); 
        }

        return Chain { seeds, maps };
    }

    fn map_all(&self) -> Vec<u64> {
        let mut mapped = self.seeds.clone();

        for map in self.maps.iter() {
            let mut current_mapped = Vec::new();
            
            for n in mapped.iter() {
                current_mapped.push(map.map(*n));
            }

            mapped = current_mapped;
        }

        return mapped;
    }

    fn smallest(&self) -> u64 {
        *self.map_all().iter().min().unwrap()
    }
}

fn main() {
    let data = aoc_tools::read_blocks::<String>("data/day5.txt").unwrap();
    let chain = Chain::from_blocks(data);

    let part1 = chain.smallest();
    println!("{}", part1);
}
