use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug)]
#[allow(dead_code)]
struct Updates {
    pages: Vec<usize>,
}

impl Updates {
    fn from_str(s: &str) -> Self {
        Self {
            pages: s.split(",").map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn validate(&self, rules: &HashSet<&str>) -> Option<usize> {
        match self
            .pages
            .windows(2)
            .all(|page| rules.contains(format!("{}|{}", page[0], page[1]).as_str()))
        {
            true => Some(self.get_center()),
            false => None,
        }
    }

    fn get_center(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

fn day5_1(data: &Vec<String>) -> usize {
    let mut rules: HashSet<&str> = HashSet::new();
    let _: Vec<bool> = data
        .iter()
        .filter(|line| line.contains("|"))
        .map(|line| rules.insert(line))
        .collect();
    data.iter()
        .filter(|line| line.contains(","))
        .map(|line| Updates::from_str(line))
        .filter_map(|u| u.validate(&rules))
        .sum()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day5_1_test.txt").unwrap();
    let sum = day5_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day5_1};

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day5_1_test.txt").unwrap();
        let result = day5_1(&d);
        println!("result: {result}");
        assert_eq!(result, 143);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day5_1.txt").unwrap();
        let result = day5_1(&d);
        println!("result: {result}");
        assert_eq!(result, 5588);
    }
}
