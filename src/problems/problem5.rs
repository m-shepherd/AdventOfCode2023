use crate::utils::fileutils;
use crate::problems::problem::Problem;

pub struct Problem5 {}

fn get_source_element(dest_element: u32, start_string: &str, lines: &Vec<&str>) -> u32 {
    let mut current_index = lines.iter().position(|&x| x == start_string).unwrap() + 1;
    while current_index < lines.len() && lines[current_index].chars().nth(0).unwrap().is_numeric() {
        let elements: Vec<u32> = lines[current_index].split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap()).collect();
        if elements[0] <= dest_element && (elements[0].checked_add(elements[2]) == None || elements[0] + elements[2] >= dest_element) {
            return elements[1] + (dest_element - elements[0]);
        }
        current_index += 1;
    }
    return dest_element;
}

impl Problem for Problem5 {
    fn solve() {
        let data = fileutils::load_problem_file(5);

        let lines: Vec<&str> = data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).collect();

        let seed_line: Vec<&str> = lines[0].split(":").map(|x| x.trim()).collect();
        let seed_data: Vec<u32> = seed_line[1].split(" ").map(|x| x.parse::<u32>().unwrap()).collect();


        let mut location = 0;
        let mut seed_found = false;
        while !seed_found {
            let humid = get_source_element(location, "humidity-to-location map:", &lines);
            let temp = get_source_element(humid, "temperature-to-humidity map:", &lines);
            let light = get_source_element(temp, "light-to-temperature map:", &lines);
            let water = get_source_element(light, "water-to-light map:", &lines);
            let fert = get_source_element(water, "fertilizer-to-water map:", &lines);
            let soil = get_source_element(fert, "soil-to-fertilizer map:", &lines);
            let seed = get_source_element(soil, "seed-to-soil map:", &lines);

            let mut i = 0;
            while i < seed_data.len() {
                let start = seed_data[i];
                let range = seed_data[i+1];
                seed_found = start <= seed && start + range >= seed;
                if seed_found { break; }
                i += 2;
            }

            if seed_found { break; }
            location += 1;
        }

        println!("Min Location {}", location);
    }
}
