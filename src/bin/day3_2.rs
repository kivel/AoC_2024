#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use std::{collections::VecDeque, fs};

use regex::Regex;

fn get_instructions(text: &str) -> Vec<&str> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    // Collect all matches as strings
    re.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn product_from_match(s: &str) -> usize {
    let parts = s.split_once(",").unwrap();
    let f1: usize = parts.0.strip_prefix("mul(").unwrap().parse().unwrap();
    let f2: usize = parts.1.strip_suffix(")").unwrap().parse().unwrap();
    f1 * f2
}
// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day3_2(data: &str) -> usize {
    let mut instructions = VecDeque::from(get_instructions(data));

    let mut factor: usize = 1;
    let mut sum: usize = 0;

    while !instructions.is_empty() {
        let i = instructions.pop_front();
        let p = match i {
            Some(inst) if inst.contains("do()") => {factor =1; continue;},
            Some(inst) if inst.contains("don't()") => {factor = 0; continue;},
            Some(inst) => product_from_match(inst),
            None => panic!("Unexpected end of instructions"),
        };
        sum += p * factor;
        println!("i={i:?}  | factor={factor} | p={p} | sum = {sum}");
    };
    sum

}
fn main() {
    let d = fs::read_to_string("./input/day3_2_test.txt").unwrap(); // Read the entire file into a String
    let sum = day3_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{advent_of_code, day3_2};

    #[test]
    fn res_test() {
        let d = fs::read_to_string("./input/day3_2_test.txt").unwrap(); // Read the entire file into a String
        let result = day3_2(&d);
        println!("result: {result}");
        assert_eq!(result, 48);
    }

    #[test]
    fn final_test() {
        let d = fs::read_to_string("./input/day3_1.txt").unwrap(); // Read the entire file into a String
        let result = day3_2(&d);
        println!("result: {result}");
        assert_eq!(result, 71668682);
    }
}
