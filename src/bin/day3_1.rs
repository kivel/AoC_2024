#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use std::fs;

use regex::Regex;

fn get_instructions(text: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    // Collect all matches as strings
    re.find_iter(text).map(|mat| mat.as_str()).collect()
}
// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day3_1(data: &str) -> usize {
    let instructions = get_instructions(data);

    instructions
        .iter()
        .map(|s| {
            let parts = s.split_once(",").unwrap();
            let f1: usize = parts.0.strip_prefix("mul(").unwrap().parse().unwrap();
            let f2: usize = parts.1.strip_suffix(")").unwrap().parse().unwrap();
            f1 * f2
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}
fn main() {
    let d = fs::read_to_string("./input/day3_1_test.txt").unwrap(); // Read the entire file into a String
    let sum = day3_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{advent_of_code, day3_1};

    #[test]
    fn res_test() {
        let d = fs::read_to_string("./input/day3_1_test.txt").unwrap(); // Read the entire file into a String
        let result = day3_1(&d);
        println!("result: {result}");
        assert_eq!(result, 161);
    }

    #[test]
    fn final_test() {
        let d = fs::read_to_string("./input/day3_1.txt").unwrap(); // Read the entire file into a String
        let result = day3_1(&d);
        println!("result: {result}");
        assert_eq!(result, 175700056);
    }
}
