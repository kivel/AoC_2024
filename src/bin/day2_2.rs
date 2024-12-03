#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn check_conditions(vec: &Vec<u32>) -> bool {
    let steps = vec
        .windows(2)
        .map(|w| w[0] as i32 - w[1] as i32)
        .collect::<Vec<i32>>();
    let monotonic_inc = steps.iter().all(|v| v.is_positive());
    let monotonic_dec = steps.iter().all(|v| v.is_negative());
    let step_size = steps.iter().all(|v| (v != &0 && v.abs() <= 3));
    (monotonic_inc || monotonic_dec) && step_size
}

fn check_conditions_modified_vector(vec: &Vec<u32>) -> Option<usize> {
    match vec
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let mut local_vec = vec.clone();
            print!("i={} ", *i);
            local_vec.remove(*i);
            check_conditions(&local_vec)
        })
        .next()
        .is_some()
    {
        true => Some(1),
        false => None,
    }
}

fn check_level(steps: &String) -> Option<usize> {
    let steps: Vec<u32> = steps
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    match check_conditions(&steps) {
        true => Some(1),
        false => check_conditions_modified_vector(&steps),
    }
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day2_2(data: &Vec<String>) -> u32 {
    data.iter()
        .inspect(|l| print!("line: {l} |"))
        .map(|l| match check_level(l) {
            Some(x) => x as u32,
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
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
        let result = day2_2(&d);
        println!("result: {result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day2_1.txt").unwrap();
        let result = day2_2(&d);
        println!("result: {result}");
        assert_eq!(result, 665);
    }
}
