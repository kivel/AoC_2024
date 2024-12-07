use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub mod aoc {}
#[allow(dead_code)]
pub struct Reader {}
#[allow(dead_code)]
impl Reader {
    // Returns an Iterator to the Reader of the lines of the file.
    pub fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(std::io::BufReader::new(file).lines())
    }

    pub fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
        let org_data = match Reader::read_lines(filename) {
            Ok(lines) => lines.collect::<Result<_, _>>().unwrap(),
            Err(e) => panic!("Error opening file {}: {}", filename, e),
        };
        Ok(org_data)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LimitedVecDeque<T> {
    deque: VecDeque<T>,
    capacity: usize,
}

#[allow(dead_code)]
impl<T> LimitedVecDeque<T> {
    /// Creates a new `LimitedVecDeque` with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Adds a value to the deque, maintaining the maximum capacity.
    pub fn push(&mut self, value: T) {
        if self.deque.len() == self.capacity {
            self.deque.pop_front(); // Remove the oldest value
        }
        self.deque.push_back(value);
    }

    /// Returns an immutable reference to the underlying `VecDeque`.
    pub fn as_deque(&self) -> &VecDeque<T> {
        &self.deque
    }

    pub fn len(&self) -> usize {
        self.deque.len()
    }
}

#[allow(dead_code)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    current_index: usize,
}

#[allow(dead_code)]
impl<T: Clone> RingBuffer<T> {
    pub fn new(items: Vec<T>) -> Self {
        RingBuffer {
            buffer: items,
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> T {
        let item = self.buffer[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.buffer.len();
        item
    }
}

#[allow(dead_code)]
pub fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut transposed_matrix: Vec<Vec<char>> = vec![vec!['.'; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }

    transposed_matrix
}

#[allow(dead_code)]
pub fn lines_to_matrix(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[allow(dead_code)]
pub fn find_char(grid: Vec<Vec<char>>, char: char) -> (usize, usize) {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == char {
                return (row_idx, col_idx);
            }
        }
    }
    panic!("The character '{}' was not found in the grid.", char)
}
