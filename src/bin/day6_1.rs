use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Direction {
    N,
    S,
    E,
    W,
}
impl Direction {
    /// Turns right (clockwise).
    fn turn_right(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Grid {
    limits: (usize, usize, usize, usize),
    obstacles: HashSet<(usize, usize)>,
}
impl Grid {
    fn from_lines(lines: &Vec<String>) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();
        let limits = (0, rows, 0, cols);

        let mut obstacles = HashSet::new();

        // lines (String) -> Vec<char>
        let grid = advent_of_code::lines_to_matrix(lines);
        // Find all occurrences of '#'
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '#' {
                    obstacles.insert((row_idx, col_idx));
                }
            }
        }
        Self { limits, obstacles }
    }

    fn is_obstacle_at(&self, pos: &(usize, usize)) -> bool {
        self.obstacles.contains(&pos)
    }

    fn is_within_bound(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= self.limits.0 as isize
            && pos.0 < self.limits.1 as isize
            && pos.1 >= self.limits.2 as isize
            && pos.1 < self.limits.3 as isize
    }

    fn check_bound(&self, pos: &(isize, isize)) -> Option<(usize, usize)> {
        match Self::is_within_bound(&self, &pos) {
            true => Some((pos.0 as usize, pos.1 as usize)),
            false => None,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Guard {
    pos: (usize, usize),
    visited_posistions: HashSet<(usize, usize)>,
    direction: Direction,
}

impl Guard {
    fn from_lines(lines: &Vec<String>) -> Self {
        let grid = advent_of_code::lines_to_matrix(lines);
        let pos = Guard::find_position(&grid).expect("Invalid position.");
        let mut visited_posistions = HashSet::new();
        visited_posistions.insert(pos);
        let direction = Guard::find_initial_direction(&grid).expect("Invalid initial direction.");
        Self {
            pos,
            visited_posistions,
            direction,
        }
    }

    fn find_position(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if c == '^' || c == 'v' || c == '>' || c == '<' {
                    return Some((row_idx, col_idx));
                }
            }
        }
        None
    }

    fn find_initial_direction(grid: &Vec<Vec<char>>) -> Option<Direction> {
        for (_, row) in grid.iter().enumerate() {
            for (_, &c) in row.iter().enumerate() {
                match c {
                    '^' => return Some(Direction::N),
                    'v' => return Some(Direction::S),
                    '>' => return Some(Direction::E),
                    '<' => return Some(Direction::W),
                    _ => continue, // Skip non-direction characters.
                }
            }
        }
        None
    }

    fn vector_from_dir(dir: Direction) -> (isize, isize) {
        match dir {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1),
        }
    }

    fn take_step(&mut self, grid: &Grid) -> Option<(usize, usize)> {
        let (dx, dy) = Guard::vector_from_dir(self.direction);
        let new_pos = ((self.pos.0 as isize + dx), (self.pos.1 as isize + dy));

        grid.check_bound(&new_pos).and_then(|valid_pos| {
            if grid.is_obstacle_at(&valid_pos) {
                self.direction = self.direction.turn_right();
                Some(self.pos) // Stay in the same position if there's an obstacle
            } else {
                self.pos = valid_pos;
                self.visited_posistions.insert(self.pos);
                Some(self.pos)
            }
        })
    }
}

fn day6_1(data: &Vec<String>) -> usize {
    let grid = Grid::from_lines(data);
    let mut guard = Guard::from_lines(data);
    println!("{:?}", grid);
    println!("{:?}", guard);
    while guard.take_step(&grid).is_some() {
        // run until we leave the grid
    }
    println!("{:?}", guard);

    guard.visited_posistions.len()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day6_1_test.txt").unwrap();
    let sum = day6_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day6_1};

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day6_1_test.txt").unwrap();
        let result = day6_1(&d);
        println!("result: {result}");
        assert_eq!(result, 41);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day6_1.txt").unwrap();
        let result = day6_1(&d);
        println!("result: {result}");
        assert_eq!(result, 4647);
    }
}
