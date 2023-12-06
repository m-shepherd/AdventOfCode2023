use crate::utils::fileutils;
use crate::problems::problem::Problem;

pub struct Problem6 {}

impl Problem for Problem6 {
    fn solve() {
        let data = fileutils::load_problem_file(6);

        let lines: Vec<&str> = data.split("\n").map(|x| x.trim()).collect();
        let time_elements: Vec<&str> = lines[0].split(" ").map(|x| x.trim()).filter(|x| x.len() > 0 && x != &"Time:").collect();
        let distance_elements: Vec<&str> = lines[1].split(" ").map(|x| x.trim()).filter(|x| x.len() > 0 && x != &"Distance:").collect();

        let mut time_string = String::from("");
        for element in time_elements {
            time_string += element; 
        }
        println!("Time {time_string}");
        let time = time_string.parse::<u64>().unwrap();

        let mut distance_string = String::from("");
        for element in distance_elements {
            distance_string += element;
        }
        println!("Distance {distance_string}");
        let distance = distance_string.parse::<u64>().unwrap();

        let mut num_options = 0;
        for j in 1..time {
            let distance_travelled = j * (time - j); 
            if distance_travelled > distance {
                num_options += 1;
            }
        }
        println!("Result is {num_options}");
    }
}

