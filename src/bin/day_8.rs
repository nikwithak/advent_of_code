use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::Chars,
};

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample_1() {
        let parsed = parse_input("inputs/day_8_sample_1.txt");
        let result = parsed.traverse();
        assert_eq!(result, 2);
    }
    #[test]
    fn part_1_sample_2() {
        let parsed = parse_input("inputs/day_8_sample_2.txt");
        let result = parsed.traverse();
        assert_eq!(result, 6);
    }
    #[test]
    fn part_1_final() {
        let parsed = parse_input("inputs/day_8.txt");
        let result = parsed.traverse();
        assert_eq!(result, 19667);
    }
}

struct SolutionMap {
    map: HashMap<String, (String, String)>,
    directions: String,
}

impl SolutionMap {
    /// Traverses the map with the instructions, and returns the length of the path taken.
    fn traverse(&self) -> u64 {
        let mut current = "AAA";
        let target = "ZZZ";
        let mut distance = 0;
        let mut instructions = self.directions.chars().peekable();

        while !current.eq(target) {
            if instructions.peek().is_none() {
                instructions = self.directions.chars().peekable();
            }
            let Some(direction) = instructions.next() else { panic!("Shouldn't ever be missing instructions"); };
            match direction {
                'L' => {
                    current = &self.map.get(current).unwrap().0;
                },
                'R' => {
                    current = &self.map.get(current).unwrap().1;
                },
                _ => {
                    panic!("invalid direction received")
                },
            }
            distance += 1;
        }

        distance
    }
}

fn parse_input(filename: &str) -> SolutionMap {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut lines = reader.lines();
    let Some(Ok(directions)) = lines.next() else { panic!("Unexpected end of input"); };
    let _ = lines.next();

    let mut map = HashMap::new();
    let regex =
        Regex::new(r"(?<node>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();
    while let Some(Ok(line)) = lines.next() {
        let mut captures = regex.captures(&line).unwrap();
        map.insert(
            captures["node"].to_string(),
            (captures["left"].to_string(), captures["right"].to_string()),
        );
    }

    SolutionMap {
        map,
        directions,
    }
}

fn main() {}
