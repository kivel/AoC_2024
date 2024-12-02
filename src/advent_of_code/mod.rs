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
