type Cost = u32;
type Position = u32;

const COST_OF_A:Cost = 3;
const COST_OF_B:Cost = 1;

struct Machine {
    // button A moves (X,Y)
    a:(Position, Position),
    b:(Position, Position),
    prize:(Position, Position)
}

//////////////////////////////////////////
/// Parsing
//////////////////////////////////////////

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/day13.pest"]
struct Day13Parser;

#[cfg(test)]
fn example1() -> &'static str {
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
}

#[test]
fn test_parse() {
    let parse1 = Day13Parser::parse(Rule::number, "42").unwrap().peek().unwrap();
    assert_eq!(parse1.as_rule(), Rule::number);
    assert_eq!(parse1.as_str(), "42");

    let input1 = 
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";

    assert!(Day13Parser::parse(Rule::machine, input1).is_ok());

    assert!(Day13Parser::parse(Rule::file, example1()).is_ok());

}

