#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use std::collections::HashMap;

// The 2nd part of the puzlle calls again for two lists (given as two columns in a ascii file).
// This time, we need to determine who often a number from the left list apears in the right list.
// Thus, the right list must be a Hashmap with the number as key and the value how often it was in the right list.
//
fn day1_1(data: &Vec<String>) -> u32 {
    // two empty lists
    let mut left_list: Vec<&str> = Vec::new();
    let mut right_hm = HashMap::new();
    // split each input at the whitespace and push the left value into a list and the right value into the hash map, initialize to 1 or increment if present.
    data.iter().for_each(|l| {
        let mut parts = l.split_whitespace();
        let num1 = parts.next().unwrap();
        let num2 = parts.next().unwrap();
        left_list.push(num1);
        right_hm
            .entry(num2)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    // iterate over the left list and lookup how often the respective number was in the left list, multiply itseld with the number of occurrences, and bum the products up.
    left_list
        .iter()
        .map(|&key| {
            match right_hm.get_key_value(key) {
                Some((_, count)) => *count * &key.parse::<u32>().unwrap(),
                None => 0, // if the number is not in the right list, it's not in the count
            }
        })
        .collect::<Vec<u32>>()
        .iter()
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
        assert_eq!(result, 31);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day1_1.txt").unwrap();
        let result = day1_1(&d);
        assert_eq!(result, 21070419);
    }
}
