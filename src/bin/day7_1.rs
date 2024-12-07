use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn calculate_results(
    v1: usize,
    v2: usize,
    operations: Vec<fn(usize, usize) -> usize>,
) -> HashSet<usize> {
    let mut results = HashSet::new();
    for operation in &operations {
        results.insert(operation(v1, v2));
    }
    results
}

fn parse_lines(lines: &[String]) -> Vec<(usize, Vec<usize>)> {
    lines
        .iter()
        .map(|line| {
            // Split the line at the colon
            let (result, numbers) = line.split_once(':').expect("Failed to split line at ':'");

            // Parse the key (before the colon)
            let key: usize = result
                .trim()
                .parse()
                .expect("Failed to parse key as number");

            // Parse the rest (after the colon)
            let values: Vec<usize> = numbers
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse value as number"))
                .collect();

            (key, values)
        })
        .collect()
}

fn day7_1(data: &Vec<String>) -> usize {
    let operators: Vec<fn(usize, usize) -> usize> = vec![
        |a, b| a + b, // Addition
        |a, b| a * b, // Multiplication
    ];

    let input = parse_lines(data);

    let mut sum: usize = 0;

    for (target, values) in input.iter() {
        let mut values = values.clone();

        // prepare
        let mut results: HashSet<usize> = HashSet::new();
        results.insert(values[0]);

        values.drain(1..).for_each(|v1| {
            let mut prev = results.clone();
            results.clear();
            prev.drain().for_each(|v2| {
                calculate_results(v1, v2, operators.clone())
                    .iter()
                    .filter(|&&v| v <= *target)
                    .for_each(|v| {
                        results.insert(*v);
                    });
            });
        });

        // println!("target:{target} prev results: {results:?}");
        if results.contains(target) {
            sum += *target;
        }
    }
    sum
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day7_test.txt").unwrap();
    let sum = day7_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day7_1};

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day7_test.txt").unwrap();
        let result = day7_1(&d);
        println!("result: {result}");
        assert_eq!(result, 3749);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day7.txt").unwrap();
        let result = day7_1(&d);
        println!("result: {result}");
        assert_eq!(result, 5540634308362);
    }
}
