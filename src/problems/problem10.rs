use crate::utils::fileutils;

use super::problem::Problem;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Default, Copy)]
struct Point { x: usize, y: usize }

pub struct Problem10 {}

fn is_in_bounds(element: &Point, grid: &Vec<Vec<&str>>) -> bool {
    element.x < grid[element.y].len() && element.y < grid.len()
}

fn check_positions(element_1: &Point, element_2: &Point, connected_loop: &HashMap<Point, &str>, search_positions: &mut Vec<Point>, grid: &Vec<Vec<&str>>, for_start: bool) {
    if is_in_bounds(element_1, grid) && grid[element_1.y][element_1.x] != "." && ((!for_start && connected_loop.get(element_1).is_none()) || (for_start && !search_positions.contains(element_1))) {
        search_positions.push(element_1.clone());
    } else if is_in_bounds(element_2, grid) && grid[element_2.y][element_2.x] != "." && ((!for_start && connected_loop.get(element_2).is_none()) || (for_start && !search_positions.contains(element_2))) {
        search_positions.push(element_2.clone());
    }
}

impl Problem for Problem10 {
    fn solve() {
        let data = fileutils::load_problem_file(10);
        let mut grid: Vec<Vec<&str>> = Vec::new();
        let mut connected_loop: HashMap<Point, &str> = HashMap::new();
        let mut search_positions: Vec<Point> = Vec::new();

        for (index, line) in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).enumerate() {
            let current_line: Vec<&str> = line.split("").map(|x| x.trim()).filter(|x| x.len() > 0).collect();
            
            if current_line.contains(&"S") {
                let current_point = Point { x: line.find("S").unwrap(), y: index };
                connected_loop.insert(current_point.clone(), "S");
                search_positions.push(current_point);
            }
            
            grid.push(current_line);
        }

        let start_position: Point = search_positions[0].clone();

        while search_positions.len() > 0 {
            let above: Point;
            if search_positions[0].y >= 1 {
                above = Point { x: search_positions[0].x, y: search_positions[0].y - 1 };
            } else {
                above = Point { x: search_positions[0].x, y: search_positions[0].y };
            }
            let below = &Point { x: search_positions[0].x, y: search_positions[0].y + 1 };
            let left: Point;
            if search_positions[0].x >= 1 {
                left = Point { x: search_positions[0].x - 1, y: search_positions[0].y };
            } else {
                left = Point { x: search_positions[0].x, y: search_positions[0].y };
            }
            let right = &Point { x: search_positions[0].x + 1, y: search_positions[0].y };

            match grid[search_positions[0].y][search_positions[0].x] {
                "|" => check_positions(&above, below, &connected_loop, &mut search_positions, &grid, false),
                "-" => check_positions(&left, right, &connected_loop, &mut search_positions, &grid, false),
                "L" => check_positions(&above, right, &connected_loop, &mut search_positions, &grid, false),
                "J" => check_positions(&above, &left, &connected_loop, &mut search_positions, &grid, false),
                "7" => check_positions(below, &left, &connected_loop, &mut search_positions, &grid, false),
                "F" => check_positions(below, right, &connected_loop, &mut search_positions, &grid, false),
                "S" => {
                    check_positions(&above, below, &connected_loop, &mut search_positions, &grid, true);
                    check_positions(&left, right, &connected_loop, &mut search_positions, &grid, true);
                    check_positions(&above, right, &connected_loop, &mut search_positions, &grid, true);
                    check_positions(&above, &left, &connected_loop, &mut search_positions, &grid, true);
                    check_positions(below, &left, &connected_loop, &mut search_positions, &grid, true);
                    check_positions(below, right, &connected_loop, &mut search_positions, &grid, true);
                }
                _ => {}
            } 
            connected_loop.insert(search_positions[0].clone(), grid[search_positions[0].y][search_positions[0].x]);
            search_positions.remove(0);
        }

        let mut path_options: Vec<Point> = Vec::new();
        if connected_loop.get(&Point { x: start_position.x - 1, y: start_position.y }).is_some() && 
                              (grid[start_position.y][start_position.x - 1] == "-" || grid[start_position.y][start_position.x - 1] == "L" ||
                               grid[start_position.y][start_position.x - 1] == "F") {
            path_options.push(Point { x: start_position.x - 1, y: start_position.y });
        }
        if connected_loop.get(&Point { x: start_position.x + 1, y: start_position.y }).is_some() {
            path_options.push(Point { x: start_position.x + 1, y: start_position.y });
        }
        if start_position.y > 0 && connected_loop.get(&Point { x: start_position.x, y: start_position.y - 1 }).is_some() &&
                              (grid[start_position.y-1][start_position.x] == "|" || grid[start_position.y-1][start_position.x] == "7" ||
                               grid[start_position.y-1][start_position.x] == "F") {
            path_options.push(Point { x: start_position.x, y: start_position.y - 1 });
        }
        if connected_loop.get(&Point { x: start_position.x, y: start_position.y + 1 }).is_some() && 
                              (grid[start_position.y + 1][start_position.x] == "|" || grid[start_position.y + 1][start_position.x] == "L" ||
                               grid[start_position.y + 1][start_position.x] == "J") {
            path_options.push(Point { x: start_position.x, y: start_position.y + 1 });
        }

        let mut enclosed_items = 0;

        let mut updated_squares: Vec<Point> = Vec::new();
        for y in 0..grid.len() {
            let mut loop_open_index: i32 = -1;
            for x in 0..grid[y].len() {
                if connected_loop.contains_key(&Point {x, y}) {
                    if loop_open_index == -1 {
                        loop_open_index = x as i32;
                    } else {
                        for i in (loop_open_index+1)..(x as i32) {
                            enclosed_items += 1;
                            updated_squares.push(Point {x: i as usize, y});
                        }
                        loop_open_index = -1;
                    }
                }
            }
        }

        for x in 0..grid[0].len() {
            let mut loop_open_index: i32 = -1;
            for y in 0..grid.len() {
                if connected_loop.contains_key(&Point {x, y}) {
                    if loop_open_index == -1 {
                        loop_open_index = y as i32;
                    } else {
                        for i in (loop_open_index+1)..(y as i32) {
                            if !updated_squares.contains(&Point {x, y: i as usize}) {
                                enclosed_items += 1;
                                updated_squares.push(Point {x, y: i as usize});
                            }
                        }
                        loop_open_index = -1;
                    }
                }
            }
        } 

        println!("Number of enclosed items: {}", enclosed_items);
    }
}
