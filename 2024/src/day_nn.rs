pub fn run_day_NN() {
    let input = std::fs::read_to_string("inputs/NN.txt").unwrap();
    println!("Day NN Part 1: {}", part_1(&input));
    println!("Day NN Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    todo!()
}
fn part_2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_NN::{part_1, part_2};
    use std::fs::read_to_string;

    const SAMPLE_INPUT: &str = // Wrap
        "";
    #[test]
    fn test_part_1_sample_input() {
        assert_eq!(0, part_1(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_1() {
        let input = read_to_string("inputs/NN.txt").unwrap();
        assert_eq!(0, part_1(&input));
    }
    #[test]
    fn test_part_2_sample_input() {
        assert_eq!(0, part_2(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_2() {
        let input = read_to_string("inputs/NN.txt").unwrap();
        assert_eq!(0, part_2(&input));
    }
}
