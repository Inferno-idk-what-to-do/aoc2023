use aoc_tools;

fn first_last_digit(str: &str) -> i32 {
    let nums = str.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
    format!("{}{}", nums[0], nums[nums.len()-1]).parse::<i32>().unwrap()
}

fn first_num(str: &str) -> i32 {
    let vector = str.chars().collect::<Vec<char>>();

    for i in 0..str.len() {
        if vector.get(i).unwrap().is_numeric() {
            return vector.get(i).unwrap().to_digit(10).unwrap() as i32;
        } else {
            if (i+4) < str.len() {
                match &String::from_iter(&vector[i..i+5])[..] {
                    "three" => return 3,
                    "seven" => return 7,
                    "eight" => return 8,
                    _ => (),
                };
            }
            if (i+3) < str.len() {
                match &String::from_iter(&vector[i..i+4])[..] {
                    "four" => return 4,
                    "five" => return 5,
                    "nine" => return 9,
                    _ => (),
                };
            }
            if (i+2) < str.len() {
                match &String::from_iter(&vector[i..i+3])[..] {
                    "one" => return 1,
                    "two" => return 2,
                    "six" => return 6,
                    _ => (),
                };
            }
        }
    }

    return 0;
}

fn last_num(str: &str) -> i32 {
    let vector = str.chars().collect::<Vec<char>>();

    for i in (0..str.len()).rev() {
        if vector.get(i).unwrap().is_numeric() {
            return vector.get(i).unwrap().to_digit(10).unwrap() as i32;
        } else {
            if (i+4) < str.len() {
                match &String::from_iter(&vector[i..i+5])[..] {
                    "three" => return 3,
                    "seven" => return 7,
                    "eight" => return 8,
                    _ => (),
                };
            }
            if (i+3) < str.len() {
                match &String::from_iter(&vector[i..i+4])[..] {
                    "four" => return 4,
                    "five" => return 5,
                    "nine" => return 9,
                    _ => (),
                };
            }
            if (i+2) < str.len() {
                match &String::from_iter(&vector[i..i+3])[..] {
                    "one" => return 1,
                    "two" => return 2,
                    "six" => return 6,
                    _ => (),
                };
            }
        }
    }

    return 0;
}

fn first_last_digit_with_spellings(str: &str) -> i32 {
    format!("{}{}", first_num(str), last_num(str)).parse::<i32>().unwrap()
}

fn main() {
    let input: Vec<String> = aoc_tools::read_lines::<String>("data/day1.txt").unwrap();    

    let part1 = input.iter().map(|line| first_last_digit(line)).sum::<i32>();
    println!("{}", part1);

    let part2 = input.iter().map(|line| first_last_digit_with_spellings(line)).sum::<i32>();
    println!("{}", part2);
}
