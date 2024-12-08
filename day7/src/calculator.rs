use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone)]
pub enum Operation {
    Sum,
    Mul,
    Concat,
}

type Number = u64;

impl Operation {
    pub fn apply(&self, n: Number, m: Number) -> Number {
        match self {
            Operation::Sum => n + m,
            Operation::Mul => n * m,
            Operation::Concat => format!("{n}{m}")
                .parse::<Number>()
                .expect("Invalid operation result"),
        }
    }

    pub fn neutral_operator(&self) -> Number {
        match self {
            Operation::Sum => 0,
            Operation::Mul => 1,
            Operation::Concat => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CalibrationEquation {
    operands: Vec<Number>,
    result: Number,
}

impl CalibrationEquation {
    pub fn result(&self) -> Number {
        self.result
    }

    pub fn try_solve(&self, supported_operations: &[Operation]) -> Option<Vec<Operation>> {
        let operation_permutations = Permutations::new(supported_operations, self.operands.len());
        operation_permutations
            .par_bridge()
            .find_first(|p| self.is_solved_by(p))
    }

    fn is_solved_by(&self, operation_sequence: &[Operation]) -> bool {
        if self.operands.is_empty() && self.result != 0 {
            false
        } else {
            let mut i = 0;
            let mut acc = operation_sequence.first().unwrap().neutral_operator();
            for operand in self.operands.iter() {
                let operation = operation_sequence.get(i).unwrap();
                acc = operation.apply(acc, *operand);
                i += 1;
            }

            self.result == acc
        }
    }
}

impl<T: AsRef<str>> From<T> for CalibrationEquation {
    fn from(value: T) -> Self {
        let mut chunks = value.as_ref().split(":");
        let result = chunks.next().expect("No result chunk found");
        let operands = chunks.next().expect("No result chunk found").trim();

        Self {
            result: result.parse().expect("Invalid result"),
            operands: operands
                .split(" ")
                .map(|v| v.parse().expect("Invalid operand"))
                .collect::<Vec<Number>>(),
        }
    }
}

/// A struct representing a lazy permutation generator
pub struct Permutations<T> {
    symbols: Vec<T>,
    length: usize,
    current: Vec<usize>,
    finished: bool,
}

impl<T: Clone> Permutations<T> {
    /// Create a new `Permutations` iterator
    pub fn new(symbols: &[T], length: usize) -> Self {
        Self {
            symbols: Vec::from(symbols),
            length,
            current: vec![0; length],
            finished: length == 0,
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // Generate the current permutation based on the indices in `self.current`
        let permutation = self
            .current
            .iter()
            .map(|&i| self.symbols[i].clone())
            .collect();

        // Update the indices to generate the next permutation
        for i in (0..self.length).rev() {
            if self.current[i] + 1 < self.symbols.len() {
                self.current[i] += 1;
                return Some(permutation);
            } else {
                self.current[i] = 0;
            }
        }

        // If we reach here, all permutations are generated
        self.finished = true;
        Some(permutation)
    }
}
