pub fn run_day_4() {
    let input = std::fs::read_to_string("inputs/04.txt").unwrap();
    println!("Day 4 Part 1: {}", part_1(&input));
    println!("Day 4 Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut x: isize = 0;
    let mut sum = 0;
    while x < arr.len() as isize {
        let mut y = 0;
        while y < arr[x as usize].len() as isize {
            sum += check(&arr, x, y, Direction::North, "XMAS");
            sum += check(&arr, x, y, Direction::South, "XMAS");
            sum += check(&arr, x, y, Direction::East, "XMAS");
            sum += check(&arr, x, y, Direction::West, "XMAS");
            sum += check(&arr, x, y, Direction::Northwest, "XMAS");
            sum += check(&arr, x, y, Direction::Northeast, "XMAS");
            sum += check(&arr, x, y, Direction::Southwest, "XMAS");
            sum += check(&arr, x, y, Direction::Southeast, "XMAS");
            y += 1;
        }
        x += 1;
    }

    sum as i32
}

enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Southeast,
    Northwest,
    Southwest,
}

impl Direction {
    pub fn get_coords(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::Northeast => (1, 1),
            Direction::Southeast => (1, -1),
            Direction::Northwest => (-1, 1),
            Direction::Southwest => (-1, -1),
        }
    }
}

fn check(
    arr: &Vec<Vec<char>>,
    x: isize,
    y: isize,
    direction: Direction,
    remaining: &str,
) -> usize {
    // println!("Checking ({x}, {y}) for {}", remaining.chars().nth(0).unwrap());
    let mut paths = 0;
    if x < 0 || y < 0 {
        // println!("Out of bounds -neg");
        return 0;
    }

    if (x as usize) < arr.len() && (y as usize) < arr[x as usize].len() {
        // println!("\tFOUND ({x}, {y})  ===== {}", arr[x as usize][y as usize]);
    }
    if (x as usize) < arr.len()
        && (y as usize) < arr[x as usize].len()
        && Some(arr[x as usize][y as usize]) == remaining.chars().nth(0)
    {
        if remaining.len() == 1 {
            // println!("Found XMAS");
            return 1;
        }
        let (xmod, ymod) = direction.get_coords();
        paths += check(arr, x + xmod, y + ymod, direction, &remaining[1..]);
    }
    paths
}

fn part_2(input: &str) -> i32 {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut x = 0;
    let mut sum = 0;
    while x < arr.len() as isize {
        let mut y = 0;
        while y < arr[x as usize].len() as isize {
            if arr[x as usize][y as usize] == 'A' {
                macro_rules! check_dir {
                    ($dir:path) => {{
                        let (xmut, ymut) = $dir.get_coords();
                        check(&arr, x - xmut, y - ymut, $dir, "MAS") > 0
                    }};
                }
                type D = Direction;
                if (check_dir!(D::Northeast) || check_dir!(D::Southwest))
                    && (check_dir!(D::Northwest) || check_dir!(D::Southeast))
                {
                    sum += 1;
                }
            }
            y += 1;
        }
        x += 1;
    }

    sum as i32
}

#[cfg(test)]
mod tests {
    use crate::day_04::{part_1, part_2};

    #[test]
    fn test_part_1_sample_input() {
        let input = // Wrap to newline
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, part_1(input));
    }
    #[test]
    fn test_part_1() {
        let input = std::fs::read_to_string("inputs/04.txt").unwrap();
        assert_eq!(2530, part_1(&input));
    }
    #[test]
    fn test_part_2_sample_input() {
        let input = // Wrap to newline
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, part_2(input));
    }
    #[test]
    fn test_part_2() {
        let input = std::fs::read_to_string("inputs/04.txt").unwrap();
        assert_eq!(1921, part_2(&input));
    }
}
