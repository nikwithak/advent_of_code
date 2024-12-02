// Historian Hysteria

use crate::Error;

pub fn run_day_2() {
    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    println!("Total safe rows (part 1): {}", part_1(&input).unwrap());
    println!("Total safe rows (part 2): {}", part_2(&input).unwrap());
}

pub fn part_1(input: &str) -> Result<usize, Error> {
    Ok(input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>())
        // Note: Some(usize::MAX) is used here to allow us to use the same function for parts 1 & 2.
        // This will fail the edge case where there are usize::MAX values in the list, and the last value triggers an Unsafe condition.
        .filter(|s| is_safe_with_errors(s, Some(usize::MAX)))
        .count())
}

/// DEPRECATED
///
/// This is the original function for part 1, before consolidating them into a single function.
fn _is_safe(vals: &Vec<i32>) -> bool {
    let mut vals = vals.iter();

    let Some(mut prev) = vals.next() else {
        return false;
    };
    let mut direction = 0;
    while let Some(val) = vals.next() {
        let diff = val - prev;
        if diff == 0
            || (direction != 0 && diff / diff.abs() != direction)
            || diff.abs() < 1
            || diff.abs() > 3
        {
            return false;
        }
        if direction == 0 {
            direction = diff / diff.abs();
        }
        prev = val;
    }
    true
}

fn is_safe_with_errors(
    input_vals: &Vec<i32>,
    skip: Option<usize>,
) -> bool {
    let mut vals = input_vals.iter().enumerate();

    if let Some(0) = skip {
        let _ = vals.next();
    }
    let Some((_i, mut prev)) = vals.next() else {
        return true;
    };
    let mut direction = 0;
    while let Some((_i, val)) = vals.next() {
        if skip.map_or(false, |skip| skip == _i) {
            continue;
        }
        let diff = val - prev;
        if diff == 0
            || (direction != 0 && diff / diff.abs() != direction)
            || diff.abs() < 1
            || diff.abs() > 3
        {
            if skip.is_none() {
                return is_safe_with_errors(input_vals, Some(_i))
                    || is_safe_with_errors(input_vals, Some(_i - 1));
            } else {
                return false;
            }
        }
        if direction == 0 {
            direction = diff / diff.abs();
        }
        prev = val;
    }
    true
}

/// Calculates the number of "safe" lines.
pub fn part_2(input: &str) -> Result<usize, Error> {
    Ok(input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .filter(|s| dbg!(is_safe_with_errors(s, None)))
        .count())
}

#[test]
fn test_part_1() -> Result<(), Error> {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(2, part_1(&input)?);

    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    assert_eq!(639, part_1(&input)?);
    Ok(())
}

#[test]
fn test_part_2() -> Result<(), Error> {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(4, part_2(&input)?);

    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    assert_eq!(674, part_2(&input)?);
    Ok(())
}
