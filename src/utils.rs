use std::fs;

// Read a line delimited file stripping away surrounding whitespace
// into a vector of Strings
//
//  Args: 
//      file_name: The name of a file read
//  Returns:
//      A vector of Strings, one for each line of the file
pub fn read_line_delimited(file_name: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name).unwrap();

    contents
        .split("\n")
        .into_iter()
        .filter(|s| *s != "")
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>()
}

