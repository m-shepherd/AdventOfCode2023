use crate::utils::fileutils;

use super::problem::Problem;

pub struct Problem11 {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point { x: usize, y: usize }

fn get_expanded_universe(data: &str) -> Vec<Vec<&str>> {
    let mut universe: Vec<Vec<&str>> = Vec::new();

    for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
        let elements: Vec<&str> = line.split("").map(|x| x.trim()).filter(|x| x.len() > 0).collect();
        let num_galaxies = elements.clone().into_iter().filter(|x| *x == "#").count();
            
        if num_galaxies == 0 {
            universe.push(elements.clone());
        }
        universe.push(elements);
    }

    let mut index = 0;
    while index < universe[0].len() {
        let mut column: Vec<&str> = Vec::new();
        for y in 0..universe.len() {
            column.push(universe[y][index]);
        }
        if column.into_iter().filter(|a| *a == "#").count() == 0 {
            for y in 0..universe.len() {
                universe[y].insert(index, ".");
            }
            index += 1;
        }
        index += 1;
    }

    universe
}

fn get_galaxy_locations(universe: &Vec<Vec<&str>>) -> Vec<Point> {
    let mut locations: Vec<Point> = Vec::new();
    for y in 0..universe.len() {
        for x in 0..universe[y].len() {
            if universe[y][x] == "#" {
                locations.push(Point { x, y });
            }
        }
    }
    locations
}

impl Problem for Problem11 {
    fn solve() {
        let data = fileutils::load_problem_file(11);
        
        let universe = get_expanded_universe(&data);
        let locations = get_galaxy_locations(&universe);

        let mut total_distance = 0;
        let mut checked_galaxies: Vec<Point> = Vec::new();

        for location in &locations {
            for i in 0..locations.len() {
                if locations[i] == *location || checked_galaxies.contains(location) || checked_galaxies.contains(&locations[i]) { 
                    continue; 
                }

                let x_distance = locations[i].x.abs_diff(location.x);
                let y_distance = locations[i].y.abs_diff(location.y);
                total_distance += x_distance + y_distance;
            }
            checked_galaxies.push(location.clone());
        }

        println!("Total distance between pairs of galaxies is {total_distance}");
    }
}
