type Cost = u32;
type Position = u32;

const COST_OF_A:Cost = 3;
const COST_OF_B:Cost = 1;

#[derive(Debug, PartialEq)]
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
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "../grammar/day13.pest"]
struct Day13Parser;


fn build_machine(machine_rule:Pair<'_, Rule>) -> Machine {
    let numbers:Vec<Position> = machine_rule.into_inner().map(|pair| pair.as_str().parse::<Position>().unwrap() ).collect();
    Machine {
        a:(numbers[0], numbers[1]),
        b:(numbers[2], numbers[3]),
        prize:(numbers[4],numbers[5])
    }
}

fn build_file(file_rule:Pair<'_, Rule>) -> Vec<Machine> {
    let mut machines = Vec::new();
    for pair in file_rule.into_inner() {
        match pair.as_rule() {
            Rule::machine => {
                let machine = build_machine(pair);
                machines.push(machine);
            }
            _ => { println!("Unexpected {}", pair); }
        }
    }
    machines
}


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
    let machine1 = build_machine(Day13Parser::parse(Rule::machine, input1).unwrap().peek().unwrap());
    assert_eq!(machine1, Machine{a:(94,34),b:(22,67),prize:(8400,5400)});

    assert!(Day13Parser::parse(Rule::file, example1()).is_ok());
    let machines = build_file(Day13Parser::parse(Rule::file, example1()).unwrap().peek().unwrap());
    assert_eq!(machines.len(), 4);

}

