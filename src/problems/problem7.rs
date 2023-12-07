use crate::utils::fileutils;

use super::problem::Problem;
use std::collections::HashMap;
use std::cmp::Ordering;

pub struct Problem7 {}

struct HandData<'a> { hand: &'a str, bet: u32 }

const CARD_ORDERING: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

fn compare_cards(a: &HandData, b: &HandData) -> Ordering {
    if a.hand == b.hand {
        return Ordering::Equal;
    }

    let a_chars: Vec<char> = a.hand.chars().collect();
    let b_chars: Vec<char> = b.hand.chars().collect();

    for i in 0..a_chars.len() {
        if CARD_ORDERING.into_iter().position(|x| x == a_chars[i]).unwrap() < CARD_ORDERING.into_iter().position(|x| x == b_chars[i]).unwrap() {
            return Ordering::Less;
        } else if CARD_ORDERING.into_iter().position(|x| x == a_chars[i]).unwrap() > CARD_ORDERING.into_iter().position(|x| x == b_chars[i]).unwrap() {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

impl Problem for Problem7 {
    fn solve() {
        let data = fileutils::load_problem_file(7);
        let mut hand_types: Vec<Vec<HandData>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            let mut cards: HashMap<&str, u32> = HashMap::new();
            let elements: Vec<&str> = line.split(" ").collect();
            let hand = elements[0];
            let card_data: Vec<&str> = hand.split("").filter(|x| x.trim().len() > 0).collect();
            for card in &card_data {
                if cards.get(card).is_some() {
                    cards.insert(card, cards.get(card).unwrap() + 1);
                } else {
                    cards.insert(card, 1);
                }
            }
            
            let hand_data = HandData { hand, bet: elements[1].parse::<u32>().unwrap() };
            let values: Vec<&u32> = cards.values().collect();
            if cards.keys().len() == 1 {
                hand_types[6].push(hand_data);
            } else if cards.keys().len() == 2 {
                if values[0] == &1 || values[0] == &4 {
                    hand_types[5].push(hand_data);
                } else {
                    hand_types[4].push(hand_data);
                }
            } else if cards.keys().len() == 3 {
                if values[0] == &3 || values[1] == &3 || values[2] == &3 {
                    hand_types[3].push(hand_data);
                } else {
                    hand_types[2].push(hand_data);
                }
            } else if cards.keys().len() == 4 {
                hand_types[1].push(hand_data);
            } else {
                hand_types[0].push(hand_data);
            }
        }

        let mut total_winnings = 0;
        let mut current_rank = 1;
        for mut set in hand_types {
            set.sort_by(|a, b| compare_cards(a, b));
            for i in 0..set.len() {
                total_winnings += set[i].bet * current_rank;
                current_rank += 1;
            }
        }
        println!("Total winnings: {total_winnings}");
    }
}
