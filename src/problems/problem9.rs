use crate::utils::fileutils;

use super::problem::Problem;

pub struct Problem9 {}

impl Problem for Problem9 {
    fn solve() {
        let data = fileutils::load_problem_file(9);
        let mut sum_new_value = 0;

        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            let elements: Vec<i64> = line.split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<i64>().unwrap()).collect();
            let mut differences: Vec<Vec<i64>> = vec![elements];

            let mut index = 0;
            while differences[index].iter().filter(|x| *x != &0).count() > 0 {
                let mut new_differences: Vec<i64> = Vec::new();

                for i in 0..(differences[index].len() - 1) {
                    new_differences.push(differences[index][i + 1] - differences[index][i]);
                }

                differences.push(new_differences);
                index += 1;
            }

            let mut new_item = 0;
            for update_index in (0..(differences.len()-1)).rev() {
                new_item = differences[update_index][0] - new_item;
                differences[update_index].insert(0, new_item);
            }    

            sum_new_value += differences[0][0];
        }

        println!("Sum of new values: {sum_new_value}");
    }
}
