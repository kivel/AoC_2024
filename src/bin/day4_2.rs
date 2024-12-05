use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// Find all occurances of `XMAS` in a matirx of letters.
// The direction of the word can be forwards or backwards, top to bottom and diagonal.
// Thus only 8 possible ways the word can be formed from the "X", those indeces can be determined deterministically.

// Approach:
// 1. Find all occurances of "X" with the respective coordinates.
// 2. Collect the letters from the coordinates belonging to the 8 options and check if they match.
// 3. Count the number of matches.

fn find_a(grid: &Vec<Vec<char>>) -> Option<HashSet<(usize, usize)>> {
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();

    // Find all occurrences of 'X'
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == 'A' {
                coordinates.insert((row_idx, col_idx));
            }
        }
    }

    // Return result
    match coordinates.is_empty() {
        true => None,
        false => Some(coordinates),
    }
}

fn get_one_step_diagonals(
    grid: &&[Vec<char>],
    start: (usize, usize),
) -> Option<Vec<Vec<(usize, usize)>>> {
    let (row, col) = start;
    let max_row = grid.len();
    let max_col = grid[0].len();

    // Check if the start point is valid
    if row == 0 || col == 0 || row >= max_row - 1 || col >= max_col - 1 {
        return None; // Invalid starting point
    }

    // Initialize vectors
    let mut nw_se: Vec<(usize, usize)> = Vec::new();
    let mut ne_sw: Vec<(usize, usize)> = Vec::new();

    // NW: (-1, -1)
    nw_se.push((row - 1, col - 1));

    // start point
    nw_se.push((row, col));

    // SE: (+1, +1)
    nw_se.push((row + 1, col + 1));

    // NE: (-1, +1)
    ne_sw.push((row - 1, col + 1));

    // start point
    ne_sw.push((row, col));

    // SW: (+1, -1)
    ne_sw.push((row + 1, col - 1));

    Some(vec![nw_se, ne_sw])
}

fn get_strings_in_directions(grid: &[Vec<char>], start: (usize, usize)) -> Option<Vec<String>> {
    let diags = match get_one_step_diagonals(&grid, start) {
        Some(diags) => diags,
        None => return None,
    };

    let res = diags
        .iter()
        .map(|diag| {
            // nw_se and ne_sw
            diag.iter()
                .map(|d| {
                    // coordinates along the diagonal
                    grid[d.0][d.1] // the character
                })
                .collect()
        })
        .collect();
    println!("start: {start:?} -> result: {:?}", res);
    Some(res)
}

fn get_strings_in_directions1(
    grid: &[Vec<char>],
    start: (usize, usize),
    max_distance: usize,
) -> Vec<String> {
    let directions = vec![
        ("NW", (-1, -1)), // Northwest
        ("NE", (-1, 1)),  // Northeast
        ("SW", (1, -1)),  // Southwest
        ("SE", (1, 1)),   // Southeast
    ];

    let mut results = Vec::new();

    let mut chars_in_direction = Vec::new();
    for (_, (dx, dy)) in directions {
        for distance in 1..=max_distance {
            // Compute new coordinates
            let new_x = start.0 as isize + dx * distance as isize;
            let new_y = start.1 as isize + dy * distance as isize;

            // Check bounds
            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[new_x as usize].len() as isize
            {
                chars_in_direction.push(grid[new_x as usize][new_y as usize]);
            }
        }
    }
    chars_in_direction.sort();
    results.push(chars_in_direction.into_iter().collect());
    println!("result: {:?}", results);
    results
}

fn day4_2(data: &Vec<String>) -> usize {
    let grid = data
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut coordinates = find_a(&grid).expect("The letter 'A' could not be found!");
    println!("coordinates: {coordinates:?}");

    coordinates
        .drain()
        .filter(|&c| get_strings_in_directions(&grid, c).is_some())
        .map(|c| {
            match get_strings_in_directions(&grid, c)
                .unwrap()
                .iter()
                .all(|s| s == "MAS" || s == "SAM")
            {
                true => 1,
                false => 0,
            }
        })
        .sum()
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day4_2_test.txt").unwrap();
    let sum = day4_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day4_2};

    #[test]
    fn small_matrix() {
        let d: Vec<String> = vec!["X.X".into(), "ABC".into(), "DXF".into(), "GHI".into()];

        let result = day4_2(&d);
        println!("result: {result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn xmas() {
        let d: Vec<String> = vec!["XMAS".into(), "MMAA".into(), "AMAM".into(), "SAMX".into()];

        let result = day4_2(&d);
        println!("result: {result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day4_2_test.txt").unwrap();
        let result = day4_2(&d);
        println!("result: {result}");
        assert_eq!(result, 9);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day4_1.txt").unwrap();
        let result = day4_2(&d);
        println!("result: {result}");
        assert_eq!(result, 1886);
    }
}
