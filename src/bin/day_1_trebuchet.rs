use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::{Regex, RegexSet};

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    fn get_calibration_value(value: &str) -> u64 {
        let mut chars = value.chars();
        let first = chars.find(|c| c.is_digit(10)).expect("No first digit found");
        let second = chars.rev().find(|c| c.is_digit(10)).unwrap_or(first);

        let mut value_str = String::new();
        value_str.push(first);
        value_str.push(second);

        return value_str.parse().unwrap();
    }

    let mut calibration_values_sum: u64 = 0;
    let file = File::open("inputs/day_1_trebuchet.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val = get_calibration_value(&line.unwrap());
        calibration_values_sum += val;
    }
    println!("Sum of calibration values: {}", calibration_values_sum);
}

fn part_2() {
    fn get_calibration_value(value: &str) -> u64 {
        // let regex = Regex::new("^(?<first>[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)).*(?<second>[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(zero))?")hhhhhhhhhh
        // let regex =
        //     Regex::new("[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
        //         .unwrap();
        let regex = RegexSet::new(vec![
            "[0-9]",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
        ])
        .unwrap();
        // Regex::new("[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
        let mut matches = regex.find_iter(value).map(|digit| match digit.as_str() {
            "one" | "1" => '1',
            "two" | "2" => '2',
            "three" | "3" => '3',
            "four" | "4" => '4',
            "five" | "5" => '5',
            "six" | "6" => '6',
            "seven" | "7" => '7',
            "eight" | "8" => '8',
            "nine" | "9" => '9',
            "zero" | "0" => '0',
            _ => panic!("Expected match, didn't fine one"),
        });

        let first = matches.next().unwrap();
        let second = matches.last().unwrap_or(first);

        let mut value_str = String::new();
        value_str.push(first);
        value_str.push(second);
        println!("{}: {}", &value, &value_str);

        return value_str.parse().unwrap();
    }

    let mut calibration_values_sum: u64 = 0;

    let file = File::open("inputs/day_1_trebuchet.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val = get_calibration_value(&line.unwrap());
        calibration_values_sum += val;
    }
    println!("Sum of calibration values: {}", calibration_values_sum);
}
