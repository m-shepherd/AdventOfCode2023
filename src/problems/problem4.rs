use crate::utils::fileutils;

use super::problem::Problem;
use std::collections::HashMap;

pub struct Problem4 {}

impl Problem for Problem4 {
    fn solve() {
        let data = fileutils::load_problem_file(4);
        let mut scratchcards: HashMap<u32, u32> = HashMap::new();
        scratchcards.insert(1, 1);

        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            let elements: Vec<&str> = line.split(":").collect();
            let name_parts: Vec<&str> = elements[0].split(" ").filter(|x| x.trim().len() > 0).collect();
            let card_number: u32 = name_parts[1].parse::<u32>().unwrap();

            if scratchcards.get(&card_number).is_none() {
                scratchcards.insert(card_number, 1);
            } else if card_number != 1 {
                scratchcards.insert(card_number, 1 + scratchcards.get(&card_number).unwrap());
            }

            let number_strings: Vec<&str> = elements[1].split("|").map(|x| x.trim()).collect();
            let winning_numbers: Vec<i32> = number_strings[0].split(" ").filter(|x| x.trim().len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();
            let player_numbers: Vec<i32> = number_strings[1].split(" ").filter(|x| x.trim().len() > 0).map(|x| x.parse::<i32>().unwrap()).collect();

            let mut num_wins = 0;
            for number in player_numbers {
                if winning_numbers.contains(&number) {
                    num_wins += 1;
                }
            }

            for i in (card_number+1)..(card_number+1+num_wins) {
                if scratchcards.get(&i).is_some() {
                    scratchcards.insert(i, scratchcards.get(&i).unwrap() + scratchcards.get(&card_number).unwrap());
                } else {
                    scratchcards.insert(i, *scratchcards.get(&card_number).unwrap());
                }
            }
        }

        let mut total_score = 0;
        for value in scratchcards.values() {
            total_score += value;
        }

        println!("Total Score is: {total_score}");
    }
}
