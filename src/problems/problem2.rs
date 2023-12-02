use crate::utils::fileutils;
use std::collections::HashMap;

use super::problem::Problem;

pub struct Problem2 {}

fn get_game_power(game_data: &str) -> u32 {
    let elements: Vec<&str> = game_data.split(":").collect();
    let ball_pulls: Vec<&str> = elements[1].split([',', ';']).map(|x| x.trim()).filter(|x| x.len() > 0).collect();
    let mut color_map: HashMap<&str, u32> = HashMap::new();

    for ball in ball_pulls {
        let parts: Vec<&str> = ball.split(" ").collect();
        let count = parts[0].parse::<u32>().unwrap();

        let color_data = color_map.get(parts[1]);
        if color_data.is_none() || color_data.unwrap() < &count {
            color_map.insert(parts[1], count);
        }
    }

    let mut game_power = 1;
    for value in color_map.values() {
        game_power *= value;
    }
    game_power
}

impl Problem for Problem2 {
    fn solve() {
        let data = fileutils::load_problem_file(2);
        let mut valid_games_sum = 0;
        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            valid_games_sum += get_game_power(line);
        }
        println!("Sum of Valid Games: {valid_games_sum}");
    }
}
