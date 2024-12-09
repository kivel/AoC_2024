use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Grid {
    matrix: Vec<Vec<char>>,
    limits: (usize, usize, usize, usize),
    antinodes: HashSet<(usize, usize)>,
}
impl Grid {
    fn from_lines(lines: &Vec<String>) -> Self {
        let matrix = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let rows = lines.len();
        let cols = lines[0].len();
        let limits = (0, rows, 0, cols);

        Self {
            matrix,
            limits,
            antinodes: HashSet::new(),
        }
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

    // 0,1 and 0,2 (dx = 0, dy = -1) -> 0,0 and 0,3
    // 0,2 and 0,1 (dx = 0, dy = +1) -> 0,0 and 0,3

    // 1,1 and 2,2 (dx = -1, dy = -1) -> 0,0 and 3,3
    // 2,2 and 1,1 (dx = +1, dy = +1) -> 0,0 and 3,3

    // 1,2 and 2,1 (dx = -1, dy = +1) -> 0,3 and 3,0 
    // 2,1 and 1,2 (dx = +1, dy = -1) -> 0,3 and 3,0 
    fn antinodes_from_pair(&mut self, pair: &((usize, usize), (usize, usize))) {
        let (pos1_x, pos1_y) = pair.1;
        let (pos2_x, pos2_y) = pair.0;
        let dx = pos1_x as isize - pos2_x as isize;
        let dy = pos1_y as isize - pos2_y as isize;

        let positive = (dx.is_negative() && dy.is_negative()) || (dx.is_positive() && dy.is_positive());
        
        let mut min_x = min(pos1_x, pos2_x) as isize - dx;
        let mut max_x = max(pos1_x, pos2_x) as isize + dx;
        let mut min_y = min(pos1_y, pos2_y) as isize - dy;
        let mut max_y = max(pos1_y, pos2_y) as isize + dy;

        match positive {
            true => {
                min_x = min(pos1_x, pos2_x) as isize - dx;
                max_x = max(pos1_x, pos2_x) as isize + dx;
                min_y = min(pos1_y, pos2_y) as isize - dy;
                max_y = max(pos1_y, pos2_y) as isize + dy;
            },
            false => {
                min_x = pos1_x as isize + dx;
                max_x = pos2_x as isize - dx;
                min_y = pos1_y as isize + dy;
                max_y = pos2_y as isize - dy;
            },
            // false => {}
            // false => {
            //     min_x = min(pos1_x, pos2_x) as isize + dx;
            //     max_x = max(pos1_x, pos2_x) as isize - dx;
            //     min_y = min(pos1_y, pos2_y) as isize + dy;
            //     max_y = max(pos1_y, pos2_y) as isize - dy;
            // },
        };
        
        let nodes = vec![(min_x, min_y), (max_x, max_y)];
        nodes
        .iter()
        .for_each(|n| {
            Self::check_bound(&self, n).and_then(|n| Some(self.antinodes.insert(n)));
        });

        // match Self::check_bound(&self, &(min_x, min_y)) {
        //     Some(p) => self.antinodes.insert(p),
        //     None => false
        // };
        // match Self::check_bound(&self, &(max_x, max_y)) {
        //     Some(p) => self.antinodes.insert(p),
        //     None => false
        // };
    }
}

fn generate_antenna_pairs(
    antennas: &HashMap<char, HashSet<(usize, usize)>>,
) -> HashMap<char, Vec<((usize, usize), (usize, usize))>> {
    let mut pairs_per_frequency = HashMap::new();

    for (&frequency, positions) in antennas {
        // Collect all pairs of antenna positions
        let pairs: Vec<_> = positions
            .iter()
            .flat_map(|&a| {
                positions
                    .iter()
                    .filter(move |&&b| a < b) // Avoid duplicate pairs and self-pairs
                    .map(move |&b| (a, b))
            })
            .collect();

        pairs_per_frequency.insert(frequency, pairs);
    }

    pairs_per_frequency
}

fn day8_1(data: &Vec<String>) -> usize {
    // let grid = data
    //     .iter()
    //     .map(|line| line.chars().collect::<Vec<char>>())
    //     .collect::<Vec<Vec<char>>>();

    let mut grid = Grid::from_lines(data);

    let mut antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    for (row_idx, row) in grid.matrix.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas.entry(c).or_default().insert((row_idx, col_idx));
                // antennas.insert((row_idx, col_idx));
            }
        }
    }

    let pairs = generate_antenna_pairs(&antennas);
    println!("antennas: {:?}", antennas);
    println!("pairs: {:?}", pairs);

    pairs.iter().for_each(|(freq, pairs)| {
        pairs.iter().for_each(|pair| grid.antinodes_from_pair(pair));
        // grid.antinodes_from_pair(pair)
    });

    for node in grid.antinodes.iter() {
        grid.matrix[node.0][node.1] = '#';
    }

    for line in &grid.matrix {
        println!("{:?}", line.iter().collect::<String>());
    }
    
    println!("grid: {:?}", grid);

    grid.antinodes.len()

    // 0
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day8_test.txt").unwrap();
    let sum = day8_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day8_1};

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day8_test.txt").unwrap();
        let result = day8_1(&d);
        println!("result: {result}");
        assert_eq!(result, 14);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day8.txt").unwrap();
        let result = day8_1(&d);
        println!("result: {result}");
        assert_eq!(result, 5540634308362);
    }
}
