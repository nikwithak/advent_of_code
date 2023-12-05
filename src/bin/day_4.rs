use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_input() {
        let input = parse_input("inputs/day_4_sample.txt");
        let result = count_points(input);
        assert_eq!(13, result);
    }
}

fn count_points(cards: Vec<Card>) -> u64 {
    cards
        .iter()
        .map(|card| {
            println!("Card {} has {} points", &card.id, &card.count_points());
            card
        })
        .fold(0, |acc, card| acc + card.count_points())
}

fn main() {
    let input = parse_input("inputs/day_4.txt");
    let result = count_points(input);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct Card {
    id: u64,
    winning_numbers: Vec<u64>,
    card_numbers: Vec<u64>,
}

impl Card {
    fn count_points(&self) -> u64 {
        let matches = self.count_matches();
        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }
    fn count_matches(&self) -> u64 {
        let mut matches = 0;
        for number in &self.card_numbers {
            if self.winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        matches
    }
}

fn parse_card(line: &str) -> Card {
    let regex =
        Regex::new(r"Card *(?<id>\d+): (?<winning_numbers>( *\d+)+)\ \| (?<card_numbers>( *\d+)+)")
            .unwrap();
    let captures = regex.captures(line).unwrap();
    Card {
        id: captures["id"].parse().unwrap(),
        winning_numbers: captures["winning_numbers"]
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect(),
        card_numbers: captures["card_numbers"]
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect(),
    }
}

fn parse_input(filename: &str) -> Vec<Card> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let cards: Vec<Card> =
        reader.lines().map(|line| line.unwrap()).map(|line| parse_card(&line)).collect();

    cards
}
