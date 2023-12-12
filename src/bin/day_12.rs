use std::{collections::HashMap, env::current_dir, iter::Enumerate, str::Chars};

use reqwest::StatusCode;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample() {
        let result = sum_possible_arrangements("inputs/day_12_sample.txt");
        assert_eq!(result, 21);
    }

    #[test]
    fn part_1_final() {
        let result = sum_possible_arrangements("inputs/day_12.txt");
        assert_eq!(result, 6871);
    }
    #[test]
    fn part_2_sample() {
        let result = sum_possible_arrangements_with_fold("inputs/day_12_sample.txt", 5);
        assert_eq!(result, 525152);
    }

    #[test]
    fn part_2_final() {
        let result = sum_possible_arrangements_with_fold("inputs/day_12.txt", 5);
        assert_eq!(result, 6871);
    }
}

// For each segment, find: {
// 1) If it is entierly ?s, then we can fit a maximum of:
//    - ???? 2 1 = ##

#[derive(Clone)]
struct StateMachine<'a> {
    current: Option<usize>,
    groups: Vec<usize>,
    str: &'a str,
    i: usize,
}

struct StateCache {
    states: HashMap<(usize, usize), bool>,
}

struct StepResult<'a> {
    next_states: Vec<StateMachine<'a>>,
    completed_state: Option<(usize, usize, bool)>,
}

enum Status {
    Running,
    Success,
    Failed,
}

impl<'a> StateMachine<'a> {
    fn is_complete(&self) -> bool {
        self.groups.len() == 0 && self.current.filter(|cur| *cur > 0).is_none()
    }

    fn get_status(&self) -> Status {
        if self.is_complete() {
            Status::Success
        } else if self.remaining_count() > self.remaining_string_len() {
            Status::Failed
        } else {
            Status::Running
        }
    }

    fn remaining_count(&self) -> usize {
        self.current.unwrap_or(0) + self.groups.iter().sum::<usize>()
    }
    fn remaining_string_len(&self) -> usize {
        self.str.len() - self.i - 1
    }

    fn can_place_tile(
        &self,
        next_char: &char,
    ) -> bool {
        if next_char.eq(&'.')
            || Some(0) == self.current
            || (self.current.is_none() && self.groups.is_empty())
        {
            false
        } else {
            true
        }
    }

    fn place_tile(&mut self) {
        if let Some(mut current) = self.current.as_mut() {
            *current -= 1;
        } else {
            self.current = self.groups.pop().map(|c| c - 1);
        }
    }

    fn step(mut self) -> Vec<StateMachine<'a>> {
        let next_char = &(self.str.as_bytes()[self.i] as char);
        let can_place = self.can_place_tile(next_char);
        let must_place = next_char.eq(&'#') || self.current.filter(|c| *c > 0).is_some();

        self.i += 1;

        if self.current.eq(&Some(0)) {
            self.current.take();
        }
        if must_place && can_place {
            self.place_tile();
            vec![self]
        } else if must_place && !can_place {
            // Failed - no new branches
            Vec::new()
        } else if can_place {
            let branch = self.clone();
            self.place_tile();
            vec![self, branch]
        } else {
            vec![self]
        }
    }

    fn new(
        groups: Vec<usize>,
        string: &'a str,
    ) -> Self {
        // Reverse it so that pop() (easily) goes front->back
        let groups: Vec<_> = groups.into_iter().rev().collect();
        Self {
            current: None,
            groups,
            str: string,
            i: 0,
        }
    }
}
// For each group: math the possibilites for each state machine
// eg. for inputs (1,2,3):
// - For each group, calculate how many ways you can fit () [empty set], (1), (1, 2), (1, 2, 3). fe

fn count_possibilities(line: &str) -> usize {
    count_possibilities_with_fold(line, 1)
}
fn count_possibilities_with_fold(
    line: &str,
    fold: usize,
) -> usize {
    let mut problem_parts = line.split_whitespace();
    let (orig_row, groups) = (problem_parts.next().unwrap(), problem_parts.next().unwrap());
    assert!(problem_parts.next().is_none());

    let group_nums = groups
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .repeat(fold);

    let mut currently_tracking: Vec<StateMachine> = Vec::new();
    currently_tracking.push(StateMachine::new(group_nums.clone(), line));

    let mut row: String = orig_row.to_string();
    row.push('?');
    row = row.repeat(fold - 1);
    row.push_str(orig_row);

    for c in row.chars() {
        let mut next_states = Vec::<StateMachine>::new();
        while let Some(track) = currently_tracking.pop() {
            next_states.append(&mut track.step());
        }
        currently_tracking = next_states;
        println!("Queue size: {}", currently_tracking.len());
    }
    // // let mut states: Vec<StateMachine> = Vec::new();
    // // // let segments = row.split('.').filter(|s| !s.is_empty());
    // // for s in segments {
    // //     // Step 1: Count all possibilities for *groupings* (do the numbers add up?)

    // //     // Step 2: Count possible mappings of each grouping using math
    // //     // Step 3: ????
    // //     for c in s.chars() {}
    // // }

    currently_tracking.iter().filter(|c| c.is_complete()).count()
}

fn sum_possible_arrangements(filename: &str) -> usize {
    sum_possible_arrangements_with_fold(filename, 1)
}
fn sum_possible_arrangements_with_fold(
    filename: &str,
    fold: usize,
) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        sum += dbg!(count_possibilities_with_fold(dbg!(line), fold));
    }
    sum
}

fn main() {}
