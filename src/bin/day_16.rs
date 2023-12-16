use std::collections::HashSet;

#[test]
fn test_part_1() {
    assert_eq!(46, part_1("inputs/day_16_sample.txt"));
    assert_eq!(6906, part_1("inputs/day_16.txt"));
}
#[test]
fn test_part_2() {
    assert_eq!(51, part_2("inputs/day_16_sample.txt"));
    assert_eq!(7330, part_2("inputs/day_16.txt"));
}
fn main() {
    println!("Part 1: {}", part_1("inputs/day_16.txt"));
    println!("Part 2: {}", part_2("inputs/day_16.txt"));
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}
type D = Direction;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Beam {
    x: usize,
    y: usize,
    dir: Direction,
}
impl Beam {
    fn new(
        x: usize,
        y: usize,
        dir: Direction,
    ) -> Self {
        Self {
            x,
            y,
            dir,
        }
    }
}

fn part_2(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|c| *c as char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ret = 0;
    let mut start_points = Vec::new();
    for y in 0..input.len() {
        start_points.push(Beam::new(0, y, D::E));
        start_points.push(Beam::new(input[y].len() - 1, y, D::W));
    }
    for x in 0..input[0].len() {
        start_points.push(Beam::new(x, 0, D::S));
        start_points.push(Beam::new(x, input.len() - 1, D::N));
    }
    while let Some(beam) = start_points.pop() {
        ret = calulate_energized_tiles(&input, beam).max(ret);
    }
    ret
}
fn part_1(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|c| *c as char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    calulate_energized_tiles(&input, Beam::new(0, 0, D::E))
}

fn calulate_energized_tiles(
    input: &Vec<Vec<char>>,
    start_beam: Beam,
) -> usize {
    let mut energized_tiles_count = 0;
    let mut stack = vec![start_beam];
    let mut memo: HashSet<Beam> = HashSet::new();
    let mut memo_2: HashSet<(usize, usize)> = HashSet::new();

    // If we find a loop, we're done!
    while let Some(beam) = stack.pop() {
        let (x, y, dir) = (beam.x, beam.y, beam.dir);
        if memo.insert(beam) && y < input.len() && x < input[y].len() {
            match (dir, input[y][x]) {
                (D::E, '|') | (D::W, '|') => stack.append(&mut vec![
                    Beam::new(x, y + 1, D::S),
                    Beam::new(x, y.saturating_sub(1), D::N),
                ]),
                (D::N, '-') | (D::S, '-') => stack.append(&mut vec![
                    Beam::new(x.saturating_sub(1), y, D::W),
                    Beam::new(x.saturating_add(1), y, D::E),
                ]),
                (D::E, '/') => stack.append(&mut vec![Beam::new(x, y.saturating_sub(1), D::N)]),
                (D::S, '/') => stack.append(&mut vec![Beam::new(x.saturating_sub(1), y, D::W)]),
                (D::W, '/') => stack.append(&mut vec![Beam::new(x, y + 1, D::S)]),
                (D::N, '/') => stack.append(&mut vec![Beam::new(x.saturating_add(1), y, D::E)]),
                (D::W, '\\') => stack.append(&mut vec![Beam::new(x, y.saturating_sub(1), D::N)]),
                (D::N, '\\') => stack.append(&mut vec![Beam::new(x.saturating_sub(1), y, D::W)]),
                (D::E, '\\') => stack.append(&mut vec![Beam::new(x, y + 1, D::S)]),
                (D::S, '\\') => stack.append(&mut vec![Beam::new(x.saturating_add(1), y, D::E)]),
                (D::N, _) => stack.append(&mut vec![Beam::new(x, y.saturating_sub(1), D::N)]),
                (D::E, _) => stack.append(&mut vec![Beam::new(x.saturating_add(1), y, D::E)]),
                (D::S, _) => stack.append(&mut vec![Beam::new(x, y + 1, D::S)]),
                (D::W, _) => stack.append(&mut vec![Beam::new(x.saturating_sub(1), y, D::W)]),
            }
            if memo_2.insert((x, y)) {
                energized_tiles_count += 1;
                // println!("ENERGIZED: ({}, {}, {:?}) {}", x, y, &dir, &energized_tiles_count);
            }
        }
    }

    energized_tiles_count
}
