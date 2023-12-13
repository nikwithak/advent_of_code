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
        assert_eq!(result, 6871);
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
    children: Vec<MemoKey>,
    parents: HashSet<MemoKey>,
    times_visited: usize,
}

#[derive(Hash, PartialEq, Debug, Eq, Clone)]
struct MemoKey {
    current: Option<usize>,
    groups: Vec<usize>,
    i: usize,
}

struct StateCache {
    states: HashMap<(usize, usize), bool>,
}

struct StepResult<'a> {
    next_states: Vec<StateMachine<'a>>,
    completed_state: Option<(usize, usize, bool)>,
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

    fn step(&mut self) -> Vec<StateMachine<'a>> {
        if self.i >= self.str.len() {
            // println!("END OF THE LINE: {:?}, {:?}", &self, &self.get_status());
            return vec![self.clone()];
        }
        let next_char = &(self.str.as_bytes()[self.i] as char);
        let can_place = self.can_place_tile(next_char);
        let must_place = next_char.eq(&'#') || self.current.filter(|c| *c > 0).is_some();

        // TODO: Fn this out
        let mut next_state = self.clone();
        next_state.children = Vec::new(); // Might not be needed, since we haven't pushed to self.children yet
        next_state.parents = HashSet::new();
        next_state.parents.insert((&*self).into());
        next_state.times_visited = 1;
        next_state.i += 1;

        if next_state.current.eq(&Some(0)) {
            next_state.current.take();
        }

        if must_place && can_place {
            next_state.place_tile();
            self.children.push((&next_state).into());
            vec![next_state]
        } else if must_place && !can_place {
            // Failed - no new branches
            Vec::new()
        } else if can_place {
            let mut branch = next_state.clone();
            branch.skip_tile();
            next_state.place_tile();
            self.children.push((&next_state).into());
            self.children.push((&branch).into());
            vec![next_state, branch]
        } else {
            next_state.skip_tile();
            self.children.push((&next_state).into());
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
            my_guess: "".into(),
            i: 0,
            children: Default::default(),
            parents: Default::default(),
            times_visited: 1,
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

    let mut row: String = orig_row.to_string();
    row.push('?');
    row = row.repeat(fold - 1);
    row.push_str(orig_row);
    currently_tracking.push(StateMachine::new(group_nums.clone(), &row));

    impl From<&StateMachine<'_>> for MemoKey {
        fn from(value: &StateMachine) -> Self {
            Self {
                current: value.current.clone(),
                groups: value.groups.clone(),
                i: value.i,
            }
        }
    }
    let mut memo: HashMap<MemoKey, StateMachine> = HashMap::new();
    while let Some(mut track) = currently_tracking.pop() {
        if let Some(track_cached) = memo.get_mut(&(&track).into()) {
            // We've already calculated (or started calculating) the chain from this state, so just mark it
            // with an additional visit. We will later multiply the downstream results of this chain by the number
            // of visits
            track_cached.times_visited += 1;
            track_cached.parents.extend(track.parents.into_iter());
            // if let Some(parent) = track.parent {
            //     if let Some(parent_count) = track_cached.parents.get_mut(&parent) {
            //         // assert!(false);
            //         *parent_count += 1;
            //     } else {
            //         track_cached.parents.insert(parent.clone(), 1);
            //     }
            // }
            // println!("{}: {} (CACHED)", &track.i, &track.my_guess);
        } else {
            let memo_key = (&track).into();
            currently_tracking.append(&mut track.step());
            // println!("{}: {} (FIRSTHIT)", &track.i, &track.my_guess);
            memo.insert(memo_key, track);
        }
    }

    let successes = memo
        .iter()
        .filter(|(_, val)| (*val).get_status().eq(&Status::Success))
        .map(|(a, b)| {
            // println!("KEY: {:?}", &a);
            // println!("Val: {:?}", &b);
            (a, b)
        })
        // .map(|(_, val)| val);
        // .map(|(key, _)| key);
        ;
    for s in successes.clone() {
        // println!("SUCCESS: {}", &s.1.my_guess);
    }
    let successes = successes.map(|(key, _)| key);

    // Calculate our results!
    let mut result = 0;
    {
        let mut stack = successes.map(|state| (state.clone(), 1)).collect::<Vec<_>>();
        // let mut stack: Vec<(StateMachine, usize)> = vec![(StateMachine::new(group_nums.clone(), line), 1)]; // Start with intiial StateMachine state

        while let Some((node_key, count)) = stack.pop() {
            let node = memo.get_mut(&node_key).unwrap();
            // println!("node (stack size: {}): {}", &stack.len(), &node.my_guess);
            if node.parents.len() == 0 {
                result += count;
            } else {
                for parent in &node.parents {
                    stack.push((parent.clone(), count));
                }
            }
        }
    }
    result
}

// One -> Split -> split ->
// (Completed nodes - ONLY AT END (i = len()))
// Complete

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
        println!("{}: {} - took {} ms", line, sum, (end_time.duration_since(time).unwrap().as_millis()));
    }
    sum
}

fn main() {}
