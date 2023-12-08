use crate::utils::fileutils;

use super::problem::Problem;
use std::collections::HashMap;
use num::integer::lcm;

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

        let start_elements: Vec<&str> = movements.keys().filter(|x| x.chars().nth(2).unwrap() == 'A').map(|x| *x).collect();

        let mut steps: Vec<u128> = Vec::new();
        let mut num_steps: u128 = 1;
        for element in start_elements {
            let mut current_element = element;
            let mut element_steps = 0;
            while current_element.chars().nth(2).unwrap() != 'Z' { 
                match directions[(element_steps % directions.len() as u128) as usize] {
                    "L" => current_element = movements.get(current_element).unwrap().left,
                    "R" => current_element = movements.get(current_element).unwrap().right,
                    _ => {}
                }
                element_steps += 1;
            }
            steps.push(element_steps);
        }

        for i in 0..steps.len() {
            num_steps = lcm(steps[i], num_steps);
        }
        println!("Number of steps required: {num_steps}");
    }
}
