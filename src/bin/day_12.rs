use std::{
    collections::{HashMap, HashSet},
    env::current_dir,
    iter::Enumerate,
    str::Chars,
    time,
};

use reqwest::StatusCode;

// This one got particularly convoluted as I tried multiple different approaches.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample() {
        let lines = vec![
            // ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];

        for line in lines {
            assert_eq!(count_possibilities(&line.0), line.1);
            // println!("PASSED: {} = {}", &line.0, &line.1);
        }
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
        assert_eq!(result, 2043098029844);
    }
}

// For each segment, find: {
// 1) If it is entierly ?s, then we can fit a maximum of:
//    - ???? 2 1 = ##

#[derive(Clone, Debug)]
struct StateMachine<'a> {
    current: Option<usize>,
    groups: Vec<usize>,
    my_guess: String,
    str: &'a str,
    i: usize,
}

#[derive(Hash, PartialEq, Debug, Eq, Clone)]
struct MemoKey {
    current: Option<usize>,
    groups: Vec<usize>,
    i: usize,
}

#[derive(PartialEq, Debug, Eq)]
enum Status {
    Running,
    Success,
    Failed,
}

impl<'a> StateMachine<'a> {
    fn is_complete(&self) -> bool {
        self.groups.len() == 0
            && self.current.filter(|cur| *cur > 0).is_none()
            && self.remaining_string_len() == 0
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
        self.str.len() - self.i
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

    fn skip_tile(&mut self) {
        self.my_guess.push('.');
    }

    fn place_tile(&mut self) {
        self.my_guess.push('#');
        if let Some(current) = self.current.as_mut() {
            *current -= 1;
        } else {
            self.current = self.groups.pop().map(|c| c - 1);
        }
    }

    fn step(&self) -> Vec<StateMachine<'a>> {
        if self.i >= self.str.len() {
            // Already at end - no next steps!
            return vec![];
        }
        if self.remaining_count() > self.remaining_string_len() {
            // Impossible to finish - no next steps!
            return Vec::new();
        }
        let next_char = &(self.str.as_bytes()[self.i] as char);
        let can_place = self.can_place_tile(next_char);
        let must_place = next_char.eq(&'#') || self.current.filter(|c| *c > 0).is_some();

        let mut next_state = self.clone();
        next_state.i += 1;

        if next_state.current.eq(&Some(0)) {
            next_state.current.take();
        }

        if must_place && can_place {
            // Only one option
            next_state.place_tile();
            vec![next_state]
        } else if must_place && !can_place {
            // Failed - no new branches
            Vec::new()
        } else if can_place {
            // Split into two states
            let mut branch = next_state.clone();
            branch.skip_tile();
            next_state.place_tile();
            vec![next_state, branch]
        } else {
            // Can't place - only one path forward
            next_state.skip_tile();
            vec![next_state]
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
            my_guess: "".into(), // For debugging
            i: 0,
        }
    }
}

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

    let mut row: String = orig_row.to_string();
    row.push('?');
    row = row.repeat(fold - 1);
    row.push_str(orig_row);
    row = row.split(".").filter(|s| !s.is_empty()).collect::<Vec<_>>().join(".");

    let mut memo: HashMap<MemoKey, usize> = HashMap::new();

    // Start stack with basic state
    let start = StateMachine::new(group_nums.clone(), &row);

    // let mut stack: Vec<StateMachine> = vec![start.clone()];

    // while !stack.is_empty() {
    //     let mut state = stack.pop().unwrap();
    //     // println!("CHECKING STATE: {}", &state.my_guess);

    //     if let Some(value) = memo.get(&(&state).into()) {
    //         // println!("CACHED: {} ({})", &state.my_guess, value);
    //         // If it's already calculated, we can skip
    //     } else if state.get_status() == Status::Success {
    //         println!("{} ({})", &state.my_guess, 1);
    //         // If it's done, then we stop!
    //         memo.insert((&state).into(), 1);
    //     } else {
    //         let next_states = state.step();
    //         if next_states.iter().filter(|s| !memo.contains_key(&(*s).into())).count() == 0 {
    //             // We have enough to calculate!
    //             let value = next_states
    //                 .iter()
    //                 .map(|s| *memo.get(&s.into()).unwrap())
    //                 .reduce(|acc, count| acc + count)
    //                 .unwrap_or(0);
    //             memo.insert((&state).into(), value);
    //             println!("{} ({})", &state.my_guess, value);
    //         } else {
    //             stack.push(state);
    //             next_states
    //                 .into_iter()
    //                 .filter(|s| !memo.contains_key(&s.into()))
    //                 .map(|s| {
    //                     // println!("NEW STATE: {}", &s.my_guess);
    //                     s
    //                 })
    //                 .for_each(|s| stack.push(s));
    //         }
    //     };
    // }

    // *memo.get(&(&start).into()).unwrap()

    fn recurse(
        memo: &mut HashMap<MemoKey, usize>,
        state: &mut StateMachine,
    ) -> usize {
        let value = if let Some(value) = memo.get(&(&*state).into()) {
            *value
        } else if state.get_status() == Status::Success {
            // println!("{} ({})", &state.my_guess, 1);
            1
        } else {
            let mut calced_value = 0;
            for mut new_state in state.step() {
                calced_value += recurse(memo, &mut new_state);
            }
            // println!("{} ({})", &state.my_guess, calced_value);
            calced_value
        };
        memo.insert((&*state).into(), value);
        value
    }
    return recurse(&mut memo, &mut StateMachine::new(group_nums.clone(), &row));
}

impl From<&StateMachine<'_>> for MemoKey {
    fn from(value: &StateMachine) -> Self {
        Self {
            current: value.current.clone(),
            groups: value.groups.clone(),
            i: value.i,
        }
    }
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
        let time = time::SystemTime::now();
        sum += count_possibilities_with_fold(line, fold);
        let end_time = time::SystemTime::now();
        println!(
            "{}: {} - took {} ms",
            line,
            sum,
            (end_time.duration_since(time).unwrap().as_millis())
        );
    }
    sum
}

fn main() {}
