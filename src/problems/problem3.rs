use crate::utils::fileutils::load_problem_file;

use super::problem::Problem;

pub struct Problem3 {}

fn is_valid_symbol(grid: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    grid.len() > y && grid[y].len() > x && grid[y][x] != '.' && !grid[y][x].is_digit(10)
}

fn check_if_symbol_surrounding(grid: &Vec<Vec<char>>, y: usize, min_x: usize, max_x: usize) -> bool {
    let min_x = if min_x > 0 { min_x } else {1};

    for i in (min_x-1)..(max_x+2) {
        if (y > 0 && is_valid_symbol(grid, y-1, i)) 
            || is_valid_symbol(grid, y, i) 
            || is_valid_symbol(grid, y+1, i) {
            return true;
        }
    }
    false
}

impl Problem for Problem3 {
    fn solve() {
        let data = load_problem_file(3);
        let grid: Vec<Vec<char>> = data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.chars().collect()).collect();
        let mut sum_of_parts = 0;

        for y in 0..grid.len() {
            let mut x = 0;
            while x < grid[y].len() {
                let mut current_number = String::from("");
                let min_x = x;
                while x < grid[y].len() && grid[y][x].is_digit(10) {
                    current_number.push(grid[y][x]);
                    x += 1;
                }

                if current_number != "" && check_if_symbol_surrounding(&grid, y, min_x, x-1) {
                    sum_of_parts += current_number.parse::<i32>().unwrap();
                }
                x += 1;
            }
        }

        println!("Sum of parts is: {sum_of_parts}");
    }
}
