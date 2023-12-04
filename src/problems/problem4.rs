use crate::utils::fileutils;

use super::problem::Problem;

pub struct Problem4 {}

impl Problem for Problem4 {
    fn solve() {
        let data = fileutils::load_problem_file(4);
        let mut total_score = 0;

        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            let elements: Vec<&str> = line.split(":").collect();
            let number_strings: Vec<&str> = elements[1].split("|").map(|x| x.trim()).collect();
            let mut game_score = 0;

            let winning_numbers: Vec<i32> = number_strings[0].split(" ").filter(|x| x.trim().len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();
            let player_numbers: Vec<i32> = number_strings[1].split(" ").filter(|x| x.trim().len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();

            for number in player_numbers {
                if winning_numbers.contains(&number) {
                    match game_score {
                        0 => game_score = 1,
                        _ => game_score *= 2
                    }
                }
            }
            total_score += game_score;
        }

        println!("Total Score is: {total_score}");
    }
}
