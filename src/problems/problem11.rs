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
            
        let mut cloned_elements = elements.clone();
        if num_galaxies == 0 {
            cloned_elements.insert(0, "Y");
            universe.push(cloned_elements);
        } else {
            cloned_elements.insert(0, "N");
            universe.push(cloned_elements);
        }
    }

    let mut header_column: Vec<&str> = Vec::new();
    for x in 0..universe[0].len() {
        let mut galaxy_count = 0;
        for y in 0..universe.len() {
            if universe[y][x] == "#" {
                galaxy_count += 1;
            }
        }
        if galaxy_count == 0 {
            header_column.push("Y");
        } else {
            header_column.push("N");
        }
    }

    universe.insert(0, header_column);

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
        let times_larger: u128 = 1000000;

        for location in &locations {
            for i in 0..locations.len() {
                if locations[i] == *location || checked_galaxies.contains(location) || checked_galaxies.contains(&locations[i]) { 
                    continue; 
                }

                let mut x_distance: u128 = 0;
                if locations[i].x < location.x {
                    for pos in locations[i].x..location.x {
                        if universe[0][pos] == "Y" {
                            x_distance += times_larger;
                        } else {
                            x_distance += 1
                        }
                    }
                } else {
                    for pos in location.x..locations[i].x {
                        if universe[0][pos] == "Y" {
                            x_distance += times_larger;
                        } else {
                            x_distance += 1
                        }
                    }
                }

                let mut y_distance: u128 = 0;
                if locations[i].y < location.y {
                    for pos in locations[i].y..location.y {
                        if universe[pos][0] == "Y" {
                            y_distance += times_larger;
                        } else {
                            y_distance += 1;
                        }
                    }
                } else {
                    for pos in location.y..locations[i].y {
                        if universe[pos][0] == "Y" {
                            y_distance += times_larger;
                        } else {
                            y_distance += 1;
                        }
                    }
                }
                total_distance += x_distance + y_distance;
            }
            checked_galaxies.push(location.clone());
        }

        println!("Total distance between pairs of galaxies is {total_distance}");
    }
}
