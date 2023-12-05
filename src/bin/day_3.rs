use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    // Part 1
    // let input = parse_input("inputs/day_3_sample.txt");
    let input = parse_input("inputs/day_3.txt");
    let value = input.iter().fold(0, |acc, part| acc + dbg!(&part).id);
    println!("value: {}", &value);

    // Part 2: Gear Ratio
    let gears = input.iter().filter(|part| part.symbol.eq(&'*'));
    let gears = gears.fold(HashMap::<(u64, u64), Vec<u64>>::new(), |mut acc, gear| {
        match acc.get_mut(&gear.symbol_location) {
            Some(values) => {
                values.push(gear.id);
            },
            None => {
                acc.insert(gear.symbol_location, vec![gear.id]);
            },
        }
        acc
    });
    let gear_ratios = dbg!(&gears)
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.iter().fold(1, |acc, value| acc * value));
    let result = gear_ratios.fold(0, |acc, ratio| acc + ratio);
    println!("Gear Ratios: {}", &result)
}

#[derive(Debug)]
struct Part {
    id: u64,
    symbol: char,
    symbol_location: (u64, u64),
}

fn find_symbol(
    string: &str,
    top_left_x: usize,
    top_left_y: usize,
    bottom_right_x: usize,
    bottom_right_y: usize,
) -> Option<(char, usize, usize)> {
    let lines = &string.lines().collect::<Vec<&str>>();
    let lines = &lines[top_left_y..=bottom_right_y.min(lines.len() - 1)];
    let symbol = lines
        .iter()
        .map(|line| &line[top_left_x..bottom_right_x.min(line.len())])
        .enumerate()
        .find_map(|(y, substr)| {
            substr.chars().enumerate().find_map(|(x, c)| match c {
                '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => None,
                _ => Some((c, x + top_left_x, y + top_left_y)),
            })
        });

    symbol
}

fn parse_input(filename: &str) -> Vec<Part> {
    let filestring = std::fs::read_to_string(filename).unwrap();
    // let lines: Vec<&[u8]> = filestring.lines().map(|line| line.as_bytes()).collect();
    let mut parts: Vec<Part> = Vec::new();
    let regex = Regex::new(r"(^|[^0-9])(?<part_id>\d+)").unwrap();
    for (y, row) in filestring.lines().enumerate() {
        let matches = regex.captures_iter(row);
        for number in matches {
            let number = number.name("part_id").unwrap();
            let symbol = find_symbol(
                &filestring,
                number.start().saturating_sub(1),
                y.saturating_sub(1),
                number.end().saturating_add(1),
                y.saturating_add(1),
            );
            if let Some(symbol) = symbol {
                parts.push(Part {
                    id: number.as_str().parse().unwrap(),
                    symbol: symbol.0,
                    symbol_location: (symbol.1 as u64, symbol.2 as u64),
                })
            } else {
                println!(
                    "Match not found: {:?} at bb {} {} {} {}",
                    number,
                    number.start().saturating_sub(1),
                    y.saturating_sub(1),
                    number.end().saturating_add(1),
                    y.saturating_add(1),
                )
            }
        }

        // for (y, row) in lines.iter().enumerate() {
        // let mut num_string = String::new();
        // let mut start: Option<u64> = None;

        // for (x, c) in row.iter().enumerate() {
        //     let x = x as u64;
        //     if *c >= ('0' as u8) && *c <= ('9' as u8) {
        //         if start.is_none() {
        //             start = Some(x);
        //         }
        //         num_string.push(*c as char);
        //     } else if let Some(start_x) = start {
        //         // We've finished parsing our string.
        //         // EDGE CASE (TODO): If string as at the end, we'll never hit this.
        //         let id: u64 = num_string.parse().unwrap();

        //         // Search for symbol:
        //         for x in (start_x.saturating_sub(1))..=x {
        //             for y in y.saturating_sub(1)..(y + 1).max(lines.len()) {
        //                 match lines[y][x as usize] {
        //                     b'.' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8'
        //                     | b'9' => {},
        //                     _ => {
        //                         parts.push(Part {
        //                             id,
        //                             symbol: lines[y][x as usize] as char,
        //                             location: (x, y as u64),
        //                         });
        //                     },
        //                 }
        //             }
        //         }

        //         // Clear everything:
        //         start = None;
        //         num_string = String::new();
        //     }
        // }
    }

    parts
}
