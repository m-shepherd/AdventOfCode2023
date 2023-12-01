use std::fs;

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Unable to read file")
}

pub fn load_problem_file(problem_number: i32) -> String {
    let problem_file = format!("resources/problem{problem_number}.txt");
    read_file(&problem_file)
}
