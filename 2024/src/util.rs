#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    S,
    #[default]
    E,
    W,
    NE,
    SE,
    NW,
    SW,
}

type D = Direction;
impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match self {
            D::N => D::S,
            D::S => D::N,
            D::E => D::W,
            D::W => D::E,
            D::NE => D::SW,
            D::SE => D::NW,
            D::NW => D::SE,
            D::SW => D::NE,
        }
    }
    pub fn get_coords(&self) -> (isize, isize) {
        match self {
            D::N => (0, 1),
            D::S => (0, -1),
            D::E => (1, 0),
            D::W => (-1, 0),
            D::NE => (1, 1),
            D::SE => (1, -1),
            D::NW => (-1, 1),
            D::SW => (-1, -1),
        }
    }
}
