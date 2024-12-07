use std::{collections::HashSet, fmt::Display, fs::read_to_string, str::FromStr};

use crate::util::Direction;

pub fn run_day_6() {
    let input = std::fs::read_to_string("inputs/04.txt").unwrap();
    println!("Day 6 Part 1: {}", part_1(&input));
    println!("Day 6 Part 2: {}", part_2(&input));
}

#[derive(Default)]
pub struct Cursor2D {
    x: isize,
    y: isize,
    direction: Direction,
    bounds: Option<Bounds>,
}

#[derive(Default)]
pub struct Bounds {
    top_left_x: isize,
    top_left_y: isize,
    bottom_right_x: isize,
    bottom_right_y: isize,
}

impl Bounds {
    pub fn contains(
        &self,
        x: isize,
        y: isize,
    ) -> bool {
        self.top_left_x < x
            && x < self.bottom_right_x
            && self.top_left_y < y
            && y < self.bottom_right_y
    }
}

impl Cursor2D {
    pub fn move_forward(&mut self) -> bool {
        self.move_dir(&self.direction.clone())
    }
    pub fn look_forward(&mut self) -> (isize, isize) {
        let (move_x, move_y) = self.direction.get_coords();
        let (new_x, new_y) = (self.x + move_x, self.y + move_y);
        (new_x, new_y)
        // self.bounds
        //     .as_ref()
        //     .filter(|b| b.contains(new_x, new_y))
        //     .map(|_| (new_x, new_y))
    }
    pub fn turn_right(&mut self) {
        type D = Direction;
        self.direction = match self.direction {
            D::N => D::E,
            D::S => D::W,
            D::E => D::S,
            D::W => D::N,
            D::NE => todo!(),
            D::SE => todo!(),
            D::NW => todo!(),
            D::SW => todo!(),
        }
    }
    pub fn move_dir(
        &mut self,
        direction: &Direction,
    ) -> bool {
        let (move_x, move_y) = direction.get_coords();
        let (new_x, new_y) = (self.x + move_x, self.y + move_y);
        // if let Some(true) = self.bounds.as_ref().map(|b| b.contains(new_x, new_y)) {
        self.x = new_x;
        self.y = new_y;
        true
        // } else {
        // false
        // }
    }

    pub fn is_in_bounds(&mut self) -> bool {
        self.bounds.as_ref().map(|b| b.contains(self.x, self.y)).unwrap_or(true)
    }
}

struct Maze2D {
    maze_map: Vec<Vec<Location>>,
    cursor: Cursor2D,
}
impl Display for Maze2D {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for (x, line) in self.maze_map.iter().enumerate() {
            for (y, loc) in line.iter().enumerate() {
                let c = if self.cursor.x as usize == x && self.cursor.y as usize == y {
                    match self.cursor.direction {
                        Direction::N => '^',
                        Direction::S => 'v',
                        Direction::E => '>',
                        Direction::W => '<',
                        _ => todo!(),
                    }
                } else if loc.already_visited.len() > 0 {
                    'X'
                } else if loc.is_obstacle {
                    '#'
                } else {
                    ' '
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")?;
        writeln!(f, "")?;
        Ok(())
    }
}
impl FromStr for Maze2D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cursor = Cursor2D::default();
        let maze_map: Vec<Vec<Location>> = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        if c == '^' {
                            cursor.x = x as isize;
                            cursor.y = y as isize;
                        }
                        Location {
                            is_obstacle: c.eq(&'#'),
                            already_visited: HashSet::new(),
                        }
                    })
                    .collect()
            })
            .collect();
        cursor.bounds = Some(Bounds {
            top_left_x: 0,
            top_left_y: 0,
            bottom_right_x: maze_map.len() as isize,
            bottom_right_y: maze_map.get(0).map(|r| r.len()).unwrap_or_default() as isize,
        });
        cursor.direction = D::W; // Because x/y are actually flopped
        let result = Maze2D {
            maze_map,
            cursor,
        };
        Ok(result)
    }
}

impl Maze2D {
    pub fn load_file(filename: &str) -> Result<Self, std::io::Error> {
        Ok(Self::from_str(&read_to_string(filename)?).unwrap())
    }

    pub fn trace_path(&mut self) -> usize {
        let mut count = 0;

        // while let Some((x, y)) = self.cursor.look_forward() {
        //     let dir = self.cursor.direction.clone();
        //     self.get_current_location_mut().map(|l| l.visit_facing_dir(&dir));
        //     if let Some(loc) =
        //         self.get_mut(x, y).filter(|l| !l.is_already_visited_facing_dir(&dir)).as_mut()
        //     {
        //         if loc.is_obstacle {
        //             self.cursor.turn_right();
        //         }
        //         if self.cursor.move_forward() {
        //             count += 1;
        //         }
        //     } else {
        //         break;
        //     }
        //     println!("{self}");
        // }
        while self
            .get_current_location()
            .filter(|loc| !loc.is_already_visited_facing_dir(&self.cursor.direction))
            .is_some()
        {
            let dir = self.cursor.direction.clone();
            let loc = self.get_current_location_mut().unwrap();
            if loc.already_visited.is_empty() {
                count += 1;
            }
            loc.visit_facing_dir(&dir);
            let (next_x, next_y) = self.cursor.look_forward();
            if self.get(next_x, next_y).map(|l| l.is_obstacle).unwrap_or(false) {
                self.cursor.turn_right();
            } else {
                self.cursor.move_forward();
            }
            // println!("{self}");
        }
        count
    }
    pub fn get_mut(
        &mut self,
        x: isize,
        y: isize,
    ) -> Option<&mut Location> {
        self.maze_map.get_mut(x as usize).and_then(|r| r.get_mut(y as usize))
    }

    pub fn get(
        &self,
        x: isize,
        y: isize,
    ) -> Option<&Location> {
        self.maze_map.get(x as usize).and_then(|r| r.get(y as usize))
    }
    pub fn get_current_location_mut(&mut self) -> Option<&mut Location> {
        self.get_mut(self.cursor.x, self.cursor.y)
    }
    pub fn get_current_location(&self) -> Option<&Location> {
        self.get(self.cursor.x, self.cursor.y)
    }
}

struct Location {
    is_obstacle: bool,
    already_visited: HashSet<Direction>,
}
impl Location {
    pub fn is_already_visited_facing_dir(
        &self,
        direction: &Direction,
    ) -> bool {
        self.already_visited.contains(direction)
    }
    pub fn visit_facing_dir(
        &mut self,
        direction: &Direction,
    ) {
        self.already_visited.insert(direction.clone());
    }
}
type D = Direction;

fn part_1(input: &str) -> i32 {
    let mut maze = Maze2D::from_str(input).unwrap();
    maze.trace_path() as i32
}
fn part_2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_06::part_1;
    use crate::day_06::part_2;
    const SAMPLE_INPUT: &str = // Wrap
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1_sample_input() {
        assert_eq!(41, part_1(SAMPLE_INPUT));
    }
    #[test]
    fn test_part_1() {
        let input = std::fs::read_to_string("inputs/06.txt").unwrap();
        assert_eq!(4656, part_1(&input));
    }
    #[test]
    fn test_part_2_sample_input() {}
    #[test]
    fn test_part_2() {}
}
