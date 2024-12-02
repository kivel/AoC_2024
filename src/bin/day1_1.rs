#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use std::cmp::max;
use std::cmp::min;

// There is no build in `abs` for u32 in Rust, who would have guessed that.
fn absolute_difference_u32(a: &u32, b: &u32) -> u32 {
    max(a, b) - min(a, b)
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day1_1(data: &Vec<String>) -> u32 {
    // two empty lists
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    // split each input at the whitespace and push the values into the lists.
    data.iter().for_each(|l| {
        let mut parts = l.split_whitespace();
        let num1 = parts.next().unwrap().parse::<u32>().unwrap();
        let num2 = parts.next().unwrap().parse::<u32>().unwrap();
        left_list.push(num1);
        right_list.push(num2);
    });
    // sort the lists.
    left_list.sort();
    right_list.sort();
    // iterate over both lists, map the absolute difference funtion onto each element and buld the sum
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(x, y)| absolute_difference_u32(x, y))
        .sum()
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day1_1_test.txt").unwrap();
    let sum = day1_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day1_1};

    #[test]
    fn day1_res() {
        let d = advent_of_code::Reader::read_file("./input/day1_1_test.txt").unwrap();
        let result = day1_1(&d);
        println!("result: {result}");
        assert_eq!(result, 11);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day1_1.txt").unwrap();
        let result = day1_1(&d);
        println!("result: {result}");
        assert_eq!(result, 1223326);
    }
}
