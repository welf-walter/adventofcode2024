const VERBOSE:bool=false;
use regex::Regex;

type Design = String;
type Designs = Vec<String>;
type Towels = Vec<String>;

fn is_design_possible(design:&Design, towels:&Towels) -> bool {
    // ((r)|(wr)..)+
    let enclosed_towels = towels.iter().map(|towel| String::from("(")+towel+")").collect::<Vec<String>>();
    let regex_str = String::from("^(") + &enclosed_towels.join("|") + ")+$";
    if VERBOSE {println!("Does '{}' match '{}'?", regex_str, design);}
    let regex = Regex::new(&regex_str).unwrap();
    regex.is_match(&design)
}

fn read_input(input:Vec<String>) -> (Towels, Designs) {
    let mut iter = input.iter();
    let towels_line = iter.next().unwrap();
    let towels = towels_line.split(", ").map(|str| str.to_string()).collect::<Towels>();

    assert_eq!(iter.next().unwrap(), "");

    let mut designs = Designs::new();
    for line in iter {
        designs.push(line.clone());
    }

    (towels, designs)
}

#[cfg(test)]
fn input1() -> Vec<String> {
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb".split('\n').map(|str| str.to_string()).collect::<Vec<_>>()
}

#[test]
fn test_example1() {
    let (towels, designs) = read_input(input1());
    assert_eq!(towels.len(), 8);
    assert_eq!(&towels[4], "bwu");
    assert_eq!(designs.len(), 8);
    assert_eq!(&designs[4], "ubwu");

    assert_eq!(is_design_possible(&designs[0], &towels), true);
    assert_eq!(is_design_possible(&designs[1], &towels), true);
    assert_eq!(is_design_possible(&designs[2], &towels), true);
    assert_eq!(is_design_possible(&designs[3], &towels), true);
    assert_eq!(is_design_possible(&designs[4], &towels), false);
    assert_eq!(is_design_possible(&designs[5], &towels), true);
    assert_eq!(is_design_possible(&designs[6], &towels), true);
    assert_eq!(is_design_possible(&designs[7], &towels), false);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day19.txt");
    let (towels, designs) = read_input(lines);
    let design_count = designs.len();
    let possible_design_count = designs.iter().filter(|&design| is_design_possible(design, &towels)).count();

    println!("Day 19, Part 1: From {} designs, there are {} designs possible", design_count, possible_design_count);

}
