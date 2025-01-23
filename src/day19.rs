const VERBOSE:bool=false;
use regex::Regex;

use crate::optimize::{ActionTrait, Problem};

type Design = String;
type Designs = Vec<String>;
type Towels = Vec<String>;

struct DesignChecker {
    _towels:Towels,
    regex:Regex
}

impl DesignChecker {

    fn new(towels:Towels) -> DesignChecker {
        let regex = Self::create_regex(&towels);
        DesignChecker {
            _towels: towels,
            regex
        }
    }

    fn create_regex(towels:&Towels) -> Regex {
        // ((r)|(wr)..)+
        let enclosed_towels = towels.iter().map(|towel| String::from("(")+towel+")").collect::<Vec<String>>();
        let regex_str = String::from("^(") + &enclosed_towels.join("|") + ")+$";
        if VERBOSE {println!("RegEx = '{}'", regex_str);}
        Regex::new(&regex_str).unwrap()
    }

    fn is_design_possible(&self, design:&Design) -> bool {
        if VERBOSE {println!("Is '{}' possible?", design);}
        self.regex.is_match(&design)
    }
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

    let checker = DesignChecker::new(towels);

    assert_eq!(checker.is_design_possible(&designs[0]), true);
    assert_eq!(checker.is_design_possible(&designs[1]), true);
    assert_eq!(checker.is_design_possible(&designs[2]), true);
    assert_eq!(checker.is_design_possible(&designs[3]), true);
    assert_eq!(checker.is_design_possible(&designs[4]), false);
    assert_eq!(checker.is_design_possible(&designs[5]), true);
    assert_eq!(checker.is_design_possible(&designs[6]), true);
    assert_eq!(checker.is_design_possible(&designs[7]), false);
}

//////////////////////////////////////////
/// Part 2
//////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct MatchState {
    //design:String,
    // number of already matched characters
    matched:usize
}

struct DesignProblem {
    design:String
}

impl Problem for DesignProblem {
    type State = MatchState;
    type Action = String;

    fn is_end_state(&self, state:&Self::State) -> bool {
        self.design.len() == state.matched
    }

    fn execute_action(&self, before:Self::State, action:Self::Action) -> Option<Self::State> {
        
    }

}

impl ActionTrait for String {
    fn all_actions() -> &'static [Self] {
        &["r".to_string(), "wr".to_string(), "b".to_string(), "g".to_string(), "bwu".to_string(), "rb".to_string(), "gb".to_string(), "br".to_string()]
    }

    fn cost(self) -> crate::optimize::Cost {
        1
    }
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day19.txt");
    let (towels, designs) = read_input(lines);

    let checker = DesignChecker::new(towels);
    let design_count = designs.len();
    let possible_design_count = designs.iter().filter(|&design| checker.is_design_possible(design)).count();

    println!("Day 19, Part 1: From {} designs, there are {} designs possible", design_count, possible_design_count);

}
