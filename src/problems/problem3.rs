use crate::utils::fileutils::load_problem_file;

use super::problem::Problem;
use std::collections::HashMap;

pub struct Problem3 {}

#[derive(PartialEq, Eq, Hash)]
struct Point { x: usize, y: usize }

fn get_gears(grid: &Vec<Vec<char>>, gears: &mut HashMap<Point, Vec<i32>>, number: i32, y: usize, x: usize) {
    if grid.len() > y && grid[y].len() > x && grid[y][x] == '*' {
        let current_point = Point { x, y };
        if gears.get(&current_point).is_some() { 
            gears.get_mut(&current_point).unwrap().push(number);
        } else {
            gears.insert(current_point, vec![number]);
        }
    }
}

fn get_surrounding_gears(grid: &Vec<Vec<char>>, gears: &mut HashMap<Point, Vec<i32>>, number: i32, y: usize, min_x: usize, max_x: usize) {
    let min_x = if min_x > 0 { min_x } else {1};

    for i in (min_x-1)..(max_x+2) {
        if y > 0 { get_gears(grid, gears, number, y-1, i) }
        get_gears(grid, gears, number, y, i); 
        get_gears(grid, gears, number, y+1, i);
    }
}

impl Problem for Problem3 {
    fn solve() {
        let data = load_problem_file(3);
        let grid: Vec<Vec<char>> = data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.chars().collect()).collect();
        let mut sum_of_gear_ratios = 0;
        let mut gears: HashMap<Point, Vec<i32>> = HashMap::new();

        for y in 0..grid.len() {
            let mut x = 0;
            while x < grid[y].len() {
                let mut current_number = String::from("");
                let min_x = x;
                while x < grid[y].len() && grid[y][x].is_digit(10) {
                    current_number.push(grid[y][x]);
                    x += 1;
                }

                if current_number != "" {
                    get_surrounding_gears(&grid, &mut gears, current_number.parse::<i32>().unwrap(), y, min_x, x-1);
                }

                x += 1;
            }
        }

        for value in gears.values() {
            if value.len() == 2 {
                sum_of_gear_ratios += value[0] * value[1];
            }
        }

        println!("Sum of gear ratios is: {sum_of_gear_ratios}");
    }
}
