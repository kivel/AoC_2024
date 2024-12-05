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

fn find_x(grid: &Vec<Vec<char>>) -> Option<HashSet<(usize, usize)>> {
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();

    // Find all occurrences of 'X'
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c == 'X' {
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

fn get_strings_in_directions(
    grid: &[Vec<char>],
    start: (usize, usize),
    max_distance: usize,
) -> Vec<String> {
    let directions = vec![
        ("N", (-1, 0)),   // North
        ("S", (1, 0)),    // South
        ("W", (0, -1)),   // West
        ("E", (0, 1)),    // East
        ("NW", (-1, -1)), // Northwest
        ("NE", (-1, 1)),  // Northeast
        ("SW", (1, -1)),  // Southwest
        ("SE", (1, 1)),   // Southeast
    ];

    let mut results = Vec::new();

    for (_, (dx, dy)) in directions {
        let mut chars_in_direction = vec!['X']; // start with a 'X' character
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
        results.push(chars_in_direction.into_iter().collect());
    }

    results
}

fn day4_1(data: &Vec<String>) -> usize {
    let grid = data
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut coordinates = find_x(&grid).expect("The letter 'X' could not be found!");
    println!("coordinates: {coordinates:?}");

    coordinates
        .drain()
        .map(|c| {
            get_strings_in_directions(&grid, c, 3)
                .iter()
                .filter(|&s| s == "XMAS")
                .count()
        })
        .sum()
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day4_1_test.txt").unwrap();
    let sum = day4_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day4_1};

    #[test]
    fn small_matrix() {
        let d: Vec<String> = vec!["X.X".into(), "ABC".into(), "DXF".into(), "GHI".into()];

        let result = day4_1(&d);
        println!("result: {result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn xmas() {
        let d: Vec<String> = vec!["XMAS".into(), "MMAA".into(), "AMAM".into(), "SAMX".into()];

        let result = day4_1(&d);
        println!("result: {result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day4_1_test.txt").unwrap();
        let result = day4_1(&d);
        println!("result: {result}");
        assert_eq!(result, 18);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day4_1.txt").unwrap();
        let result = day4_1(&d);
        println!("result: {result}");
        assert_eq!(result, 2545);
    }
}
