#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use std::cmp::max;
use std::cmp::min;

fn absolute_difference_u32(a: &u32, b: &u32) -> u32 {
    max(a, b) - min(a, b)
}

fn day1_1(data: &Vec<String>) -> u32 {
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();
    data.iter().for_each(|l| {
        let mut parts = l.split_whitespace();
        let num1 = parts.next().unwrap().parse::<u32>().unwrap();
        let num2 = parts.next().unwrap().parse::<u32>().unwrap();
        col1.push(num1);
        col2.push(num2);
    });
    col1.sort();
    col2.sort();
    col1.iter()
        .zip(col2.iter())
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
        assert_eq!(result, 11);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day1_1.txt").unwrap();
        let result = day1_1(&d);
        assert_eq!(result, 1223326);
    }
}
