use crate::utils::fileutils;
use itertools;

use super::problem::Problem;

pub struct Problem1 {}

fn check_for_digit_string(digit_chars: &Vec<char>, current_index: usize, current_chars: &Vec<char>, digit_value: usize) -> Option<usize> {
    for i in current_index..(current_index + digit_chars.len()) {
        if i >= current_chars.len() || current_chars[i] != digit_chars[i - current_index] {
            return None;
        }
    }
    Some(digit_value)
}

fn get_number(current_chars: &Vec<char>, digit_chars: &Vec<Vec<char>>, range_reverse: bool) -> String {
    let range = match range_reverse {
        false => itertools::Either::Left(0..current_chars.len()),
        true => itertools::Either::Right((0..current_chars.len()).rev())
    };

    for i in range {
        if current_chars[i].is_digit(10) {
            return current_chars[i].to_string();
        }
        for (index, digit) in digit_chars.iter().enumerate() {
            if check_for_digit_string(&digit, i, &current_chars, index + 1).is_some() {
                return (index + 1).to_string();
            }
        }
    }
    String::from("")
}

impl Problem for Problem1 {
    fn solve() {
        let data = fileutils::load_problem_file(1);
        let digit_strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let digit_chars: Vec<Vec<char>> = digit_strings.iter().map(|x| x.chars().collect()).collect();

        let mut total_sum = 0;

        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            let chars: Vec<char> = line.chars().collect();
            let mut number: String = String::from("");

            number = number + &get_number(&chars, &digit_chars, false) + &get_number(&chars, &digit_chars, true);

            total_sum += number.parse::<u32>().unwrap();
        }

        println!("Total Sum: {total_sum}");
    }
}
