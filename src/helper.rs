// Some helper functions

// split a string at empty lines
// often used to seperate parts of the input for advent of code puzzles
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