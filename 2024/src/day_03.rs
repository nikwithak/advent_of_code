use regex::Regex;

pub fn run_day_3() {
    let input = std::fs::read_to_string("inputs/03.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    let regex =
        regex::Regex::new("mul\\((?<first>\\d{1,3}),(?<second>\\d{1,3})\\)").expect("Bad regex");
    let mut total = 0;
    for (_, [first, second]) in regex.captures_iter(input).map(|c| c.extract()) {
        total += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }
    total
}

fn part_2(input: &str) -> i32 {
    // Newlines are messing up the regex matching.
    let input = input.replace("\n", "");
    // Remove all the bits in the middle that we don't care about.
    let cleanup_regex = Regex::new("don't\\(\\).*?do\\(\\)").unwrap();
    // Now that we have the ignored bits removed, it's just the same problem as part 1
    let input = cleanup_regex.replace_all(&input, "x");
    part_1(&*input)
}

#[test]
fn test_part_1_sample_input() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, part_1(input));
}

#[test]
fn test_part_1_input() {
    let input = std::fs::read_to_string("inputs/03.txt").unwrap();
    assert_eq!(173731097, part_1(&input));
}

#[test]
fn test_part_2_sample_input() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, part_2(input));
}

#[test]
fn test_part_2_input() {
    let input = std::fs::read_to_string("inputs/03.txt").unwrap();
    assert_eq!(93729253, part_2(&input));
}
