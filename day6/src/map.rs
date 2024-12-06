use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

impl Map {
    fn coords_to_index(&self, coords: (usize, usize)) -> usize {
        (coords.0 * self.width) + coords.1
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    pub fn get(&self, coords: (usize, usize)) -> Option<&Cell> {
        self.cells.get(self.coords_to_index(coords))
    }

    pub fn set(&mut self, index: usize, value: Cell) {
        if let Some(cell) = self.cells.get_mut(index) {
            *cell = value;
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cell> {
        self.cells.iter()
    }

    pub fn into_next(&self) -> Option<Map> {
        let mut next_state = self.clone();

        let maybe_guard = self.iter().enumerate().find(|(_idx, cell)| cell.is_guard());
        if let Some((index, guard)) = maybe_guard {
            let guard = guard.as_guard().expect("Expected Cell::Guard");
            let (row, column) = self.index_to_coords(index);

            let next_guard_coords = match guard.direction {
                Direction::Up => {
                    if row > 0 {
                        Some((row - 1, column))
                    } else {
                        None
                    }
                },
                Direction::Right => {
                    if column < self.width - 1 {
                        Some((row, column + 1))
                    } else {
                        None
                    }
                },
                Direction::Down => {
                    if row < self.height - 1 {
                        Some((row + 1, column))
                    } else {
                        None
                    }
                },
                Direction::Left => {
                    if column > 0 {
                        Some((row, column - 1))
                    } else {
                        None
                    }
                },
            };

            if let Some(next_guard_cell_coords) = next_guard_coords {
                let next_guard_cell = self.get(next_guard_cell_coords).unwrap();
                if *next_guard_cell == Cell::Obstable {
                    next_state.set(index, Cell::Guard(guard.rotate_right()))
                } else {
                    next_state.set(index, Cell::Visited);
                    next_state.set(
                        self.coords_to_index(next_guard_cell_coords),
                        Cell::Guard(Guard {
                            direction: guard.direction.clone(),
                        }),
                    )
                }
            } else {
                // The guard will go outside the map
                next_state.set(index, Cell::Visited);
            }
            Some(next_state)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn pretty_print(&self) {
        for i in 0..self.width {
            if i != 0 {
                println!();
            }
            for j in 0..self.height {
                print!("{} ", self.get((i, j)).unwrap())
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

        for line in raw.lines() {
            for char in line.chars() {
                cells.push(Cell::from(char));
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Obstable,
    Guard(Guard),
    Free,
    Visited,
}

impl Cell {
    pub fn is_guard(&self) -> bool {
        match self {
            Cell::Guard(_) => true,
            _ => false,
        }
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
                Cell::Obstable => "#",
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
            '#' => Cell::Obstable,
            '.' => Cell::Free,
            'X' => Cell::Visited,
            direction => Cell::Guard(Guard {
                direction: direction.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Guard {
    direction: Direction,
}

impl Guard {
    fn rotate_right(&self) -> Guard {
        Guard {
            direction: self.direction.rotate_right(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
