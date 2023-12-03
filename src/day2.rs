use aoc_tools;

use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Game {
    id: i32,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
}

impl Default for Game {
    fn default() -> Self {
        Game { id: 0, max_red: 0, max_green: 0, max_blue: 0 }
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let declaration = s.split(":").collect::<Vec<&str>>();
        let game_id = declaration[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let main_split = declaration[1].split(" ").collect::<Vec<&str>>();
        for i in 0..main_split.len() {
            let s = main_split[i];
            if let Ok(num) = s.parse::<i32>() {
                match main_split[i+1] {
                    "red" | "red," | "red;"=> if num > max_red { max_red = num; },
                    "green" | "green," | "green;" => if num > max_green { max_green = num; },
                    "blue" | "blue," | "blue;" => if num > max_blue { max_blue = num; },
                    _ => (),
                } ;
            }
        }

        Ok(Game { id: game_id, max_red, max_green, max_blue })
    }
}

impl Game {
    fn is_valid(&self, max_red: i32, max_green: i32, max_blue: i32) -> bool {
        self.max_red <= max_red && self.max_green <= max_green && self.max_blue <= max_blue
    }

    fn power(&self) -> i32 {
        self.max_red * self.max_green * self.max_blue
    }
}

fn main() {
    let input = aoc_tools::read_lines::<Game>("data/day2.txt").unwrap();

    let part1: i32 = input.iter().filter(|game| game.is_valid(12, 13, 14)).map(|g| g.id).sum();
    println!("{}", part1);

    let part2: i32 = input.iter().map(|g| g.power()).sum();
    println!("{}", part2);
}
