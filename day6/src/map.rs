use rayon::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    guard_idx: Option<usize>,
}

impl Map {
    pub fn coords_to_index(&self, coords: (usize, usize)) -> usize {
        (coords.0 * self.width) + coords.1
    }

    pub fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get_coords(&self, coords: (usize, usize)) -> Option<&Cell> {
        self.cells.get(self.coords_to_index(coords))
    }

    pub fn get_index(&self, index: usize) -> Option<&Cell> {
        self.cells.get(index)
    }

    pub fn set(&mut self, index: usize, value: Cell) {
        if let Some(cell) = self.cells.get_mut(index) {
            if value.is_guard() {
                self.guard_idx = Some(index);
            }
            *cell = value;
        }
    }

    pub fn iter(&self) -> rayon::slice::Iter<'_, Cell> {
        self.cells.par_iter()
    }

    pub fn get_guard(&self) -> Option<(usize, &Cell)> {
        self.guard_idx
            .map(|idx| (idx, self.cells.get(idx).unwrap()))
    }

    pub fn next(&mut self) -> bool {
        let maybe_guard = self
            .guard_idx
            .map(|idx| (idx, self.cells.get(idx).unwrap()));
        if let Some((index, guard)) = maybe_guard {
            let guard = guard.as_guard().expect("Expected Cell::Guard");
            let guard_direction = guard.direction().clone();
            let (row, column) = self.index_to_coords(index);

            let next_guard_coords = match guard.direction {
                Direction::Up => {
                    if row > 0 {
                        Some((row - 1, column))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if column < self.width - 1 {
                        Some((row, column + 1))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if row < self.height - 1 {
                        Some((row + 1, column))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if column > 0 {
                        Some((row, column - 1))
                    } else {
                        None
                    }
                }
            };

            if let Some(next_guard_cell_coords) = next_guard_coords {
                let next_guard_cell = self.get_coords(next_guard_cell_coords).unwrap();
                if *next_guard_cell == Cell::Obstacle {
                    self.set(index, Cell::Guard(guard.rotate_right()))
                } else {
                    self.set(index, Cell::Visited);
                    self.set(
                        self.coords_to_index(next_guard_cell_coords),
                        Cell::Guard(Guard {
                            direction: guard_direction,
                        }),
                    )
                }
            } else {
                // The guard will go outside the map
                self.set(index, Cell::Visited);
                self.guard_idx = None;
            }
            true
        } else {
            false
        }
    }

    #[allow(unused)]
    pub fn pretty_print(&self) {
        for i in 0..self.width {
            println!();
            for j in 0..self.height {
                print!("{} ", self.get_coords((i, j)).unwrap())
            }
        }
        println!()
    }
}

impl From<&str> for Map {
    fn from(raw: &str) -> Self {
        let width = raw.lines().nth(0).expect("No first line in input").len();
        let height = raw.lines().count();
        let mut cells = vec![];
        let mut guard_idx = None;
        let mut i = 0;
        for line in raw.lines() {
            for char in line.chars() {
                let cell = Cell::from(char);
                if cell.is_guard() {
                    guard_idx = Some(i);
                }
                cells.push(cell);
                i += 1;
            }
        }

        Self {
            width,
            height,
            cells,
            guard_idx,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Obstacle,
    Guard(Guard),
    Free,
    Visited,
}

impl Cell {
    pub fn is_guard(&self) -> bool {
        matches!(self, Cell::Guard(_))
    }

    pub fn as_guard(&self) -> Option<&Guard> {
        match self {
            Cell::Guard(guard) => Some(guard),
            _ => None,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Obstacle => "#",
                Cell::Guard(guard) => guard.direction.symbol(),
                Cell::Free => ".",
                Cell::Visited => "X",
            }
        )
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Obstacle,
            '.' => Cell::Free,
            'X' => Cell::Visited,
            direction => Cell::Guard(Guard {
                direction: direction.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Guard {
    direction: Direction,
}

#[allow(unused)]
impl Guard {
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn rotate_right(&self) -> Guard {
        Guard {
            direction: self.direction.rotate_right(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn symbol(&self) -> &str {
        match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "⌄",
            Direction::Left => "<",
        }
    }

    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '⌄' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unexpected direction character"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
