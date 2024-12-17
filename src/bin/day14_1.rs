use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
    grid_size: (isize, isize)
}

impl Robot {
    fn new(position: (isize, isize), velocity: (isize, isize), grid_size: (isize, isize)) -> Self {
        Self { position, velocity, grid_size }
    }

    fn from_string(s: &String, grid_size: (isize, isize)) -> Self {
        // p=0,4 v=3,-3
        let (p_str, v_str) = s.split_once(" ").unwrap();
        let (pxs, pys) = p_str
            .strip_prefix("p=")
            .and_then(|s| s.split_once(","))
            .unwrap();
        let pos_x = pxs.parse::<isize>().unwrap();
        let pos_y = pys.parse::<isize>().unwrap();
        let (vxs, vys) = v_str
            .strip_prefix("v=")
            .and_then(|s| s.split_once(","))
            .unwrap();
        let vel_x = vxs.parse::<isize>().unwrap();
        let vel_y = vys.parse::<isize>().unwrap();
        Self::new((pos_x, pos_y), (vel_x, vel_y), grid_size)
    }

    // fn move_robot(&mut self, time: isize) {
    //     // move the robot
    //     self.position.0 += time * self.velocity.0;
    //     self.position.1 += time * self.velocity.1;
    //     self.position.0 = match self.position.0.is_positive() {
    //         true => self.position.0 % self.grid_size.0,
    //         false => self.grid_size.0 + self.position.0 % self.grid_size.0
    //     };
    //     self.position.1 = match self.position.1.is_positive() {
    //         true => self.position.1 % self.grid_size.1,
    //         false => self.grid_size.1 + self.position.1 % self.grid_size.1
    //     };
    // }

    fn move_robot(&mut self, time: isize) {
        // Move the robot
        self.position.0 += time * self.velocity.0;
        self.position.1 += time * self.velocity.1;
    
        // Correctly wrap around using mathematical modulo
        self.position.0 = ((self.position.0 % self.grid_size.0) + self.grid_size.0) % self.grid_size.0;
        self.position.1 = ((self.position.1 % self.grid_size.1) + self.grid_size.1) % self.grid_size.1;
    }
}

fn count_robots_in_quadrants(robots: Vec<Robot>) -> [isize; 4] {

    let center_x = robots[0].grid_size.0/2;  // Vertical center line
    let center_y = robots[0].grid_size.1/2; // Horizontal center line

    println!("center x = {}, y = {}", center_x, center_y);

    let mut quadrants = [0; 4]; // Q1, Q2, Q3, Q4 counts

    for robot in robots {
        let x = robot.position.0;
        let y = robot.position.1;

        // Ignore robots on the center lines
        if x == center_x || y == center_y {
            continue;
        }

        // Classify the robot into a quadrant
        if x > center_x && y < center_y {
            quadrants[0] += 1; // Q1: Top-right
        } else if x < center_x && y < center_y {
            quadrants[1] += 1; // Q2: Top-left
        } else if x < center_x && y > center_y {
            quadrants[2] += 1; // Q3: Bottom-left
        } else if x > center_x && y > center_y {
            quadrants[3] += 1; // Q4: Bottom-right
        }
    }

    quadrants
}

fn day14_1(data: &Vec<String>, grid_size: (isize, isize)) -> usize {
    // let grid_size = (11,7);
    let mut robots: Vec<Robot> = data.iter().map(|line| Robot::from_string(line, grid_size)).collect();

    robots.iter_mut().for_each(|robot| {
        robot.move_robot(100);
        println!("position: {:?}", &robot.position);
    });

    let robots_per_quadrant = count_robots_in_quadrants(robots);
    println!("quadrants: {:?}", robots_per_quadrant);
    robots_per_quadrant.iter().product::<isize>() as usize
    // 0
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day14_test.txt").unwrap();
    let sum = day14_1(&d, (11, 7));
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day14_1, Robot};

    #[test]
    fn robot_from_string_test() {
        let robot = Robot::from_string(&"p=0,4 v=3,-3".to_owned(), (11,7));
        assert_eq!(robot, Robot::new((0, 4), (3, -3), (11,7)));
    }

    //p=2,4 v=2,-3
    #[test]
    fn robot_move_test() {
        let mut robot = Robot::from_string(&"p=2,4 v=2,-3".to_owned(), (11,7));
        // 1 second
        robot.move_robot(1);
        assert_eq!(robot.position, (4, 1));
        // 2 seconds
        robot.move_robot(1);
        assert_eq!(robot.position, (6, 5));
        // 3 seconds
        robot.move_robot(1);
        assert_eq!(robot.position, (8, 2));
        // 4 seconds
        robot.move_robot(1);
        assert_eq!(robot.position, (10, 6));
        // 5 seconds
        robot.move_robot(1);
        assert_eq!(robot.position, (1, 3));
        // move 5 second in one go
        let mut robot = Robot::from_string(&"p=2,4 v=2,-3".to_owned(), (11,7));
        robot.move_robot(5);
        assert_eq!(robot.position, (1, 3));
    }

    #[test]
    fn res_test() {
        let d = advent_of_code::Reader::read_file("./input/day14_test.txt").unwrap();
        let result = day14_1(&d, (11,7));
        println!("result: {result}");
        assert_eq!(result, 12);
    }

    #[test]
    fn res_final() {
        let d = advent_of_code::Reader::read_file("./input/day14.txt").unwrap();
        let result = day14_1(&d, (101,103));
        println!("result: {result}");
        assert_eq!(result, 236628054);
    }
}
