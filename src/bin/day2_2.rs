use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// There is no build in `abs` for u32 in Rust, who would have guessed that.
fn step_size_ok(a: &u32, b: &u32, step_size: usize) -> bool {
    match a.abs_diff(*b) {
        step if step > step_size.try_into().unwrap() => false, // must be below step_size
        step if step == 0 => false,                            // can't be zero
        _ => true,
    }
}

fn is_valid_bad_indices(vec: &Vec<u32>) -> (bool, HashSet<usize>) {
    let window_size: usize = 2;
    let mut bad_indices = HashSet::new();
    vec.windows(window_size)
        .enumerate()
        .filter(|(_, _)| {
            let monotonic_inc = vec.windows(2).all(|w| (w[0] < w[1]));
            let monotonic_dec = vec.windows(2).all(|w| (w[0] > w[1]));
            let step_size = vec.windows(2).all(|w| step_size_ok(&w[0], &w[1], 3));
            !((monotonic_inc || monotonic_dec) && step_size)
        })
        .for_each(|(i, _)| {
            bad_indices.insert(i);
            bad_indices.insert(i + 1);
        });

    (bad_indices.is_empty(), bad_indices)
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
        match is_valid_bad_indices(&steps) {
            (true, _) => Some(Level { steps }),
            (false, bad_indices) => {
                match bad_indices.iter().any(|i| {
                    let mut local_vec = steps.clone();
                    print!(" {} ", *i);
                    local_vec.remove(*i);
                    is_valid_bad_indices(&local_vec).0
                }) {
                    true => Some(Level { steps }),
                    false => None,
                }
            }
        }
    }
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day2_2(data: &Vec<String>) -> u32 {
    data.iter()
        .inspect(|l| print!("line: {l} |"))
        .map(|l| match Level::new(l) {
            Some(_) => 1,
            None => 0,
        })
        .inspect(|x| println!(" --> {x}"))
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
    let sum = day2_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day2_2};

    #[test]
    fn day1_res() {
        let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
        let result = day2_2(&d);
        println!("result: {result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day2_1.txt").unwrap();
        let result = day2_2(&d);
        println!("result: {result}");
        assert_eq!(result, 665);
    }
}
