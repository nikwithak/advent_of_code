use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

pub fn run_day_08() {
    let input = std::fs::read_to_string("inputs/08.txt").unwrap();
    println!("Day 08 Part 1: {}", part_1(&input));
    println!("Day 08 Part 2: {}", part_2(&input));
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}
type NodeMap = HashMap<char, Vec<Point>>;

fn build_nodemap(input: &str) -> (NodeMap, usize, usize) {
    let mut map: NodeMap = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let antenna = map.entry(c).or_insert(Vec::new());
                antenna.push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
            max_y = y;
        }
        max_x = x;
    }
    (map, max_x, max_y)
}

// impl Sub<(i64, i64)> for Point {
//     type Output = Point;

//     fn sub(
//         self,
//         rhs: (i64, i64),
//     ) -> Self::Output {
//         Point {
//             x: self.x - rhs.0,
//             y: self.y - rhs.1,
//         }
//     }
// }
// impl Add<(i64, i64)> for Point {
//     type Output = Point;

//     fn add(
//         self,
//         rhs: (i64, i64),
//     ) -> Self::Output {
//         Point {
//             x: self.x + rhs.0,
//             y: self.y + rhs.1,
//         }
//     }
// }
impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(
        self,
        rhs: &Point,
    ) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Add<&Point> for &Point {
    type Output = Point;

    fn add(
        self,
        rhs: &Point,
    ) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn find_antinodes(
    lhs: &Point,
    rhs: &Point,
    max_x: i64,
    max_y: i64,
) -> Vec<Point> {
    let dist = rhs - lhs;
    let mut result = Vec::new();
    let mut cur = lhs.clone();
    while cur.x >= 0 && cur.y >= 0 && cur.x <= max_x && cur.y <= max_y {
        result.push(cur.clone());
        cur = &cur - &dist;
    }
    cur = rhs.clone();
    while cur.x >= 0 && cur.y >= 0 && cur.x <= max_x && cur.y <= max_y {
        result.push(cur.clone());
        cur = &cur + &dist;
    }

    result
}
fn find_antinode_pairs(
    lhs: &Point,
    rhs: &Point,
) -> Vec<Point> {
    let node_a = Point {
        x: lhs.x - (rhs.x - lhs.x),
        y: lhs.y - (rhs.y - lhs.y),
    };
    let node_b = Point {
        x: rhs.x + (rhs.x - lhs.x),
        y: rhs.y + (rhs.y - lhs.y),
    };
    vec![node_a, node_b]
}
fn find_all_antinodes(
    nodes: &NodeMap,
    max_x: i64,
    max_y: i64,
) -> HashMap<Point, Vec<char>> {
    let mut antinodes = HashMap::new();
    for (c, points) in nodes {
        let mut lhs_iter = points.iter();
        while let Some(lhs) = lhs_iter.next() {
            let mut rhs_iter = lhs_iter.clone();
            while let Some(rhs) = rhs_iter.next() {
                println!(
                    "Looking for antinodes between ({}, {}) and ({}, {})",
                    &lhs.x, &lhs.y, &rhs.x, &rhs.y
                );
                if lhs == rhs {
                    continue;
                }
                for antinode in find_antinode_pairs(lhs, rhs) {
                    println!("antinode for {c} at ({},{})", &antinode.x, &antinode.y);
                    if antinode.x >= 0
                        && antinode.y >= 0
                        && antinode.x <= max_x
                        && antinode.y <= max_y
                    {
                        antinodes.entry(antinode).or_insert(Vec::new()).push(*c);
                    }
                }
                println!("Antinodes has {} antinodes", antinodes.len());
            }
        }
    }
    antinodes
}
fn part_1(input: &str) -> u64 {
    let (map, max_x, max_y) = build_nodemap(input);
    find_all_antinodes(&map, max_x as i64, max_y as i64).len() as u64
}

fn part_2(input: &str) -> u64 {
    let (map, max_x, max_y) = build_nodemap(input);
    fn find_all_antinodes(
        nodes: &NodeMap,
        max_x: i64,
        max_y: i64,
    ) -> HashMap<Point, Vec<char>> {
        let mut antinodes = HashMap::new();
        for (c, points) in nodes {
            let mut lhs_iter = points.iter();
            while let Some(lhs) = lhs_iter.next() {
                let mut rhs_iter = lhs_iter.clone();
                while let Some(rhs) = rhs_iter.next() {
                    for antinode in find_antinodes(lhs, rhs, max_x, max_y) {
                        if antinode.x >= 0
                            && antinode.y >= 0
                            && antinode.x <= max_x
                            && antinode.y <= max_y
                        {
                            antinodes.entry(antinode).or_insert(Vec::new()).push(*c);
                        }
                    }
                }
            }
        }
        antinodes
    }
    find_all_antinodes(&map, max_x as i64, max_y as i64).len() as u64
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day_08::{part_1, part_2};

    const SAMPLE_INPUT: &str = // Wrap
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn test_part_1_sample_input() {
        assert_eq!(14, part_1(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_1() {
        let input = read_to_string("inputs/08.txt").unwrap();
        assert_eq!(256, part_1(&input));
    }
    #[test]
    fn test_part_2_sample_input() {
        assert_eq!(34, part_2(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_2() {
        let input = read_to_string("inputs/08.txt").unwrap();
        assert_eq!(0, part_2(&input));
    }
}
