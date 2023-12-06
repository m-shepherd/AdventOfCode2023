use crate::utils::fileutils;
use crate::problems::problem::Problem;

pub struct Problem6 {}

impl Problem for Problem6 {
    fn solve() {
        let data = fileutils::load_problem_file(6);

        let lines: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();
        let time_elements: Vec<u32> = lines[0].split(" ").map(|x| x.trim()).filter(|x| x.len() > 0 && x != &"Time:").map(|x| x.parse::<u32>().unwrap()).collect();
        let distance_elements: Vec<u32> = lines[1].split(" ").map(|x| x.trim()).filter(|x| x.len() > 0 && x != &"Distance:").map(|x| x.parse::<u32>().unwrap()).collect();

        let mut result = 1;
        for i in 0..time_elements.len() {
            let mut num_options = 0;
            for j in 1..time_elements[i] {
                let distance_travelled = j * (time_elements[i] - j); 
                if distance_travelled > distance_elements[i] {
                    num_options += 1;
                }
            }
            result *= num_options;
        }
        println!("Result is {result}");
    }
}
