use std::io::Chain;
use nalgebra::{iter::{ColumnIter, RowIter}, Dyn, Matrix, OMatrix, Scalar};

type DMatrixiChar = OMatrix<char, Dyn, Dyn>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagonalDirection {
    Left,
    Right,
    Both
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(usize, usize);

impl From<&(usize, usize)> for Point {
    fn from(value: &(usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Clone)]
pub struct MatrixSlice<'a, T: Scalar> {
    matrix: &'a OMatrix<T, Dyn, Dyn>,
    start: Point,
    end: Point,
    sequence: Vec<T>,
}

impl<'a, T: Scalar> MatrixSlice<'a, T> {
    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn sequence(&self) -> &Vec<T> {
        &self.sequence
    }

    pub fn points_between_diagonal(start: Point, end: Point) -> Option<Vec<Point>> {
        if end.0.abs_diff(start.0) != end.1.abs_diff(start.1) {
            None
        } else {
            let step_x: isize = (end.0 as isize - start.0 as isize).signum(); // -1, 0, or 1
            let step_y: isize = (end.1 as isize - start.1 as isize).signum(); // -1, 0, or 1

            let mut coordinates = Vec::new();
            let mut current = Point(start.0, start.1);

            while current != end {
                coordinates.push(current);
                if (step_x) >= 0 {
                    current.0 += step_x as usize;
                } else {
                    current.0 -= step_x as usize;
                }

                if (step_y) >= 0 {
                    current.1 += step_y as usize;
                } else {
                    current.1 -= step_y as usize;
                }
            }

            coordinates.push(end);

            Some(coordinates)
        }
    }
}

impl<'a> MatrixSlice<'a, char> {
    pub fn sequence_content(&self) -> String {
        self.sequence.iter().collect()
    }
}

pub struct TraversableMatrix<T: Scalar> {
    inner: OMatrix<T, Dyn, Dyn>
}

impl<S: AsRef<str>> From<S> for TraversableMatrix<char> {
    fn from(value: S) -> Self {
        let mut rows: usize = 0;
        let cols = value.as_ref().lines().nth(0).unwrap_or("").len();

        let chars_matrix = value.as_ref().chars().fold(vec![], |mut acc, ch| {
            if ch != '\n' {
                acc.push(ch);
            } else {
                rows += 1;
            }
            acc
        });
        rows += 1;

        let matrix: OMatrix<char, Dyn, Dyn> = DMatrixiChar::from_row_iterator(rows, cols, chars_matrix);
        
        Self {
            inner: matrix
        }
    }
}

impl<T: Scalar> AsRef<Matrix<T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>>> for TraversableMatrix<T> {
    fn as_ref(&self) -> &Matrix<T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        self.inner()
    }
}

impl<T: Scalar> TraversableMatrix<T> {
    pub fn inner(&self) -> &Matrix<T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        &self.inner
    }
    
    pub fn row_iter(&self) -> RowIter<'_, T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        self.inner.row_iter()
    }

    pub fn column_iter(&self) -> ColumnIter<'_, T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        self.inner.column_iter()
    }

    pub fn left_diagonal_iter(&self) -> DiagonalIter<T> {
        DiagonalIter::new(self, DiagonalDirection::Left)
    }

    pub fn right_diagonal_iter(&self) -> DiagonalIter<T> {
        DiagonalIter::new(self, DiagonalDirection::Right)
    }

    pub fn diagonal_iter(&self) -> DiagonalIter<T> {
        DiagonalIter::new(self, DiagonalDirection::Both)
    }
}

pub struct DiagonalIter<'a, T: Scalar> {
    direction: DiagonalDirection,
    matrix: &'a TraversableMatrix<T>,
    values: Vec<MatrixSlice<'a, T>>,
    idx: usize
}

impl<'a, T: Scalar> DiagonalIter<'a, T> {
    fn new(matrix: &'a TraversableMatrix<T>, direction: DiagonalDirection) -> Self {
        let values = get_diagonals(&matrix.inner, direction);

        Self {
            direction,
            matrix,
            values,
            idx: 0
        }
    }
}

impl<'a, T: Scalar> Iterator for DiagonalIter<'a, T> {
    type Item = MatrixSlice<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.values.get(self.idx).cloned();
        self.idx += 1;
        ret
    }
}

fn get_diagonals<'a, T: Scalar>(matrix: &'a OMatrix<T, Dyn, Dyn>, direction: DiagonalDirection) -> Vec<MatrixSlice<'a, T>> {
    let nrows = matrix.nrows();
    let ncols = matrix.ncols();
    let mut diagonals: Vec<MatrixSlice<'a, T>> = Vec::new();

    if direction == DiagonalDirection::Left || direction == DiagonalDirection::Both {
        // Diagonals from bottom-left to top-right
        for start_row in (0..nrows).rev() {
            let mut i = start_row;
            let mut j = 0;
            let mut diagonal = Vec::new();
    
            while i < nrows && j < ncols {
                diagonal.push(matrix[(i, j)].clone());
                i += 1;
                j += 1;
            }
    
            diagonals.push(MatrixSlice {
                sequence: diagonal,
                start: Point(start_row, 0),
                end: Point(i - 1, j - 1),
                matrix,
            });
        }
    
        for start_col in 1..ncols {
            let mut i = 0;
            let mut j = start_col;
            let mut diagonal = Vec::new();
    
            while i < nrows && j < ncols {
                diagonal.push(matrix[(i, j)].clone());
                i += 1;
                j += 1;
            }
    
            diagonals.push(MatrixSlice {
                sequence: diagonal,
                start: Point(0, start_col),
                end: Point(i - 1, j - 1),
                matrix,
            });
        }
    }

    if direction == DiagonalDirection::Right || direction == DiagonalDirection::Both {
        // Diagonals from top-left to bottom-right
        for start_col in 0..ncols {
            let mut i = 0;
            let mut j = start_col;
            let mut diagonal = Vec::new();
    
            while i < nrows {
                diagonal.push(matrix[(i, j)].clone());
                i += 1;
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
    
            diagonals.push(MatrixSlice {
                sequence: diagonal,
                start: Point(0, start_col),
                end: Point(i - 1, j + 1),
                matrix,
            });
        }
    
        for start_row in 1..nrows {
            let mut i = start_row;
            let mut j = ncols - 1;
            let mut diagonal = Vec::new();
    
            while i < nrows {
                diagonal.push(matrix[(i, j)].clone());
                i += 1;
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            // Skip the central diagonal if it's already added
            if diagonal.len() != nrows && direction == DiagonalDirection::Both {
                diagonals.push(MatrixSlice {
                    sequence: diagonal,
                    start: Point(start_row, ncols - 1),
                    end: Point(i - 1, j + 1),
                    matrix,
                });
            }
        }
    }

    diagonals
}