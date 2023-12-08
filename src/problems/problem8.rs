use crate::utils::fileutils;

use super::problem::Problem;
use std::collections::HashMap;

#[derive(Debug)]
struct MoveStep<'a> { left: &'a str, right: &'a str }

pub struct Problem8 {}

impl Problem for Problem8 {
    fn solve() {
        let data = fileutils::load_problem_file(8);
        let mut movements: HashMap<&str, MoveStep> = HashMap::new();
        let mut directions: Vec<&str> = Vec::new();

        for (index, line) in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).enumerate() {
            if index == 0 {
                directions = line.split("").filter(|x| x.len() > 0).collect();
            } else {
                let elements: Vec<&str> = line.split("=").map(|x| x.trim()).collect();
                let left = &elements[1][1..4];
                let right = &elements[1][6..9];

                movements.insert(elements[0], MoveStep { left, right });
            }
        }

        let mut current_element = "AAA";
        let mut num_steps = 0;
        while current_element != "ZZZ" {
            match directions[num_steps % directions.len()] {
                "L" => current_element = movements.get(current_element).unwrap().left,
                "R" => current_element = movements.get(current_element).unwrap().right,
                _ => {}
            }
            num_steps += 1;
        }
        println!("Number of steps required: {num_steps}");
    }
}
