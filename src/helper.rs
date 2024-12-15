// Some helper functions

// split a string at empty lines
// often used to seperate parts of the input for advent of code puzzles
#[cfg(test)]
pub fn split_input_sections<const SECTION_COUNT:usize>(input:&str) -> [String;SECTION_COUNT] {
    let mut parts = input.split("\n\n");
    const EMPTY_STRING: std::string::String = String::new();
    let mut sections:[String;SECTION_COUNT] = [EMPTY_STRING;SECTION_COUNT];
    for i in 0..SECTION_COUNT {
        let part = parts.next().unwrap();
        sections[i] = String::from(part);
    }
    assert!(parts.next().is_none());
    sections
}

#[test]
fn test_split_input_sections() {
    let input =
"FOO
BAR
HOSE

Blumenkohl
Sahnetorte";
    let parts = split_input_sections::<2>(input);
    assert_eq!(parts[0], "FOO\nBAR\nHOSE".to_string());
    assert_eq!(parts[1], "Blumenkohl\nSahnetorte".to_string());
}

// split a list of strings at empty string
// often used to seperate parts of the input file for advent of code puzzles
pub fn split_lines_sections<const SECTION_COUNT:usize>(lines:Vec<String>) -> [Vec<String>;SECTION_COUNT] {
    let mut section = 0;
    const EMPTY_VECTOR:Vec<String> = Vec::new();
    let mut sections:[Vec<String>;SECTION_COUNT] = [EMPTY_VECTOR;SECTION_COUNT];
    for line in lines {
        if line == "" {
            section += 1;
        } else {
            sections[section].push(line);
        }
    }
    assert_eq!(section, SECTION_COUNT - 1);
    sections
}


use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file(filename:&str) -> Vec<String> {
    let file = File::open(filename).unwrap_or_else(|_| panic!("Could not open {}", filename));
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();
    lines
}