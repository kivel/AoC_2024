#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day1_1(data: &Vec<String>) -> u32 {
    data.iter()
        .map(|l| {
            let characters = l.chars();
            let digits = &characters
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<u32>>();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
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
    fn day1_ind() {
        let result = day1_1(&vec!["1abc2".to_string()]);
        assert_eq!(result, 12);
        let result = day1_1(&vec!["pqr3stu8vwx".to_string()]);
        assert_eq!(result, 38);
        let result = day1_1(&vec!["a1b2c3d4e5f".to_string()]);
        assert_eq!(result, 15);
        let result = day1_1(&vec!["treb7uchet".to_string()]);
        assert_eq!(result, 77);
    }

    #[test]
    fn day1_res() {
        let d = advent_of_code::Reader::read_file("./input/day1_1_test.txt").unwrap();
        let result = day1_1(&d);
        assert_eq!(result, 142);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day1_1.txt").unwrap();
        let result = day1_1(&d);
        assert_eq!(result, 55017);
    }
}

