use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = parse_input("inputs/day_9_sample.txt");
        let result = calculate_new_values(input);

        assert_eq!(result, 114);
    }
    #[test]
    fn part_1_final() {
        let input = parse_input("inputs/day_9.txt");
        let result = calculate_new_values(input);

        assert_eq!(result, 1702218515);
    }

    #[test]
    fn part_2_sample() {
        let input = parse_input("inputs/day_9_sample.txt");
        let result = calculate_left_values(input);

        assert_eq!(result, 2);
    }
    #[test]
    fn part_2_final() {
        let input = parse_input("inputs/day_9.txt");
        let result = calculate_left_values(input);

        assert_eq!(result, 925);
    }
}

fn main() {}

#[derive(Debug)]
struct ValueHistory {
    layers: Vec<Vec<i64>>,
}

impl ValueHistory {
    fn from_line(line: &str) -> Self {
        let layers = vec![line.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect()];
        ValueHistory {
            layers,
        }
    }
}

fn calculate_left_values(input: Vec<ValueHistory>) -> i64 {
    dbg!(input).into_iter().fold(0, |acc, mut val| {
        val.calculate_rows();
        acc + val.calculate_prev_value()
    })
}

fn calculate_new_values(input: Vec<ValueHistory>) -> i64 {
    dbg!(input).into_iter().fold(0, |acc, mut val| {
        val.calculate_rows();
        acc + val.calculate_next_value()
    })
}

impl ValueHistory {
    fn calculate_rows(&mut self) {
        fn calc_next_row(vec: &Vec<i64>) -> Vec<i64> {
            let mut new_row = Vec::new();
            for items in vec.windows(2) {
                new_row.push(items[1] - items[0]);
            }
            new_row
        }
        while let Some(layer) = self.layers.last().filter(|vec| vec.iter().any(|n| *n != 0)) {
            self.layers.push(calc_next_row(layer));
        }
    }

    fn calculate_prev_value(&mut self) -> i64 {
        let mut iter = self.layers.iter_mut().rev();

        let mut prev_val = 0;
        while let Some(row) = iter.next() {
            prev_val = row.first().unwrap() - prev_val;
        }
        prev_val
    }
    fn calculate_next_value(&mut self) -> i64 {
        let mut iter = self.layers.iter_mut().rev();

        let mut prev_val = 0;
        while let Some(row) = iter.next() {
            prev_val = row.last().unwrap() + prev_val;
            row.push(prev_val);
        }
        prev_val
    }
}

fn parse_input(filename: &str) -> Vec<ValueHistory> {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut vec = Vec::new();
    for line in reader.lines() {
        vec.push(ValueHistory::from_line(&line.unwrap()));
    }
    vec
}
