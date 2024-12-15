use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
struct Stone {
    value: u64,
}

impl Stone {
    fn new(value: u64) -> Self {
        Self { value }
    }

    fn blink(&mut self) -> Vec<Self> {
        // first rule: if value==0, return Stone with value = 1
        if self.value == 0 {
            vec![Stone::new(1)]
        // second rule: if the number has and even number of digits, split into two Stones
        } else if self.value.to_string().len() % 2 == 0 {
            let str_value = self.value.to_string();
            let (left_half, right_half) = str_value.split_at(self.value.to_string().len() / 2);
            vec![
                Stone::new(left_half.parse::<u64>().unwrap()),
                Stone::new(right_half.parse::<u64>().unwrap()),
            ]
        } else {
            vec![Stone::new(&self.value * 2024)]
        }
    }
}

fn day10_1(data: &str, n_blinks: usize) -> usize {
    let stones: Vec<Stone> = data
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|v| Stone::new(v))
        .collect();

    let mut iterations: HashMap<usize, Vec<Stone>> = HashMap::from([(0, stones)]);
    for i in 0..n_blinks as usize {
        let stones = &iterations[&i];
        let new_stones = stones.iter().map(|s| s.clone().blink()).flatten().collect();
        iterations.insert(i + 1, new_stones);
    }

    iterations.get(&n_blinks).unwrap().len()
}

fn main() {
    // let d = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();
    let d = "125 17";
    let sum = day10_1(&d, 6);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day10_1, Stone};

    #[test]
    fn rule1_test() {
        let mut stone = Stone::new(0);
        let blinked = stone.blink();
        assert_eq!(blinked.len(), 1);
        assert_eq!(blinked[0].value, 1);
    }

    #[test]
    fn rule2_test() {
        let mut stone = Stone::new(253000);
        let blinked = stone.blink();
        assert_eq!(blinked.len(), 2);
        assert_eq!(blinked[0].value, 253);
        assert_eq!(blinked[1].value, 0);
    }

    #[test]
    fn rule3_test() {
        let mut stone = Stone::new(1);
        let blinked = stone.blink();
        assert_eq!(blinked.len(), 1);
        assert_eq!(blinked[0].value, 2024);
    }

    #[test]
    fn res_test() {
        // let d = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();
        let result = day10_1("125 17", 25);
        println!("result: {result}");
        assert_eq!(result, 55312);
    }

    #[test]
    fn res_final() {
        // let d = advent_of_code::Reader::read_file("./input/day10.txt").unwrap();
        let result = day10_1("5 89749 6061 43 867 1965860 0 206250", 25);
        println!("result: {result}");
        assert_eq!(result, 203609);
    }

    // #[test]
    // fn res_final_part2() {
    //     // let d = advent_of_code::Reader::read_file("./input/day10.txt").unwrap();
    //     let result = day10_1("5 89749 6061 43 867 1965860 0 206250", 35);
    //     println!("result: {result}");
    //     assert_eq!(result, 203609);
    // }
}
