use crate::utils::fileutils;
use crate::problems::problem::Problem;

pub struct Problem5 {}

fn get_next_step(source: &Vec<u32>, dest_list: &mut Vec<u32>, lines: &Vec<&str>, start_string: &str) {
    let mut source_list = source.clone();
    let mut current_index = lines.iter().position(|&x| x == start_string).unwrap() + 1;
    while current_index < lines.len() && lines[current_index].chars().nth(0).unwrap().is_numeric() && source_list.len() != 0 {
        let elements: Vec<u32> = lines[current_index].split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap()).collect();
        let mut i = 0;
        while i < source_list.len() {
            if elements[1] <= source_list[i] && (elements[1].checked_add(elements[2]) == None || elements[1] + elements[2] >= source_list[i]) {
                dest_list.push(elements[0] + (source_list[i] - elements[1]));
                source_list.remove(i);
            } else {
                i += 1;
            }
        }
        current_index += 1;
    }
    source_list.iter().for_each(|x| dest_list.push(*x));
}

impl Problem for Problem5 {
    fn solve() {
        let data = fileutils::load_problem_file(5);

        let lines: Vec<&str> = data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).collect();

        let seed_line: Vec<&str> = lines[0].split(":").map(|x| x.trim()).collect();
        let seeds: Vec<u32> = seed_line[1].split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        let mut soil: Vec<u32> = vec![];
        let mut fert: Vec<u32> = vec![];
        let mut water: Vec<u32> = vec![];
        let mut light: Vec<u32> = vec![];
        let mut temp: Vec<u32> = vec![];
        let mut humid: Vec<u32> = vec![];
        let mut location: Vec<u32> = vec![];

        get_next_step(&seeds, &mut soil, &lines, "seed-to-soil map:");
        get_next_step(&soil, &mut fert, &lines, "soil-to-fertilizer map:");
        get_next_step(&fert, &mut water, &lines, "fertilizer-to-water map:");
        get_next_step(&water, &mut light, &lines, "water-to-light map:");
        get_next_step(&light, &mut temp, &lines, "light-to-temperature map:");
        get_next_step(&temp, &mut humid, &lines, "temperature-to-humidity map:");
        get_next_step(&humid, &mut location, &lines, "humidity-to-location map:");

        println!("Min Location {}", location.iter().min().unwrap());
    }
}
