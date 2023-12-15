use crate::utils::fileutils;

use super::problem::Problem;

pub struct Problem12 {}

fn get_possibilities(elements: &[&str], sequences: &[usize]) -> u32 {
    /*print!("Elements: ");
    for element in elements {
        print!("{element},");
    }
    print!(" Sequence: ");
    for element in sequences {
        print!("{element},");
    }
    println!("");*/
    if sequences.len() == 0 && elements.iter().filter(|x| *x == &"#").count() == 0 { return 1; } 
    if elements.len() == 0 || sequences.len() == 0 { return 0; }

    if elements[0] == "." {
        return get_possibilities(&elements[1..], sequences);
    }

    if elements[0] == "#" && elements.len() >= sequences[0] && elements.iter().position(|x| *x == ".").unwrap_or(elements.len()) >= sequences[0] {
        if elements.len() == sequences[0] && sequences.len() == 1 && elements[sequences[0] - 1] != "." {
            return 1;
        }

        if elements.len() == sequences[0] { return 0; }

        if elements[sequences[0]] != "#" {
            return get_possibilities(&elements[(sequences[0] + 1)..], &sequences[1..]);
        }
        return 0;
    }
    if elements[0] == "#" {
        return 0;
    }
    
    let mut total = 0;
    total += get_possibilities(&elements[1..], sequences);
    if elements.len() >= sequences[0] && elements.iter().position(|x| *x == ".").unwrap_or(elements.len()) >= sequences[0] {
        if elements.len() == sequences[0] && sequences.len() == 1 && elements[sequences[0] - 1] != "." {
            return 1;
        }

        if elements.len() == sequences[0] { return 0; }

        if elements[sequences[0]] != "#" {
            total += get_possibilities(&elements[(sequences[0] + 1)..], &sequences[1..]);
        }
    }

    total
}

impl Problem for Problem12 {
    fn solve() {
        let data = fileutils::load_problem_file(12);
        let mut total_arrangements = 0;

        for line in data.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0) {
            let sections: Vec<&str> = line.split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).collect();
            let elements: Vec<&str> = sections[0].split("").map(|x| x.trim()).filter(|x| x.len() > 0).collect();
            let sequences: Vec<usize> = sections[1].split(",").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<usize>().unwrap()).collect();

            let possibilities = get_possibilities(&elements, &sequences);
            total_arrangements += possibilities;

            println!("Possibilities: {possibilities}");
        }

        println!("Total possible arrangements: {total_arrangements}");
    }
}
