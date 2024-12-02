#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use std::cmp::max;
use std::cmp::min;

// There is no build in `abs` for u32 in Rust, who would have guessed that.
fn absolute_difference_u32(a: &u32, b: &u32) -> u32 {
    max(a, b) - min(a, b)
}
fn is_valid(vec: &Vec<u32>) -> bool {
    // fn is_valid<T: PartialOrd>(vec: &[T]) -> bool {
    let monotonic_inc = vec.windows(2).all(|w| (w[0] < w[1]));
    let monotonic_dec = vec.windows(2).all(|w| (w[0] > w[1]));
    let step_size = vec
        .windows(2)
        .all(|w| absolute_difference_u32(&w[0], &w[1]) < 4);
    (monotonic_inc || monotonic_dec) && step_size
}

#[derive(Debug)]
struct Level {
    steps: Vec<u32>,
}

impl Level {
    fn new(steps: &String) -> Option<Level> {
        let steps: Vec<u32> = steps
            .split_whitespace()
            .into_iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        match is_valid(&steps) {
            true => Some(Level { steps }),
            false => None,
        }
    }
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day2_1(data: &Vec<String>) -> u32 {
    data.iter()
        .map(|l| match Level::new(l) {
            Some(level) => 1,
            None => 0,
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
    let sum = day2_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day2_1};

    #[test]
    fn day1_res() {
        let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
        let result = day2_1(&d);
        println!("result: {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day2_1.txt").unwrap();
        let result = day2_1(&d);
        println!("result: {result}");
        assert_eq!(result, 631);
    }
}
