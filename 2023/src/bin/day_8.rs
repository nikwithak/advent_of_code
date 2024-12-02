use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    str::Chars,
    sync::Mutex,
};

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample_1() {
        let parsed = parse_input("inputs/day_8_sample_1.txt");
        let result = parsed.traverse("AAA");
        assert_eq!(result, 2);
    }
    #[test]
    fn part_1_sample_2() {
        let parsed = parse_input("inputs/day_8_sample_2.txt");
        let result = parsed.traverse("AAA");
        assert_eq!(result, 6);
    }
    #[test]
    fn part_1_final() {
        let parsed = parse_input("inputs/day_8.txt");
        let result = parsed.traverse("AAA");
        assert_eq!(result, 19667);
    }
    #[test]
    fn part_2_sample() {
        let parsed = parse_input("inputs/day_8_part_2_sample.txt");
        let result = parsed.lcm_approach();
        assert_eq!(result, 6);
    }
    #[test]
    fn part_2_final() {
        let parsed = parse_input("inputs/day_8.txt");
        let result = parsed.lcm_approach();
        assert_eq!(result, 19185263738117);
    }
}

struct SolutionMap {
    map: HashMap<String, (String, String)>,
    directions: String,
}

impl SolutionMap {
    /// Traverses the map with the instructions, and returns the length of the path taken.
    fn traverse(
        &self,
        start: &str,
    ) -> u64 {
        let mut current = start;
        let mut distance = 0;
        let mut instructions = self.directions.chars().peekable();

        while !current.ends_with("Z") {
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

    fn lcm_approach(&self) -> u64 {
        fn gcd(
            mut a: u64,
            mut b: u64,
        ) -> u64 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }
        fn lcm(
            a: u64,
            b: u64,
        ) -> u64 {
            (a / gcd(a, b)) * b
        }

        self.map
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|t| self.traverse(t))
            .map(|t| dbg!(t))
            .reduce(|a, b| dbg!(lcm(a, b)))
            .unwrap()
    }
}

fn parse_input(filename: &str) -> SolutionMap {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut lines = reader.lines();
    let Some(Ok(directions)) = lines.next() else { panic!("Unexpected end of input"); };
    let _ = lines.next();

    let mut map = HashMap::new();
    let regex = Regex::new(r"(?<node>.{3}) = \((?<left>.{3}), (?<right>.{3})\)").unwrap();
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

fn main() {
    let parsed = parse_input("inputs/day_8.txt");
    let part_1_result = parsed.traverse("AAA");
    println!("Part 1: {}", &part_1_result);
    let part_2_result = parsed.lcm_approach();
    println!("Part 2: {}", &part_2_result);
}
