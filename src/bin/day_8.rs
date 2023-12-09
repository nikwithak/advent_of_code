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
                let new_a = b;
                b = a % b;
                a = new_a;
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

    /// Left for posterity - I misunderstood a part of the problem (or maybe the input was just easier than I expected??)
    /// I thought I had to account for an offset, not realizing that the ghosts reset at each cycle. This solution was never finished,
    /// as I realized that the LCM without accounting for offset was enough.
    fn _traverse_all(&self) -> u64 {
        let start_nodes = self.map.keys().filter(|k| k.ends_with("A"));
        let instruction_length = self.directions.len();
        let cycles =
            dbg!(start_nodes.map(|node| find_cycle_definitions(node, self)).collect::<Vec<_>>());

        // TODO: Bin the cycle defs by what part of the sequence they *start/end* on.
        let mut cycles = cycles.into_iter();
        let first = cycles.next().unwrap();
        let matches = cycles.fold(first, |acc, val| {
            let combined = find_combined_cycles(acc, val);
            combined
        });

        fn calculate_intersection(
            offset: usize,
            left: CycleDefinition,
            right: CycleDefinition,
        ) -> u64 {
            let A = left.first_seen;
            let B = right.first_seen;

            0
        }

        fn find_combined_cycles(
            left: Vec<(u64, CycleDefinition)>,
            right: Vec<(u64, CycleDefinition)>,
        ) -> Vec<(u64, CycleDefinition)> {
            let result = Vec::new();
            // Doing nested loops here, but could optimize by keeping hashmap instead of Vec.
            for (i, left_def) in left {
                for (j, right_def) in &right {
                    if i != *j {
                        // If they don't land on the same beat, ignore.
                        continue;
                    } else {
                    }
                }
            }
            result
        }

        // let result = find_first_intersection(cycles, instruction_length);
        let result = 0;

        unimplemnted!("This solution was never finished");
    }

    /// Doesn't work - inputs / cycles way too large, causes exponential growth. Works only for small inputs.
    fn _traverse_all(&self) -> u64 {
        let mut current = self.map.keys().filter(|k| k.ends_with("A")).collect::<Vec<&String>>();
        let mut distance = 0;
        let mut instructions = self.directions.chars().peekable();

        while !current.iter().all(|cur| cur.ends_with("Z")) {
            if instructions.peek().is_none() {
                instructions = self.directions.chars().peekable();
            }
            let Some(direction) = instructions.next() else { panic!("Shouldn't ever be missing instructions"); };
            let mut next = Vec::new();
            for current in current {
                match direction {
                    'L' => {
                        next.push(&self.map.get(current).unwrap().0);
                    },
                    'R' => {
                        next.push(&self.map.get(current).unwrap().1);
                    },
                    _ => {
                        panic!("invalid direction received")
                    },
                }
            }
            current = next;
            distance += 1;
        }

        distance
    }
}

#[derive(Debug)]
struct CycleDefinition {
    first_seen: u64,
    cycle_length: Option<u64>,
}
/// Runs a ghost through the map, until it finds a cyclical loop. From there, we need to return:
///  * The number of steps until it reaches a looping Z node the loop (offset),
///  * The number of steps that it takes to cycle back to that Z node
///  - multiple options for looping Z nodes
fn find_cycle_definitions(
    start_node: &str,
    map: &SolutionMap,
) -> Vec<(u64, CycleDefinition)> {
    struct BreadcrumbedNode<'a> {
        left: &'a str,
        right: &'a str,
        cycle_locations: HashMap<u64, CycleDefinition>,
    }
    enum VisitStatus {
        NoLoop,
        FirstLoop,
        SecondLoop,
    }

    impl<'a> BreadcrumbedNode<'a> {
        fn new(paths: (&'a str, &'a str)) -> Self {
            BreadcrumbedNode {
                left: paths.0,
                right: paths.1,
                cycle_locations: Default::default(),
            }
        }

        /// Logs the visit, and returns the loop status for this visit
        fn visit(
            &mut self,
            total_distance: usize,
            single_cycle_distance: usize,
        ) -> VisitStatus {
            if let Some(mut node) = self.cycle_locations.get_mut(&(single_cycle_distance as u64)) {
                if node.cycle_length.is_some() {
                    return VisitStatus::SecondLoop;
                }
                node.cycle_length = Some((total_distance - single_cycle_distance) as u64);
                VisitStatus::FirstLoop
            } else {
                self.cycle_locations.insert(
                    single_cycle_distance as u64,
                    CycleDefinition {
                        first_seen: total_distance as u64,
                        cycle_length: None,
                    },
                );
                VisitStatus::NoLoop
            }
        }
    }

    let mut distance = 0;
    let mut instructions = map.directions.chars().enumerate().peekable();
    let nodes: HashMap<_, _> =
    // Store left, right, and the location in the cycle
    // Mutex to hack around the borrow-checker
        map.map.iter().map(|(node, (l, r))| (node.as_str(), Mutex::new(BreadcrumbedNode::new((l, r))))).collect();

    let mut current = nodes.get(start_node).unwrap();

    // while !current.ends_with("Z") {
    loop {
        let mut cur = current.lock().unwrap();
        if instructions.peek().is_none() {
            instructions = map.directions.chars().enumerate().peekable();
        }
        let Some((i, direction)) = instructions.next() else { panic!("Shouldn't ever be missing instructions"); };
        match direction {
            'L' => {
                current = nodes.get(cur.left).unwrap();
            },
            'R' => {
                current = nodes.get(cur.right).unwrap();
            },
            _ => {
                panic!("invalid direction received")
            },
        }
        match cur.visit(distance, i) {
            VisitStatus::NoLoop => (), // Do Nothing
            VisitStatus::FirstLoop => (),
            VisitStatus::SecondLoop => break, // We've cycled a second time - should have found all the cycles by now
        }
        distance += 1;
    }

    let cycle_definitions = nodes
        .into_iter()
        .map(|(_, node)| (node.into_inner().unwrap()))
        .map(|node| {
            node.cycle_locations
                .into_iter()
                // Remove any visits that never looped. Could cause a bug for some datasets, but unlikely to matter.
                .filter(|(_, cycle_def)| cycle_def.cycle_length.is_some())
                .collect::<Vec<_>>()
        })
        .reduce(|mut acc, mut val| {
            acc.append(&mut val);
            acc
        })
        .unwrap();

    cycle_definitions
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

fn main() {}
