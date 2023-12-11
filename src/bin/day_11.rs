use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample() {
        let input = parse_input("inputs/day_11_sample.txt");
        assert_eq!(sum_of_shortest_paths(input, 1), 374);
    }
    #[test]
    fn part_1_final() {
        let input = parse_input("inputs/day_11.txt");
        assert_eq!(sum_of_shortest_paths(input, 1), 9648398);
    }

    #[test]
    fn part_2_sample() {
        let input = parse_input("inputs/day_11_sample.txt");
        assert_eq!(sum_of_shortest_paths(input, 99), 8410);
    }
    #[test]
    fn part_2_final() {
        let input = parse_input("inputs/day_11.txt");

        assert_eq!(sum_of_shortest_paths(input, 999999), 9648398);
    }
}

struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(
        x: usize,
        y: usize,
    ) -> Self {
        Self {
            x,
            y,
        }
    }
}

struct Solution {
    galaxy_locations: Vec<Point>,
    rows_with_galaxies: HashSet<usize>,
    cols_with_galaxies: HashSet<usize>,
}

fn get_shortest_path(
    a: &Point,
    b: &Point,
    input: &Solution,
    size_of_empty: usize,
) -> usize {
    let y_range = min(a.y, b.y)..max(a.y, b.y);
    let x_range = min(a.x, b.x)..max(a.x, b.x);
    let mut num_col_gaps = y_range.filter(|col| !input.cols_with_galaxies.contains(col)).count();
    let mut num_row_gaps = x_range.filter(|row| !input.rows_with_galaxies.contains(row)).count();

    ((a.x.abs_diff(b.x) + num_row_gaps * size_of_empty)
        + (a.y.abs_diff(b.y) + num_col_gaps * size_of_empty))
}

fn sum_of_shortest_paths(
    mut input: Solution,
    size_of_empty: usize,
) -> usize {
    let mut sum = 0;
    let galaxies = &input.galaxy_locations;
    for i in 0..input.galaxy_locations.len() {
        for j in i..input.galaxy_locations.len() {
            let path = get_shortest_path(&galaxies[i], &galaxies[j], &input, size_of_empty);
            sum += path;
        }
    }
    sum
}

fn parse_input(filename: &str) -> Solution {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.as_bytes().into_iter().map(|c| *c as char).collect())
        .collect::<Vec<Vec<char>>>();
    let mut galaxy_locations: Vec<Point> = Vec::new();
    let mut rows_with_galaxies = HashSet::<usize>::new();
    let mut cols_with_galaxies = HashSet::<usize>::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.eq(&'#') {
                // Galaxy found!
                galaxy_locations.push(Point::new(x, y));
                rows_with_galaxies.insert(x);
                cols_with_galaxies.insert(y);
            }
        }
    }

    Solution {
        galaxy_locations,
        rows_with_galaxies,
        cols_with_galaxies,
    }
}

fn main() {}
