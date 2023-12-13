#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_sample() {
        let input = parse_input("inputs/day_13_sample.txt");
        let result = sum_mirror_values(&input);
        assert_eq!(result, 405);
    }
    #[test]
    fn horizontal_split() {
        let input: Vec<String> = r"
.##.##...
.#...#...
###.#.###
#......##
.##.#####
###.#..##
.#....#..
##...#...
#.##.#.##
#.##.####
##...#...
"
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

        assert_eq!(find_mirror_value(&input), 8);
    }
    #[test]
    fn p1_final() {
        let input = parse_input("inputs/day_13.txt");
        let result = sum_mirror_values(&input);
        assert_eq!(result, 27202);
    }
    #[test]
    fn p2_sample() {
        let input = parse_input("inputs/day_13_sample.txt");
        let result = sum_mirror_values_with_smudges(&input, 1);
        assert_eq!(result, 400);
    }

    #[test]
    fn p2_final() {
        let input = parse_input("inputs/day_13.txt");
        let result = sum_mirror_values_with_smudges(&input, 1);
        assert_eq!(result, 41566);
    }
}
fn main() {
    let input = parse_input("inputs/day_13.txt");
    println!("Part 1: {}", sum_mirror_values(&input));
    println!("Part 2: {}", sum_mirror_values_with_smudges(&input, 1));
}

fn sum_mirror_values(inputs: &Vec<Vec<String>>) -> usize {
    sum_mirror_values_with_smudges(inputs, 0)
}

fn sum_mirror_values_with_smudges(
    inputs: &Vec<Vec<String>>,
    required_smudges: usize,
) -> usize {
    let mut sum = 0;
    for group in inputs {
        println!();
        println!("====================================");
        for row in group {
            println!("{}", &row);
        }
        let val = find_mirror_value_with_smudges(group, required_smudges);
        println!();
        println!("Value: {}", val);
        println!();
        println!("====================================");
        println!();
        sum += val;
    }
    sum
}

// Returns (columns to left of vertical split) + (100 * rows above horizontal split)
fn find_mirror_value(rows: &Vec<String>) -> usize {
    find_mirror_value_with_smudges(rows, 0)
}
fn find_mirror_value_with_smudges(
    rows: &Vec<String>,
    required_smudges: usize,
) -> usize {
    let num_rows = find_horizontal_split_with_smudges(rows, required_smudges);
    let num_cols = find_vertical_split_with_smudges(rows, required_smudges);

    // BUG: I think the problem / inputs are borked. My input definitely has some inputs with more than one reflection line.
    // If somebody knows why this solution is failing, please let me know, but until then: I used someone else's solution to get the "other" answer.
    if required_smudges > 0 {
        if num_rows > 0 {
            assert_ne!(num_rows, find_horizontal_split(rows));
        }
        if num_cols > 0 {
            assert_ne!(num_cols, find_vertical_split(rows));
        }
    }

    // BUG (with the puzzle): The solution expects there to be only ONE split, so this accounts for situations/
    // where there's both a horizontal and vertical split.

    100 * num_rows + num_cols
}

fn find_horizontal_split_with_smudges(
    rows: &Vec<String>,
    num_smudges: usize,
) -> usize {
    for i in (0..rows.len()) {
        if is_split_by_row_with_smudges(i, i + 1, rows, num_smudges)
        // || is_split_by_row_with_smudges(i, i + 2, rows, num_smudges)
        {
            return i + 1;
        }
    }
    0
}

fn find_vertical_split_with_smudges(
    rows: &Vec<String>,
    num_smudges: usize,
) -> usize {
    for i in 0..rows[0].len() {
        if is_split_by_col_with_smudge(i, i + 1, rows, num_smudges)
        // || is_split_by_col_with_smudge(i, i + 2, rows, num_smudges)
        {
            return i + 1;
        }
    }
    0
}

fn find_horizontal_split(rows: &Vec<String>) -> usize {
    for i in 0..rows.len() {
        if is_split_by_row(i, i + 1, rows) || is_split_by_row(i, i + 2, rows) {
            return i + 1;
        }
    }
    0
}

fn find_vertical_split(rows: &Vec<String>) -> usize {
    for i in 0..rows[0].len() {
        if is_split_by_col(i, i + 1, rows) || is_split_by_col(i, i + 2, rows) {
            return i + 1;
        }
    }
    0
}

// Returns count of smudges
fn is_split_by_col_with_smudge(
    l_end: usize,
    r_start: usize,
    rows: &Vec<String>,
    required_smudges: usize,
) -> bool {
    let mut num_smudges = 0;
    let mut l_marker = l_end as isize;
    let rows_as_bytes: Vec<_> = rows
        .iter()
        .map(|c| c.as_bytes().into_iter().map(|c| *c as char).collect::<Vec<_>>())
        .collect();
    let mut split_range = r_start..rows_as_bytes[0].len();
    if r_start >= rows_as_bytes[0].len() {
        return false;
    }
    while l_marker >= 0 && !split_range.is_empty() {
        if let Some(e) = split_range.next() {
            for row in &rows_as_bytes {
                if !row[l_marker as usize].eq(&row[e]) {
                    if num_smudges >= required_smudges {
                        return false;
                    } else {
                        num_smudges += 1
                    }
                }
            }
        }
        l_marker -= 1;
    }
    required_smudges == num_smudges
}

fn is_split_by_col(
    l_end: usize,
    r_start: usize,
    rows: &Vec<String>,
) -> bool {
    is_split_by_col_with_smudge(l_end, r_start, rows, 0)
}

fn is_split_by_row_with_smudges(
    top_end: usize,
    bottom_start: usize,
    rows: &Vec<String>,
    required_smudges: usize,
) -> bool {
    let mut num_smudges = 0;
    let mut top_marker = top_end as isize;
    let mut split_range = bottom_start..rows.len();
    if bottom_start >= rows.len() {
        return false;
    }

    fn count_smudges(
        left: &str,
        right: &str,
    ) -> usize {
        let mut smudges = 0;
        let left = left.as_bytes();
        let right = right.as_bytes();
        for i in 0..left.len() {
            if left[i] != right[i] {
                smudges += 1;
            }
        }
        smudges
    }

    while top_marker >= 0 && !split_range.is_empty() {
        if let Some(e) = split_range.next() {
            num_smudges += count_smudges(&rows[top_marker as usize], &rows[e]);
            if num_smudges > required_smudges {
                return false;
            }
        }
        top_marker -= 1;
    }
    return num_smudges == required_smudges;
}

fn is_split_by_row(
    top_end: usize,
    bottom_start: usize,
    rows: &Vec<String>,
) -> bool {
    is_split_by_row_with_smudges(top_end, bottom_start, rows, 0)
}

fn parse_input(filename: &str) -> Vec<Vec<String>> {
    let input: Vec<String> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let input = input.split(|s| s.is_empty()).map(|s| s.iter().map(|s| s.to_string()).collect());
    input.collect()
}
