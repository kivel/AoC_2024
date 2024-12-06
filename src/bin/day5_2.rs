use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug)]
#[allow(dead_code)]
struct Rules {
    rules: Vec<(usize, usize)>,
}

impl Rules {
    fn from_vec_string(vec: &Vec<String>) -> Self {
        // let rules: Vec<(usize, usize)> =
        Self {
            rules: vec
                .iter()
                .filter(|&line| line.contains("|"))
                .map(|line| {
                    line.split_once("|")
                        .map(|(left, right)| {
                            (
                                left.parse::<usize>().ok().unwrap(),
                                right.parse::<usize>().ok().unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Update {
    pages: Vec<usize>,
}

impl Update {
    fn from_str(s: &str) -> Self {
        Self {
            pages: s.split(",").map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn validate(&self, rules: &HashSet<&str>) -> Option<usize> {
        match self.is_valid(rules) {
            true => Some(self.get_center()),
            false => None,
        }
    }

    fn is_valid(&self, rules: &HashSet<&str>) -> bool {
        self.pages
            .windows(2)
            .all(|page_pair| rules.contains(format!("{}|{}", page_pair[0], page_pair[1]).as_str()))
    }

    fn get_center(&self) -> usize {
        println!("x");
        self.pages[self.pages.len() / 2]
    }
}

fn order_vector(input: Vec<usize>, rules: &Vec<(usize, usize)>) -> Vec<usize> {
    // Step 1: Build the graph
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();

    for &(a, b) in rules {
        graph.entry(a).or_default().insert(b);
        *in_degree.entry(b).or_default() += 1;
        in_degree.entry(a).or_default(); // Ensure `a` is in the in-degree map
    }

    // Step 2: Perform topological sort
    let mut queue: VecDeque<usize> = VecDeque::new();
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut result = Vec::new();
    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Ensure the result only contains numbers present in the input
    result.retain(|x| input.contains(x));

    // Step 3: Append any missing numbers from the input
    let mut remaining: Vec<usize> = input.into_iter().filter(|x| !result.contains(x)).collect();
    result.append(&mut remaining);

    result
}

fn day5_2(data: &Vec<String>) -> usize {
    let rule_list = Rules::from_vec_string(data);

    // from part 1:
    let mut rules: HashSet<&str> = HashSet::new();
    let _: Vec<bool> = data
        .iter()
        .filter(|line| line.contains("|"))
        .map(|line| rules.insert(line))
        .collect();

    // only invalid updates --> .filter(|u| !u.is_valid(&rules))
    data.iter()
        .filter(|line| line.contains(","))
        .map(|line| Update::from_str(line))
        .filter(|u| !u.is_valid(&rules))
        .map(|update| order_vector(update.pages, &rule_list.rules))
        .inspect(|v| println!("{v:?}"))
        .filter_map(|pages| Update { pages: pages }.validate(&rules))
        .sum()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day5_1_test.txt").unwrap();
    let sum = day5_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day5_2};

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day5_1_test.txt").unwrap();
        let result = day5_2(&d);
        println!("result: {result}");
        assert_eq!(result, 123);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day5_1.txt").unwrap();
        let result = day5_2(&d);
        println!("result: {result}");
        assert_eq!(result, 5588);
    }
}
