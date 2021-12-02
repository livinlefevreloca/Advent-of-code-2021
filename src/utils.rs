use std::fs;

pub fn read_line_delimited(file_name: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name).unwrap();

    contents
        .split("\n")
        .into_iter()
        .filter(|s| *s != "")
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>()
}

