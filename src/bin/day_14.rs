use std::fmt::Display;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pt1_sample() {
        let mut input = parse_input("inputs/day_14_sample.txt");
        let part_1 = part_1(input);
        assert_eq!(part_1, 136);
    }
    #[test]
    fn pt1_final() {
        let mut input = parse_input("inputs/day_14.txt");
        let part_1 = part_1(input);
        assert_eq!(part_1, 136);
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

impl Solution {
    fn tilt_north(&mut self) {
        let (mut x, mut y) = (0, self.chars.len() - 1);
        // Assume all rows are of equal length, and we have at least one row in every input.
        for x in 0..self.chars[0].len() {
            let mut next_space = 0;
            for y in 0..self.chars.len() {
                println!(
                    "({}, {}): {} -> next open space: {}",
                    x, y, &self.chars[y][x], next_space
                );
                match self.chars[y][x] {
                    'O' => {
                        self.chars[y][x] = '.';
                        self.chars[next_space][x] = 'O';
                        next_space += 1;
                    },
                    '.' => {}, // Do nothing,
                    '#' => {
                        // Wall / doesn't move - blcoking point!
                        next_space = y + 1;
                    },
                    _ => panic!("Shouldn't get here"),
                }
            }
        }
    }

    fn calc_total_load(&self) -> usize {
        let mut result = 0;
        for (y, row) in self.chars.iter().enumerate() {
            for char in row {
                if *char == 'O' {
                    result += self.chars.len() - y;
                }
            }
        }
        result
    }
}

fn part_1(mut input: Solution) -> usize {
    println!("Before: {}", &input);
    input.tilt_north();
    println!("After: {}", &input);
    input.calc_total_load()
}

fn main() {
    let mut input = parse_input("inputs/day_14.txt");
    let part_1 = part_1(input);
    println!("Part 1: {}", part_1);

    let mut input = parse_input("inputs/day_14.txt");
    let part_2 = part_2(&input);
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

fn part_2(input: &Solution) -> usize {
    todo!()
}
