use std::fs;

pub fn file_in() -> String {

    return fs::read_to_string("static/values.txt")
        .expect("Error reading file.")
        .trim()
        .to_string();
}
