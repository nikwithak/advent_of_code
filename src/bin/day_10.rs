use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_samples() {
        let input = load_input("inputs/day_10_sample.txt");
        assert_eq!(find_farthest_distance_from_s(&input), 4);

        let input = load_input("inputs/day_10_sample_2.txt");
        assert_eq!(find_farthest_distance_from_s(&input), 8);
    }
    #[test]
    fn part_1_final() {
        let input = load_input("inputs/day_10.txt");
        assert_eq!(find_farthest_distance_from_s(&input), 8);
    }
}

fn find_s(input: &Vec<Vec<u8>>) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, s)| {
            s.iter()
                .enumerate()
                .find_map(|(x, s)| {
                    if s.eq(&('S' as u8)) {
                        Some(x)
                    } else {
                        None
                    }
                })
                .map(|x| Some((x, y)))
        })
        .flatten()
        .unwrap()
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn next_dir(
    path: &char,
    from_dir: &Direction,
) -> Option<Direction> {
    type D = Direction;
    match (path, from_dir) {
        ('L', D::S) => Some(D::E),
        ('L', D::W) => Some(D::N),
        ('7', D::E) => Some(D::S),
        ('7', D::N) => Some(D::W),
        ('J', D::E) => Some(D::N),
        ('J', D::S) => Some(D::W),
        ('F', D::W) => Some(D::S),
        ('F', D::N) => Some(D::E),
        ('-', D::W) => Some(D::W),
        ('-', D::E) => Some(D::E),
        ('|', D::S) => Some(D::S),
        ('|', D::N) => Some(D::N),
        _ => None,
    }
}

fn find_start_dir(
    x: usize,
    y: usize,
    input: &Vec<Vec<u8>>,
) -> Direction {
    assert!(input[y][x] as char == 'S');
    if vec!['7', 'F', '|'].contains(&(input[y - 1][x] as char)) {
        Direction::N
    } else if vec!['L', 'J', '|'].contains(&(input[y + 1][x] as char)) {
        Direction::S
    } else if vec!['-', 'F', 'L'].contains(&(input[y][x - 1] as char)) {
        Direction::W
    } else if vec!['7', 'J', '1'].contains(&(input[y][x + 1] as char)) {
        Direction::E
    } else {
        panic!("No path found!!")
    }
}

fn find_farthest_distance_from_s(input: &Vec<Vec<u8>>) -> u64 {
    let start = find_s(input);
    let mut direction = find_start_dir(start.0, start.1, input);
    let (mut x, mut y) = start;
    let mut total_dist = 1;

    match direction {
        Direction::N => y -= 1,
        Direction::S => y += 1,
        Direction::E => x += 1,
        Direction::W => x -= 1,
    }

    while let Some(dir) = next_dir(&(dbg!(input[y][x] as char)), &direction) {
        println!("({}, {}) = {}: {:?}", x, y, input[y][x] as char, &dir);
        match dir {
            Direction::N => y -= 1,
            Direction::S => y += 1,
            Direction::E => x += 1,
            Direction::W => x -= 1,
        }
        total_dist += 1;
        direction = dir;
    }

    match total_dist % 2 {
        1 => total_dist / 2 + 1,
        0 => total_dist / 2,
        _ => panic!(),
    }
}

fn load_input(filename: &str) -> Vec<Vec<u8>> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.lines().into_iter().map(|s| s.as_bytes().into()).collect()
}

fn main() {}
