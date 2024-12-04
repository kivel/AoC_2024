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
    s.strip_prefix("mul(")
        .and_then(|inner| inner.strip_suffix(")"))
        .and_then(|contents| contents.split_once(','))
        .map(|(f1, f2)| 
            f1.parse::<usize>()
                .and_then(|x| f2.parse::<usize>().map(|y| x * y))
                .expect("Invalid number in multiplication")
        )
        .expect("Invalid multiplication instruction")
}

fn day3_2(data: &str) -> usize {
    let instructions = get_instructions(data);
    
    instructions.iter()
        .fold((0, 1), |(sum, factor), instruction| match *instruction {
            "do()" => (sum, 1),
            "don't()" => (sum, 0),
            inst => (sum + product_from_match(inst) * factor, factor)
        })
        .0
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
