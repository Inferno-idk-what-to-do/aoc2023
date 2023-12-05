use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;
use std::string::ParseError;

use aoc_tools;

#[derive(Debug)]
struct Card {
    id: u32,
    winners: HashSet<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect::<Vec<&str>>();
        let id = tokens[1].split(":").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();

        let winners_nums = &tokens[2..];
        let mut idx = 0;

        let mut winners = HashSet::new();
        while winners_nums[idx] != "|" {
            winners.insert(winners_nums[idx].parse::<u32>().unwrap()); 
            idx += 1;
        }
        // skip bar
        idx += 1;

        let mut numbers = Vec::new();
        while idx < winners_nums.len() {
            numbers.push(winners_nums[idx].parse::<u32>().unwrap()); 
            idx += 1;
        }

        return Ok(Card { id, winners, numbers });
    }
}

impl Default for Card {
    fn default() -> Self {
        Card { id: 0, winners: HashSet::new(), numbers: Vec::new() }
    }
}

impl Card {
    fn num_winners(&self) -> u32 {
        self.numbers.iter().filter(|num| self.winners.contains(&num)).map(|re| re.clone()).collect::<Vec<u32>>().len() as u32
    }

    fn score(&self) -> u32 {
        let num_winners = self.num_winners();
        if num_winners == 0 {
            0
        } else {
            2u32.pow(self.num_winners()-1) as u32
        }
    }
}

struct CardStack {
    cards: Vec<Card>,
    copies: Vec<u32>,
}

impl CardStack {
    fn create(&mut self, card_vec: Vec<Card>) {
        self.cards = card_vec;
        self.copies = repeat(1).take(self.cards.len()).collect::<Vec<u32>>();

        // run over each card
        for card_idx in 0..self.cards.len() {
            let num_winners = self.cards[card_idx].num_winners(); 

            // run over each following card for each num winners
            for copies_idx in (card_idx+1)..(card_idx + num_winners as usize + 1usize) {
                // inc num copies by number over copies of current card
                self.copies[copies_idx] += self.copies[card_idx];
            }
        }
    } 

    fn total_cards(&self) -> u32 {
        self.copies.iter().sum()
    }
}

fn main() {
    let data = aoc_tools::read_lines::<Card>("data/day4.txt").unwrap();
    
    let part1 = data.iter().map(|card| card.score()).sum::<u32>();
    println!("{}", part1);

    let mut stack = CardStack { cards: vec![], copies: vec![] };
    stack.create(data);
    let part2 = stack.total_cards();
    println!("{}", part2);
}
