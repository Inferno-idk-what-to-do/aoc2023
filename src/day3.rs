use std::collections::HashSet;

use aoc_tools;

#[derive(Debug, Clone, Copy)]
struct Number {
    x: usize,
    y: usize,
    length: usize,
}

impl Number {
    fn ignore() -> HashSet<char> {
        let mut ignore: HashSet<char> = HashSet::new();
        ignore.insert('0');
        ignore.insert('1');
        ignore.insert('2');
        ignore.insert('3');
        ignore.insert('4');
        ignore.insert('5');
        ignore.insert('6');
        ignore.insert('7');
        ignore.insert('8');
        ignore.insert('9');
        ignore.insert('.');

        return ignore;
    }

    fn verify(&self, schematic: &Vec<Vec<char>>) -> bool {
        let ignore = Number::ignore();

        // can check left side
        if self.y > 0 {

            // left
            if !ignore.contains(&schematic[self.x][self.y-1]) { return true; }

            if self.x > 0 {
                // up left (diagonal)
                if !ignore.contains(&schematic[self.x-1][self.y-1]) { return true; }
            }
            if (self.x+1) < schematic.len() {
                // down left (diagonal)
                if !ignore.contains(&schematic[self.x+1][self.y-1]) { return true; }
            }
        }
        
        // can check right side
        if (self.y+self.length) < schematic[0].len() {
            if !ignore.contains(&schematic[self.x][self.y+self.length]) { return true; }

            if self.x > 0 {
                // up right (diagonal)
                if !ignore.contains(&schematic[self.x-1][self.y+self.length]) { return true; }
            }
            if (self.x+1) < schematic.len() {
                // down right (diagonal)
                if !ignore.contains(&schematic[self.x+1][self.y+self.length]) { return true; }
            }
        }

        // can check above
        if self.x > 0 {
            for idx in self.y..(self.y+self.length) {
                if !ignore.contains(&schematic[self.x-1][idx]) { return true; }
            }
        }

        // can check below
        if (self.x+1) < schematic.len() {
            for idx in self.y..(self.y+self.length) {
                if !ignore.contains(&schematic[self.x+1][idx]) { return true; }
            }
        } 

        return false;
    }

    fn value(&self, schematic: &Vec<Vec<char>>) -> i32 {
        let mut num_str = String::new();

        for i in self.y..(self.y+self.length) {
            num_str.push(schematic[self.x][i]);
        }

        return num_str.parse::<i32>().unwrap();
    }

    fn contains(&self, row: i32, col: i32) -> bool {
        if self.x as i32 == row {
            return self.y as i32 <= col && ((self.y+self.length) as i32) > col; 
        }

        return false;
    }
}

fn find_numbers(schematic: &Vec<String>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for row in 0..schematic.len() {
        let chars = schematic[row].chars().collect::<Vec<char>>();
        let mut col = 0;

        while col < schematic[row].len() {
            if let Some(_) = chars[col].to_digit(10) {
                let mut size = 1;
                let starting_col = col;

                col += 1;
                while let Some(_) = chars.get(col).unwrap_or(&'a').to_digit(10) {
                    col += 1;
                    size += 1;

                    if col > schematic[row].len() {
                        break;
                    }
                }

                numbers.push(Number { x: row, y: starting_col, length: size });
            } else {
                col += 1;
            }
        }
    }
    
    numbers
}

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
}

impl Gear {
    fn ratio(&self, nums: &Vec<Number>, schematic: &Vec<Vec<char>>) -> Option<i32> {
        let mut near: Vec<Number> = Vec::new();

        for num in nums {
            if num.contains(self.x as i32 -1, self.y as i32 -1) { near.push(*num); }
            else if num.contains(self.x as i32 -1, self.y as i32) { near.push(*num); }
            else if num.contains(self.x as i32 -1, self.y as i32 +1) { near.push(*num); }
            else if num.contains(self.x as i32 , self.y as i32 -1) { near.push(*num); }
            else if num.contains(self.x as i32 , self.y as i32 +1) { near.push(*num); }
            else if num.contains(self.x as i32 +1, self.y as i32 -1) { near.push(*num); }
            else if num.contains(self.x as i32 +1, self.y as i32) { near.push(*num); }
            else if num.contains(self.x as i32 +1, self.y as i32 +1) { near.push(*num); }
        }

        if near.len() == 2 {
            return Some(near[0].value(schematic) * near[1].value(schematic));
        } else {
            return None; 
        }
    }
}

fn gears(schematic: &Vec<Vec<char>>) -> Vec<Gear> {
    let mut gears = Vec::new();

    for row in 0..schematic.len() {
        for col in 0..schematic[0].len() {
            let c = schematic[row][col];

            if c == '*' { gears.push(Gear { x: row, y: col}); }
        }
    }

    return gears;
}

fn main() {
    let data: Vec<String> = aoc_tools::read_lines("data/day3.txt").unwrap();
    let data_chars = data.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let part1 = find_numbers(&data).iter()
                                  .filter(|number| number.verify(&data_chars))
                                  .map(|num_filtered| num_filtered.value(&data_chars))
                                  .collect::<Vec<i32>>()
                                  .iter()
                                  .sum::<i32>();
    println!("{}", part1);

    let numbers = find_numbers(&data).iter()
                                    .filter(|number| number.verify(&data_chars))
                                    .map(|num_borrowed| num_borrowed.clone())
                                    .collect::<Vec<Number>>();

    let gears = gears(&data_chars);

    let part2 = gears.iter().map(|gear| gear.ratio(&numbers, &data_chars).unwrap_or(0)).sum::<i32>();
    println!("{}", part2);
}
