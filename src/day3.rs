
//////////////////////////////////////////
/// Parser
//////////////////////////////////////////

const VERBOSE:bool = false;

type Multiplication = (u32, u32);
type Multiplications = Vec<Multiplication>;

struct Parser {
    multiplications:Multiplications
}

enum Instruction {
    MUL,
    DO,
    DONT,
    UNKNOWN
}

impl Parser {
    fn parse_line<const PUZZLE_PART:u32>(line:&str, multiplications:&mut Multiplications, mul_enabled:&mut bool) {
        let mut next_instruction = Instruction::UNKNOWN;

        if VERBOSE { println!("---"); }

        let parts = line.split("(");
        for part in parts {
            let this_instruction = next_instruction;

            next_instruction = Instruction::UNKNOWN;
            if part.ends_with("mul") {
                next_instruction = Instruction::MUL;
            }
            if PUZZLE_PART == 2 && part.ends_with("do") {
                next_instruction = Instruction::DO;
            }
            if PUZZLE_PART == 2 && part.ends_with("don't") {
                next_instruction = Instruction::DONT;
            }

            match this_instruction {
                Instruction::UNKNOWN => { continue; },
                Instruction::DO => { *mul_enabled = true; continue; },
                Instruction::DONT => { *mul_enabled = false; continue; },
                Instruction::MUL => { if ! *mul_enabled { continue; } }
            }

            if VERBOSE { print!("mul({}: ", &part); }

            let rbracket_split = part.split_once(")");
            if rbracket_split.is_none() {
                if VERBOSE { println!("no ')'"); }
                continue;
            }

            let Some((a_comma_b, _rbracket)) = rbracket_split else { panic!("{:?}", rbracket_split)};
            let comma_split = a_comma_b.split_once(",");
            if comma_split.is_none() {
                if VERBOSE { println!("no ','"); }
                continue;
            }

            let Some((a_str, b_str)) = comma_split else { panic!("{:?}", comma_split)};
            let a = match a_str.parse::<u32>() {
                Ok(a) => a,
                Err(_) => {
                    if VERBOSE { println!("{} is no number", a_str); }
                    continue;
                }
            };
            if a > 999 {
                if VERBOSE { println!("{} is too big", a); }
                continue;
            }

            let b = match b_str.parse::<u32>() {
                Ok(b) => b,
                Err(_) => {
                    if VERBOSE { println!("{} is no number", b_str); }
                    continue;
                }
            };
            if b > 999 {
                if VERBOSE { println!("{} is too big", b); }
                continue;
            }

            if VERBOSE { println!("ok"); }

            multiplications.push((a,b));
        }
    }

    fn parse<const PUZZLE_PART:u32>(lines:Vec<String>) -> Parser {
        let mut multiplications:Multiplications = Multiplications::new();
        let mut mul_enabled = true;
        for line in &lines {
            Self::parse_line::<PUZZLE_PART>(line, &mut multiplications, &mut mul_enabled);
        }
        Parser { multiplications:multiplications }
    }

    fn sum_of_multiplications(&self) -> u32 {
        self.multiplications.iter().map( |(a,b)| a * b).sum()
    }
}

#[test]
fn test_parser() {

    let parser1 = Parser::parse::<1>(vec!["limulbatrimul(22fimul(12,34)brmul(9999,12)eemul(999,12)".to_string()]);
    assert_eq!(parser1.multiplications, vec![(12,34), (999,12)]);
    assert_eq!(parser1.sum_of_multiplications(), 12*34 + 999*12);

    let parser2 = Parser::parse::<1>(vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()]);
    assert_eq!(parser2.multiplications, vec![(2,4),(5,5),(11,8),(8,5)]);
    assert_eq!(parser2.sum_of_multiplications(), 161);

    let parser3 = Parser::parse::<2>(vec!["limul(12,34)bladon't()4)brmul(99,12)".to_string()]);
    assert_eq!(parser3.multiplications, vec![(12,34)]);

    let parser4 = Parser::parse::<2>(vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()]);
    assert_eq!(parser4.multiplications, vec![(2,4),(8,5)]);
    assert_eq!(parser4.sum_of_multiplications(), 48);

    let parser5 = Parser::parse::<2>(vec!["liadon't()".to_string(), "4)brmul(99,12)".to_string()]);
    assert_eq!(parser5.multiplications, vec![]);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn puzzle() {
    let file = File::open("input/day3.txt").expect("Could not open input/day3.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();

    let parser1 = Parser::parse::<1>(lines.clone());
    let sum1 = parser1.sum_of_multiplications();
    println!("Day 3, Part 1: Sum of {} multiplications is {}", parser1.multiplications.len(), sum1);

    let parser2 = Parser::parse::<2>(lines);
    let sum2 = parser2.sum_of_multiplications();
    println!("Day 3, Part 2: Sum of {} multiplications is {}", parser2.multiplications.len(), sum2);

}