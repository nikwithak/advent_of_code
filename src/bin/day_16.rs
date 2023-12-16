use std::collections::HashSet;

#[test]
fn part_1_sample() {
    assert_eq!(46, part_1("inputs/day_16_sample.txt"));
}
fn main() {
    println!("Part 1: {}", part_1("inputs/day_16.txt"));
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}
type D = Direction;

fn part_1(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|c| *c as char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut energized_tiles_count = 0;
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
    let mut stack = vec![Beam::new(0, 0, D::E)];
    let mut memo: HashSet<Beam> = HashSet::new();
    let mut memo_2: HashSet<(usize, usize)> = HashSet::new();

    // If we find a loop, we're done!
    while let Some(beam) = stack.pop() {
        let (x, y, dir) = dbg!((beam.x, beam.y, beam.dir));
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
                println!("ENERGIZED: ({}, {}, {:?}) {}", x, y, &dir, &energized_tiles_count);
            }
        }
    }

    energized_tiles_count
}
