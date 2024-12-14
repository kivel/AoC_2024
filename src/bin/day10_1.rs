use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug)]
#[allow(dead_code)]
struct Trailhead {
    start_position: (usize, usize),
    steps: HashMap<usize, HashSet<(usize, usize)>>,
}

impl Trailhead {
    fn from_position(start_position: (usize, usize)) -> Self {
        Self {
            start_position,
            steps: HashMap::from([(0, HashSet::from([start_position]))]),
        }
    }

    fn find_next_step_positions(
        &mut self,
        grid: &Grid,
        position: &(usize, usize),
    ) -> Vec<(usize, usize)> {
        let coordinates: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        let height = grid.matrix[position.0][position.1];
        coordinates
            .iter()
            .map(|offset| {
                (
                    position.0 as isize + offset.0,
                    position.1 as isize + offset.1,
                )
            })
            .filter(|position| grid.check_bound(position).is_some())
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|&(x, y)| grid.matrix[x][y] == height + 1)
            .collect()
    }

    fn add_step(&mut self, step: usize, position: (usize, usize)) {
        self.steps
            .entry(step)
            .and_modify(|v| {
                v.insert(position);
            })
            .or_insert(HashSet::from([position]));
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Grid {
    matrix: Vec<Vec<usize>>,
    limits: (usize, usize, usize, usize),
}
impl Grid {
    fn from_lines(lines: &Vec<String>) -> Self {
        let matrix = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let rows = lines.len();
        let cols = lines[0].len();
        let limits = (0, rows, 0, cols);

        Self { matrix, limits }
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

fn day10_1(data: &Vec<String>) -> usize {
    let grid = Grid::from_lines(data);
    let mut trails: Vec<Trailhead> = vec![];
    for (row_idx, row) in grid.matrix.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == 0 {
                trails.push(Trailhead::from_position((row_idx, col_idx)));
            }
        }
    }

    for trail in &mut trails {
        let mut height: usize = 0;
        while height <= 9 {
            println!("height: {}", height);
            let steps = match trail.steps.get(&height) {
                Some(steps) => steps.clone(),
                None => continue,
            };
            println!("steps: {:?}", &steps);
            height += 1;
            steps.iter().for_each(|step| {
                let next_steps = trail.find_next_step_positions(&grid, step);
                for step in next_steps {
                    trail.add_step(height, step);
                }
            });
        }
    }

    trails
        .iter()
        .map(|trail| {
            let n = match trail.steps.get(&9) {
                Some(steps) => steps.len(),
                None => 0,
            };
            println!("number of accessible trails = {n}");
            n
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .sum()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();
    let sum = day10_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day10_1};

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();
        let result = day10_1(&d);
        println!("result: {result}");
        assert_eq!(result, 36);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day10.txt").unwrap();
        let result = day10_1(&d);
        println!("result: {result}");
        assert_eq!(result, 413);
    }
}
