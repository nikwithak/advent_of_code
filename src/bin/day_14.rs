use std::{collections::HashMap, fmt::Display};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pt1_sample() {
        let input = parse_input("inputs/day_14_sample.txt");
        let part_1 = part_1(input);
        assert_eq!(part_1, 136);
    }
    #[test]
    fn pt1_final() {
        let input = parse_input("inputs/day_14.txt");
        let part_1 = part_1(input);
        assert_eq!(part_1, 106378);
    }
    #[test]
    fn pt2_sample() {
        let input = parse_input("inputs/day_14_sample.txt");
        let part_1 = part_2(input);
        assert_eq!(part_1, 64);
    }
    #[test]
    fn pt2_final() {
        let input = parse_input("inputs/day_14.txt");
        let part_1 = part_2(input);
        assert_eq!(part_1, 90795);
    }
}
struct Solution {
    chars: Vec<Vec<char>>,
}

impl Display for Solution {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let str = self
            .chars
            .iter()
            .map(|row| row.iter().cloned().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        f.write_str(&str)
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn shift_col(
    arr: &mut Vec<Vec<char>>,
    col_idx: usize,
    dir: &Direction,
) {
    let (mut next_space, space_offset, iter) = match dir {
        Direction::East => panic!("can't shift row north"),
        Direction::West => panic!("Can't shift row south"),
        Direction::South => {
            ((arr.len() - 1), -1 as isize, (0..arr.len()).rev().collect::<Vec<_>>().into_iter())
        },
        Direction::North => (0, 1, (0..arr.len()).collect::<Vec<_>>().into_iter()),
    };

    for i in iter {
        match arr[i][col_idx] {
            'O' => {
                arr[i][col_idx] = '.';
                arr[next_space as usize][col_idx] = 'O';
                next_space = next_space.saturating_add_signed(space_offset);
            },
            '.' => {}, // Do nothing,
            '#' => {
                // Wall / doesn't move - blocking point!
                next_space = i.saturating_add_signed(space_offset);
            },
            _ => panic!("Shouldn't get here"),
        }
    }
}
// fn shift_col(
//     arr: &mut Vec<Vec<char>>,
//     col_idx: usize,
//     dir: Direction,
// ) {
//     let (next_space, space_offset, iter) = match dir {
//         Direction::East => panic!("can't shift row north"),
//         Direction::West => panic!("Can't shift row south"),
//         Direction::North => (0 as usize, 1, 0..arr.len()),
//         Direction::South => (arr.len() - 1, -1, (0..arr.len()).rev()),
//     };

//     for i in iter {
//         println!("({}, {}): {} -> next open space: {}", x, y, &self.chars[y][x], next_space);
//         match self.chars[y][x] {
//             'O' => {
//                 count_o += 1;
//                 row[i] = '.';
//                 row[next_space] = 'O';
//                 next_space += space_offset;
//             },
//             '.' => {}, // Do nothing,
//             '#' => {
//                 // Wall / doesn't move - blcoking point!
//                 count_hash += 1;
//                 next_space = i + space_offset;
//             },
//             _ => panic!("Shouldn't get here"),
//         }
//     }
// }

fn shift_row(
    row: &mut Vec<char>,
    dir: &Direction,
) {
    let (mut next_space, space_offset, iter) = match dir {
        Direction::North => panic!("can't shift row north"),
        Direction::South => panic!("Can't shift row south"),
        Direction::East => {
            ((row.len() - 1), -1 as isize, (0..row.len()).rev().collect::<Vec<_>>().into_iter())
        },
        Direction::West => (0, 1, (0..row.len()).collect::<Vec<_>>().into_iter()),
    };

    for i in iter {
        match row[i] {
            'O' => {
                row[i] = '.';
                row[next_space as usize] = 'O';
                next_space = next_space.saturating_add_signed(space_offset);
            },
            '.' => {}, // Do nothing,
            '#' => {
                // Wall / doesn't move - blocking point!
                next_space = i.saturating_add_signed(space_offset);
            },
            _ => panic!("Shouldn't get here"),
        }
    }
}

impl Solution {
    fn tilt(
        &mut self,
        dir: Direction,
    ) {
        match dir {
            Direction::North | Direction::South => {
                for i in 0..self.chars[0].len() {
                    shift_col(&mut self.chars, i, &dir);
                }
            },
            Direction::West | Direction::East => {
                for row in &mut self.chars {
                    shift_row(row, &dir);
                }
            },
        }
    }
    fn tilt_north(&mut self) {
        self.tilt(Direction::North)
        // // Assume all rows are of equal length, and we have at least one row in every input.
        // for x in 0..self.chars[0].len() {
        //     let mut next_space = 0;
        //     for y in 0..self.chars.len() {
        //         println!(
        //             "({}, {}): {} -> next open space: {}",
        //             x, y, &self.chars[y][x], next_space
        //         );
        //         match self.chars[y][x] {
        //             'O' => {
        //                 self.chars[y][x] = '.';
        //                 self.chars[next_space][x] = 'O';
        //                 next_space += 1;
        //             },
        //             '.' => {}, // Do nothing,
        //             '#' => {
        //                 // Wall / doesn't move - blcoking point!
        //                 next_space = y + 1;
        //             },
        //             _ => panic!("Shouldn't get here"),
        //         }
        //     }
        // }
    }

    fn calc_total_load(&self) -> usize {
        let mut result = 0;
        for (y, row) in self.chars.iter().enumerate() {
            result += row.iter().filter(|c| **c == 'O').count() * (self.chars.len() - y);
        }
        result
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }
}

fn part_1(mut input: Solution) -> usize {
    println!("Before: {}", &input);
    input.tilt_north();
    println!("After: {}", &input);
    input.calc_total_load()
}

fn part_2(mut input: Solution) -> usize {
    let num_cycles = 1000000000;
    println!("Before: {}", &input);
    let mut memo = HashMap::new();

    let mut i = num_cycles;
    let mut loop_found = false;
    while i > 0 {
        input.cycle();
        println!();
        println!("{} cycle:", num_cycles - i);
        println!("{}", &input);
        println!("Load Value: {}", &input.calc_total_load());
        println!();
        if !loop_found && memo.contains_key(&input.chars) {
            if let Some(prev) = memo.get(&input.chars) {
                let loop_iter = prev - i;
                println!("First hit: {}", prev);
                println!("Loop found at i: {}", i);
                loop_found = true;
                i = i % loop_iter;
            }
        } else {
            memo.insert(input.chars.clone(), i);
        }
        i -= 1;
    }
    input.calc_total_load()
}

fn main() {
    let mut input = parse_input("inputs/day_14.txt");
    let part_1 = part_1(input);
    println!("Part 1: {}", part_1);

    let mut input = parse_input("inputs/day_14.txt");
    let part_2 = part_2(input);
    println!("Part 2: {}", part_2);
}

fn parse_input(filename: &str) -> Solution {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|row| row.as_bytes().iter().map(|b| *b as char).collect())
        .collect();
    Solution {
        chars: input,
    }
}
