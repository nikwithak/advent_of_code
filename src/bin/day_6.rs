#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample_input() {
        let races = parse_input("inputs/day_6_sample.txt");
        let val = races.iter().fold(1, |acc, race| {
            let (min, max) = dbg!(race.get_button_times());
            (max - min + 1) * acc
        });
        assert_eq!(val, 288);
    }
    #[test]
    fn part_2_sample_input() {
        let race = parse_input_ignore_whitespace("inputs/day_6_sample.txt");
        let (min, max) = dbg!(race.get_button_times());
        let val = max - min + 1;
        assert_eq!(val, 71503);
    }
    #[test]
    fn part_1_final_input() {
        let races = parse_input("inputs/day_6.txt");
        let val = races.iter().fold(1, |acc, race| {
            let (min, max) = dbg!(race.get_button_times());
            (max - min + 1) * acc
        });
        assert_eq!(val, 503424);
    }
    #[test]
    fn part_2_final_input() {
        let race = parse_input_ignore_whitespace("inputs/day_6.txt");
        let (min, max) = dbg!(race.get_button_times());
        let val = max - min + 1;
        assert_eq!(val, 32607562);
    }
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_button_times(&self) -> (u64, u64) {
        let t = self.time as f64;
        let r = self.distance as f64;

        let min = (t - dbg!(t * t - 4. * r).sqrt()) / 2.;
        let max = (t + (t * t - 4. * r).sqrt()) / 2.;

        let mut min = min.ceil() as u64;
        let mut max = max.floor() as u64;

        // Verify the edge cases:
        if (t - min as f64) * min as f64 <= r {
            min += 1;
        }
        if (t - max as f64) * max as f64 <= r {
            max -= 1;
        }
        if max < min {
            max = min
        }

        (min, max)
    }
}

fn parse_input_ignore_whitespace(filename: &str) -> Race {
    let input_str = std::fs::read_to_string(filename).unwrap();
    let values: Vec<u64> = input_str
        .lines()
        .into_iter()
        .map(|s| s.split(":").last().unwrap())
        .map(|s| s.split_whitespace().collect())
        .map(|s: Vec<&str>| s.join(""))
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let time = dbg!(&values)[0];
    let distance = values[1];

    Race {
        time,
        distance,
    }
}

fn parse_input(filename: &str) -> Vec<Race> {
    let input_str = std::fs::read_to_string(filename).unwrap();
    let values: Vec<Vec<u64>> = input_str
        .lines()
        .into_iter()
        .map(|s| s.split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect())
        .collect();

    let times = &dbg!(&values)[0];
    let distances = &values[1];

    assert!(times.len() == distances.len());

    times
        .iter()
        .enumerate()
        .map(|(i, t)| Race {
            time: *t,
            distance: distances[i],
        })
        .collect()
}

fn main() {
    let races = parse_input("inputs/day_6.txt");
    let val = races.iter().fold(1, |acc, race| {
        let (min, max) = dbg!(race.get_button_times());
        (max - min + 1) * acc
    });
    println!("Part 1: {}", val);

    let race = parse_input_ignore_whitespace("inputs/day_6.txt");
    let (min, max) = dbg!(race.get_button_times());
    let val = max - min + 1;
    println!("Part 2: {}", val);
}
