use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_samples() {
        let mut input = load_input("inputs/day_10_sample.txt");
        assert_eq!(find_farthest_distance_from_s(&mut input), 4);

        let mut input = load_input("inputs/day_10_sample_2.txt");
        assert_eq!(find_farthest_distance_from_s(&mut input), 8);
    }
    #[test]
    fn part_1_final() {
        let mut input = load_input("inputs/day_10.txt");
        assert_eq!(find_farthest_distance_from_s(&mut input), 7093);
    }
    #[test]
    fn part_2_sample() {
        let mut input = load_input("inputs/day_10_part_2_sample.txt");
        assert_eq!(count_area_in_loop(&mut input), 10);
    }
    #[test]
    fn part_2_final() {
        let mut input = load_input("inputs/day_10.txt");
        assert_eq!(count_area_in_loop(&mut input), 0);
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
    (path, visited): &(u8, bool),
    from_dir: &Direction,
) -> Option<Direction> {
    type D = Direction;
    let path = *path as char;
    if *visited {
        return None;
    }
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
) -> (char, Direction) {
    let mut dirs: Vec<Direction> = Vec::new();
    assert!(input[y][x] as char == 'S');
    if vec!['7', 'F', '|'].contains(&(input[y.saturating_sub(1)][x] as char)) {
        dirs.push(Direction::N);
    }
    if vec!['L', 'J', '|'].contains(&(input[y + 1][x] as char)) {
        dirs.push(Direction::S);
    }
    if vec!['-', 'F', 'L'].contains(&(input[y][x.saturating_sub(1)] as char)) {
        dirs.push(Direction::W);
    }
    if vec!['7', 'J', '-'].contains(&(input[y][x + 1] as char)) {
        dirs.push(Direction::E);
    }
    dbg!(&dirs);
    let char = match (&dirs[0], &dirs[1]) {
        (&Direction::N, &Direction::E) | (&Direction::E, &Direction::N) => 'L',
        (&Direction::S, &Direction::E) | (&Direction::E, &Direction::S) => 'F',
        (&Direction::W, &Direction::E) | (&Direction::E, &Direction::W) => '-',

        (&Direction::S, &Direction::N) | (&Direction::N, &Direction::S) => '|',
        (&Direction::W, &Direction::N) | (&Direction::N, &Direction::W) => 'J',

        (&Direction::S, &Direction::W) | (&Direction::W, &Direction::S) => '7',
        _ => {
            dbg!(dirs);
            panic!()
        },
    };
    (char, dirs.pop().unwrap())
}

fn count_area_in_loop(input: &mut Vec<Vec<u8>>) -> u64 {
    let breadcrumbed = mark_path(input).1; // To add breadcrumbs
    println!(
        "{}",
        breadcrumbed
            .iter()
            .map(|line| line
                .iter()
                .map(|(b, part_of_path)| if *part_of_path {
                    *b as char
                } else {
                    ' '
                })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    let mut inside_area = 0;

    // Pretty sure I convoluted the heck out of this. Needed to track whether we are inside or outside a loop, based on the directions of the curve we hit.
    // Whatever, it works! not gonna fuss with it.
    #[derive(Debug)]
    enum InsideStatus {
        // Inside(c) tracks the last turn we hit in the wall/path (c)
        InsideTop,
        InsideMiddle,
        InsideBottom,
        Outside,
    }

    for row in breadcrumbed {
        let mut status = InsideStatus::Outside;
        for (space, part_of_path) in row {
            let c = &(space as char);
            // dbg!((c, &part_of_path, &status));
            match (part_of_path, &status, c) {
                (true, InsideStatus::Outside, 'L') => status = InsideStatus::InsideTop,
                (true, InsideStatus::Outside, '7') => status = InsideStatus::InsideMiddle,
                (true, InsideStatus::Outside, 'J') => status = InsideStatus::InsideMiddle,
                (true, InsideStatus::Outside, 'F') => status = InsideStatus::InsideBottom,
                (true, InsideStatus::Outside, '|') => status = InsideStatus::InsideMiddle,

                (true, InsideStatus::InsideTop, 'L') => status = InsideStatus::InsideBottom, //??
                (true, InsideStatus::InsideTop, '7') => status = InsideStatus::InsideMiddle,
                (true, InsideStatus::InsideTop, 'J') => status = InsideStatus::Outside,
                (true, InsideStatus::InsideTop, 'F') => status = InsideStatus::Outside,
                (true, InsideStatus::InsideTop, '|') => status = InsideStatus::Outside,

                (true, InsideStatus::InsideBottom, 'L') => status = InsideStatus::Outside,
                (true, InsideStatus::InsideBottom, '7') => status = InsideStatus::Outside,
                (true, InsideStatus::InsideBottom, 'J') => status = InsideStatus::InsideMiddle,
                (true, InsideStatus::InsideBottom, 'F') => status = InsideStatus::InsideTop, //??
                (true, InsideStatus::InsideBottom, '|') => status = InsideStatus::Outside,

                (true, InsideStatus::InsideMiddle, 'L') => status = InsideStatus::InsideBottom,
                (true, InsideStatus::InsideMiddle, '7') => status = InsideStatus::Outside,
                (true, InsideStatus::InsideMiddle, 'J') => status = InsideStatus::InsideMiddle,
                (true, InsideStatus::InsideMiddle, 'F') => status = InsideStatus::InsideTop, //??
                (true, InsideStatus::InsideMiddle, '|') => status = InsideStatus::Outside,

                (false, InsideStatus::InsideTop, _)
                | (false, InsideStatus::InsideBottom, _)
                | (false, InsideStatus::InsideMiddle, _) => inside_area += 1,
                (_, _, _) => {}, // Do nothing - walking along the path
            }
        }
    }

    inside_area
}

fn find_farthest_distance_from_s(input: &mut Vec<Vec<u8>>) -> u64 {
    let path = mark_path(input);
    let total_dist = path.0;
    match total_dist % 2 {
        1 => total_dist / 2 + 1,
        0 => total_dist / 2,
        _ => panic!(),
    }
}

///  Returns a tuple containing:
/// result.0 = The length of the path
/// result.1 = A copy of the original array, modified to include a bool indiciation for each space if it's part of the path.
fn mark_path(input: &mut Vec<Vec<u8>>) -> (u64, Vec<Vec<(u8, bool)>>) {
    let start = find_s(input);
    let (s_val, mut direction) = find_start_dir(start.0, start.1, input);
    let (mut x, mut y) = start;
    let mut input: Vec<Vec<(u8, bool)>> = input
        .iter()
        .map(|row| row.iter().map(|step| (*step, false)).collect())
        .collect();
    input[y][x].1 = true; // true = "Part of main path"
    input[y][x].0 = s_val as u8;

    let mut total_dist = 1;
    match direction {
        Direction::N => y -= 1,
        Direction::S => y += 1,
        Direction::E => x += 1,
        Direction::W => x -= 1,
    }

    while let Some(dir) = next_dir(&input[y][x], &direction) {
        println!("({}, {}) = {}: {:?}", x, y, input[y][x].0 as char, &dir);
        input[y][x].1 = true;
        match dir {
            Direction::N => y -= 1,
            Direction::S => y += 1,
            Direction::E => x += 1,
            Direction::W => x -= 1,
        }
        total_dist += 1;
        direction = dir;
    }

    (total_dist, input)
}

fn load_input(filename: &str) -> Vec<Vec<u8>> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.lines().into_iter().map(|s| s.as_bytes().into()).collect()
}

fn main() {}
