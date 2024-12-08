pub fn run_day_07() {
    let input = std::fs::read_to_string("inputs/07.txt").unwrap();
    println!("Day 7 Part 1: {}", part_1(&input));
    println!("Day 7 Part 2: {}", part_2(&input));
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(":").map(str::trim);
    let test_value: u64 = parts.next().map(str::parse).and_then(Result::ok).unwrap();
    let operands: Vec<u64> = parts
        .next()
        .map(str::split_whitespace)
        .unwrap()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    (test_value as u64, operands)
}
fn part_1(input: &str) -> u64 {
    fn is_match(
        target: u64,
        cur_val: u64,
        operands: &[u64],
    ) -> bool {
        if operands.len() == 0 {
            return cur_val == target;
        }
        if cur_val > target {
            return false;
        }

        is_match(target, cur_val * operands[0], &operands[1..])
            || is_match(target, cur_val + operands[0], &operands[1..])
    }

    let valid_lines = input
        .lines()
        .map(parse_line)
        .filter(|(target, operands)| is_match(*target, operands[0], &operands[1..]));

    valid_lines.fold(0, |acc, (target, _)| acc + target)
}

fn part_2(input: &str) -> u64 {
    fn is_match(
        target: u64,
        cur_val: u64,
        operands: &[u64],
    ) -> bool {
        fn concat(
            lhs: u64,
            rhs: u64,
        ) -> u64 {
            (lhs.to_string() + &rhs.to_string()).parse::<u64>().unwrap()
        }
        if operands.len() == 0 {
            return cur_val == target;
        }
        if cur_val > target {
            return false;
        }

        is_match(target, cur_val * operands[0], &operands[1..])
            || is_match(target, cur_val + operands[0], &operands[1..])
            || is_match(target, concat(cur_val, operands[0]), &operands[1..])
    }

    let valid_lines = input
        .lines()
        .map(parse_line)
        .filter(|(target, operands)| is_match(*target, operands[0], &operands[1..]));
    for (tar, vals) in valid_lines.clone() {
        println!("{tar}: {}", vals.iter().map(u64::to_string).collect::<Vec<_>>().join(" "));
    }

    valid_lines.fold(0, |acc, (target, _)| acc + target)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day_07::{part_1, part_2};

    const SAMPLE_INPUT: &str = // Wrap
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn test_part_1_sample_input() {
        assert_eq!(3749, part_1(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_1() {
        let input = read_to_string("inputs/07.txt").unwrap();
        assert_eq!(538191549061, part_1(&input));
    }
    #[test]
    fn test_part_2_sample_input() {
        assert_eq!(11387, part_2(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_2() {
        let input = read_to_string("inputs/07.txt").unwrap();
        assert_eq!(34612812972206, part_2(&input));
    }
}
