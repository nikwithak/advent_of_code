// Historian Hysteria

use std::collections::HashMap;

use crate::Error;

/// Runs the input and prints out the values from part 1 & part 2.
pub fn run_day_1() {
    let input = std::fs::read_to_string("inputs/01.txt").unwrap();
    println!("Total distance: {}", part_1(&input).unwrap());
    println!("Total similarity score: {}", part_2(&input).unwrap());
}

/// Calculates the total distance between the values in the two lists
/// by comparing the lowest from each list, then the next lowest from each list, etc.
pub fn part_1(input: &str) -> Result<usize, Error> {
    let (mut l, mut r) = parse(input)?;
    l.sort();
    r.sort();
    let mut sum = 0;
    for (l, r) in l.into_iter().zip(r) {
        sum += l.abs_diff(r);
    }
    Ok(sum)
}

/// Calculates the "Similarity Score". For a given number N:
/// L(N) = The number of times N appears in the left column.
/// R(N) = The number of times N appears in the right column
///
/// Similarity Score = N * L(N) * R(N)
pub fn part_2(input: &str) -> Result<usize, Error> {
    let (l, r) = parse(input)?;
    let mut binned: HashMap<usize, (usize, usize)> = HashMap::new();
    for l in l {
        let entry = binned.entry(l).or_insert((0, 0));
        entry.0 += 1;
    }

    for r in r {
        let entry = binned.entry(r).or_insert((0, 0));
        entry.1 += 1;
    }

    let result = binned.into_iter().fold(0, |total, (val, (l, r))| total + (val * l * r));
    Ok(dbg!(result))
}

/// Parses the input string into the two vectors - left column and right column.
pub fn parse(input: &str) -> Result<(Vec<usize>, Vec<usize>), Error> {
    let (mut l_vals, mut r_vals) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let mut vals = line.split_whitespace().map(|val| val.parse());
        let l_val = vals.next().ok_or("No left value found")?.unwrap();
        let r_val = vals.next().ok_or("No right value found")?.unwrap();
        l_vals.push(l_val);
        r_vals.push(r_val);
    }
    Ok((l_vals, r_vals))
}

#[test]
fn test_part_1() -> Result<(), Error> {
    let input = std::fs::read_to_string("inputs/01.txt").unwrap();
    assert_eq!(1223326, part_1(&input)?);
    Ok(())
}

#[test]
fn test_part_2() -> Result<(), Error> {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(31, part_2(input)?);
    Ok(())
}
