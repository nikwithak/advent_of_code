use std::{
    arch::x86_64::__cpuid,
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
    str::Chars,
};

use regex::{Regex, RegexSet};

fn main() {
    println!("Part 1: Sum of IDs of valid games: {}", sum_id_of_valid_games());
    println!(
        "Part 2: Sum of power of all games: {}",
        get_total_power_for_input("inputs/day_2.txt")
    );
}

struct Game {
    id: u64,
    rounds: Vec<GameRound>,
}
#[derive(Debug)]
struct GameRound {
    blue: u64,
    red: u64,
    green: u64,
}

fn _parse_game_with_regex(line: &str) -> Game {
    // Get Game ID:
    let id = &Regex::new(r"Game (?<id>\d+): ").unwrap().captures(&line).unwrap()["id"];
    let rounds: Vec<GameRound> =
        Regex::new(r"([:;] ((?<red>\d+) red)|((?<blue>\d+) blue)|((?<green>\d+) green),?)+(;|$)")
            .unwrap()
            .captures_iter(&line)
            .map(|capture| GameRound {
                red: capture.name("red").map(|c| c.as_str().parse().unwrap()).unwrap_or(0),
                green: capture.name("green").map(|c| c.as_str().parse().unwrap()).unwrap_or(0),
                blue: capture.name("blue").map(|c| c.as_str().parse().unwrap()).unwrap_or(0),
            })
            .collect();
    Game {
        id: id.parse().unwrap(),
        rounds,
    }
}

fn parse_game_id(game_str: &str) -> u64 {
    Regex::new(r"Game (?<id>\d+)").unwrap().captures(game_str).unwrap()["id"]
        .parse()
        .unwrap()
}

fn parse_game_round(round_str: &str) -> GameRound {
    let parts = round_str.split(',').map(|p| p.trim().split(' '));
    let mut round = GameRound {
        blue: 0,
        red: 0,
        green: 0,
    };

    for mut part in parts {
        let amount = dbg!(&mut part).next().unwrap();
        let color = part.next().unwrap();
        match color {
            "red" => round.red += amount.parse::<u64>().unwrap(),
            "blue" => round.blue += amount.parse::<u64>().unwrap(),
            "green" => round.green += amount.parse::<u64>().unwrap(),
            _ => panic!("invalid game string"),
        }
    }

    round
}

fn parse_game_with_chars(line: &str) -> Game {
    let mut game_round_strs = line.split(&[';', ':']);
    let game_string = game_round_strs.next().unwrap();
    let game_id = parse_game_id(game_string);

    let mut rounds: Vec<GameRound> = Vec::new();

    for game_round in game_round_strs {
        rounds.push(parse_game_round(game_round))
    }

    Game {
        id: game_id,
        rounds,
    }
}

impl Game {
    fn get_minimum_contents(&self) -> GameRound {
        let mut min_contents = self.rounds.iter().fold(
            GameRound {
                blue: 0,
                red: 0,
                green: 0,
            },
            |mut acc, round| {
                acc.red = acc.red.max(round.red);
                acc.blue = acc.blue.max(round.blue);
                acc.green = acc.green.max(round.green);
                acc
            },
        );

        min_contents
    }

    fn get_power(&self) -> u64 {
        let contents = self.get_minimum_contents();
        let power = contents.red * contents.blue * contents.green;
        power
    }
}

fn parse_games(filename: &str) -> Vec<Game> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut games = Vec::new();

    for line in reader.lines() {
        let line = dbg!(line.expect("Whoops"));

        let game = parse_game_with_chars(&line);
        games.push(game);
    }
    games
}

fn get_total_power_for_input(filename: &str) -> u64 {
    parse_games(filename)
        .iter()
        .map(|game| game.get_power())
        .fold(0, |mut acc, power| acc + power)
}

fn sum_id_of_valid_games() -> u64 {
    let games = parse_games("inputs/day_2.txt");

    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    let mut result: u64 = 0;

    for game in games {
        let Game {
            id,
            rounds,
            // } = _parse_game_with_regex(&line);
        } = game;

        if dbg!(rounds)
            .iter()
            .find(|round| round.blue > MAX_BLUE || round.green > MAX_GREEN || round.red > MAX_RED)
            .is_none()
        {
            result += id;
        }
    }
    result
}
