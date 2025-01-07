type Cost = u32;
type Position = i32;

const COST_OF_A:Cost = 3;
const COST_OF_B:Cost = 1;

#[derive(Debug, PartialEq)]
struct Machine {
    // button A moves (X,Y)
    a:(Position, Position),
    b:(Position, Position),
    prize:(Position, Position)
}

// solve equation m * a + n * b = prize
fn solve_equation(a:(Position, Position), b:(Position, Position), prize:(Position, Position)) -> (/*m: */Position, /*n: */ Position) {
    // m * a.0 + n * b.0 = prize.0
    // m * a.1 + n * b.1 = prize.1
    // ---------------------------
    // m * a.0 * a.1 + n * b.0 * a.1 = prize.0 * a.1
    // m * a.1 * a.0 + n * b.1 * a.0 = prize.1 * a.0
    // ---------------------------
    // n * b.0 * a.1 - n * b.1 * a.0 = prize.0 * a.1 - prize.1 * a.0
    // ---------------------------
    // n * ( b.0 * a.1 - b.1 * a.0 ) = prize.0 * a.1 - prize.1 * a.0
    // n = ( prize.0 * a.1 - prize.1 * a.0 ) / ( b.0 * a.1 - b.1 * a.0 )
    let n = ( prize.0 * a.1 - prize.1 * a.0 ) / ( b.0 * a.1 - b.1 * a.0 );
    // m * a.0 * b.1 + n * b.0 * b.1 = prize.0 * b.1
    // m * a.1 * b.0 + n * b.1 * b.0 = prize.1 * b.0
    // ---------------------------
    // m * a.0 * b.1 - m * a.1 * b.0 = prize.0 * b.1 - prize.1 * b.0
    // ---------------------------
    // m * ( a.0 * b.1 - a.1 * b.0 ) = prize.0 * b.1 - prize.1 * b.0
    // ---------------------------
    // m = ( prize.0 * b.1 - prize.1 * b.0 ) / ( a.0 * b.1 - a.1 * b.0 )
    let m = ( prize.0 * b.1 - prize.1 * b.0 ) / ( a.0 * b.1 - a.1 * b.0 );
    (m,n)
}

impl Machine {
    fn get_cost_to_win(&self) -> Option<Cost> {
        // push A m times, push B n times
        let (m,n) = solve_equation(self.a, self.b, self.prize);
        if m * self.a.0 + n * self.b.0 == self.prize.0 &&
           m * self.a.1 + n * self.b.1 == self.prize.1 {
               let cost:Cost = m as Cost *COST_OF_A + n as Cost *COST_OF_B;
               Some(cost)
        } else {
            None
        }
    }
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

    assert_eq!(solve_equation((94, 34), (22, 67), (8400, 5400)), (80, 40));
}

#[test]
fn test_machine() {
    let machines = build_file(Day13Parser::parse(Rule::file, example1()).unwrap().peek().unwrap());

    assert_eq!(solve_equation((94, 34), (22, 67), (8400, 5400)), (80, 40));
    assert_eq!(machines[0].get_cost_to_win(), Some(280));
    assert_eq!(solve_equation((26, 66), (67, 21), (12748, 12176)), (141, 135));
    assert_eq!(machines[1].get_cost_to_win(), None);
    assert_eq!(machines[2].get_cost_to_win(), Some(200));
    assert_eq!(machines[3].get_cost_to_win(), None);
    assert_eq!(machines.iter().map(|machine| machine.get_cost_to_win().unwrap_or(0)).sum::<Cost>(), 280+200);
}


//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day13.txt");
    let lines_concatenated = lines.join("\n");
    let machines = build_file(Day13Parser::parse(Rule::file, &lines_concatenated ).unwrap().peek().unwrap());

    let sum_of_cost = machines.iter().map(|machine| machine.get_cost_to_win().unwrap_or(0)).sum::<Cost>();
    println!("Day 13, Part 1: Sum of costs to win all prizes is {}", sum_of_cost);
}
