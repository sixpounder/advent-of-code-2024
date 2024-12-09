use nalgebra::{
    iter::{ColumnIter, RowIter},
    DMatrix, Dyn, Matrix, OMatrix, Scalar,
};

type DMatrixiChar = OMatrix<char, Dyn, Dyn>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagonalDirection {
    Left,
    Right,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(usize, usize);

impl From<&(usize, usize)> for Point {
    fn from(value: &(usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Debug, Clone)]
pub struct MatrixSlice<'a, T: Scalar> {
    matrix: &'a OMatrix<T, Dyn, Dyn>,
    start: Point,
    end: Point,
}

impl<'a, T: Scalar> MatrixSlice<'a, T> {
    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn coord_sequence(&self) -> Vec<(Point, &T)> {
        let mut out = vec![];
        let path = points_between(self.start(), self.end());
        for point in path {
            if let Some(item) = self.matrix.get((point.0, point.1)) {
                out.push((point, item));
            } else {
                panic!("Cannot get item at point {} {}", point.0, point.1)
            }
        }

        out
    }

    pub fn diagonal_points(start: Point, end: Point) -> Option<Vec<Point>> {
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

    pub fn cross_slice(&self) -> Option<MatrixSlice<'_, T>> {
        let slice = MatrixSlice::<T>::diagonal_points(self.start(), self.end());
        let binding = slice
            .unwrap()
            .iter()
            .map(|p| (p.0, p.1))
            .collect::<Vec<(usize, usize)>>();
        let slice = binding.as_slice();
        let coords = compute_cross(slice);

        let mut content = vec![];
        for coord in coords.iter() {
            let item = self.matrix.get(*coord).unwrap();
            content.push(item.clone());
        }

        Some(MatrixSlice {
            start: Point::from(coords.first().unwrap()),
            end: Point::from(coords.last().unwrap()),
            matrix: &self.matrix,
        })
    }
}

impl<'a> MatrixSlice<'a, char> {
    pub fn sequence_content(&self) -> String {
        self.coord_sequence()
            .iter()
            .map(|(_point, item)| *item)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct TraversableMatrix<T: Scalar> {
    inner: OMatrix<T, Dyn, Dyn>,
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

        let matrix: OMatrix<char, Dyn, Dyn> =
            DMatrixiChar::from_row_iterator(rows, cols, chars_matrix);

        Self { inner: matrix }
    }
}

impl<T: Scalar> AsRef<Matrix<T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>>>
    for TraversableMatrix<T>
{
    fn as_ref(&self) -> &Matrix<T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        self.inner()
    }
}

#[allow(unused)]
impl<T: Scalar> TraversableMatrix<T> {
    pub fn new_from_iter<I: IntoIterator<Item = T>>(
        rows: usize,
        cols: usize,
        iter: I,
    ) -> TraversableMatrix<T> {
        Self {
            inner: DMatrix::from_iterator(rows, cols, iter),
        }
    }

    pub fn set(&mut self, at: Point, value: T) {
        if let Some(point) = self.inner.get_mut((at.0, at.1)) {
            *point = value;
        }
    }

    pub fn inner(&self) -> &Matrix<T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        &self.inner
    }

    pub fn iter(
        &self,
    ) -> nalgebra::iter::MatrixIter<'_, T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        self.inner.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> nalgebra::iter::MatrixIterMut<'_, T, Dyn, Dyn, nalgebra::VecStorage<T, Dyn, Dyn>> {
        self.inner.iter_mut()
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

    pub fn slice(&self, start: Point, end: Point) -> MatrixSlice<'_, T> {
        MatrixSlice {
            matrix: &self.inner,
            start,
            end,
        }
    }
}

pub struct DiagonalIter<'a, T: Scalar> {
    values: Vec<MatrixSlice<'a, T>>,
    idx: usize,
}

impl<'a, T: Scalar> DiagonalIter<'a, T> {
    fn new(matrix: &'a TraversableMatrix<T>, direction: DiagonalDirection) -> Self {
        let values = get_diagonals(&matrix.inner, direction);

        Self { values, idx: 0 }
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

fn get_diagonals<'a, T: Scalar>(
    matrix: &'a OMatrix<T, Dyn, Dyn>,
    direction: DiagonalDirection,
) -> Vec<MatrixSlice<'a, T>> {
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
            let mut j: usize = start_col;
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
                    start: Point(start_row, ncols - 1),
                    end: Point(i - 1, j + 1),
                    matrix,
                });
            }
        }
    }

    diagonals
}

/// Function to compute the "cross" of a given set of diagonal coordinates
fn compute_cross(coordinates: &[(usize, usize)]) -> Vec<(usize, usize)> {
    // Find the min and max y-coordinates
    let min_y = coordinates.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = coordinates.iter().map(|&(_, y)| y).max().unwrap();

    // Compute the middle y-coordinate (center of the diagonal)
    let mid_y = (min_y + max_y) / 2;

    // Compute the cross coordinates
    coordinates
        .iter()
        .filter_map(|&(x, y)| {
            // Reflect y across mid_y using checked subtraction
            let reflected_y = mid_y.checked_mul(2)?.checked_sub(y);
            reflected_y.map(|new_y| (x, new_y))
        })
        .collect()
}

/// Function to find all points between two coordinates in a matrix
pub fn points_between(start: Point, end: Point) -> Vec<Point> {
    let (x1, y1) = (start.0, start.1);
    let (x2, y2) = (end.0, end.1);

    // Calculate the direction of movement
    let step_x = if x2 > x1 {
        1
    } else if x2 < x1 {
        -1
    } else {
        0
    };
    let step_y = if y2 > y1 {
        1
    } else if y2 < y1 {
        -1
    } else {
        0
    };

    // Calculate the number of steps needed (maximum of row or column distance)
    let steps = ((x2 as isize - x1 as isize).abs()).max((y2 as isize - y1 as isize).abs()) as usize;

    // Collect all points along the line
    let mut points = Vec::new();
    for i in 0..=steps {
        let x = (x1 as isize + i as isize * step_x) as usize;
        let y = (y1 as isize + i as isize * step_y) as usize;
        points.push(Point(x, y));
    }

    points
}

pub fn distance(start: Point, end: Point) -> usize {
    points_between(start, end).len() - 1
}
