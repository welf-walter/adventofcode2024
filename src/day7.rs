type Value = u64;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    PLUS,
    MULTIPLY,
    UNKNOWN
}

#[derive(Debug, PartialEq, Clone)]
struct Equation {
    result:Value,
    operands:Vec<Value>,
    operators:Vec<Operator>
}

type Equations = Vec<Equation>;

fn parse_equation(line:&str) -> Equation {
    let mut split_colon = line.split(": ");
    let result = split_colon.next().unwrap().parse::<Value>().unwrap();
    let operands_str = split_colon.next().unwrap();
    assert_eq!(split_colon.next(), None);
    let mut operands = Vec::new();
    let mut operators = Vec::new();
    let split_space = operands_str.split(' ');
    for operand_str in split_space {
        operands.push(operand_str.parse::<Value>().unwrap());
    }
    for _operator_index in 0..operands.len()-1 {
        operators.push(Operator::UNKNOWN);
    }
    Equation { result, operands, operators }
}

fn equation_is_true(eq:&Equation) -> bool {
    let mut operand_iter = eq.operands.iter();
    let mut operator_iter = eq.operators.iter();
    let mut myresult = *operand_iter.next().unwrap();
    loop {
        let operand_o = operand_iter.next();
        if operand_o.is_none() { return myresult == eq.result; }
        let operand = operand_o.unwrap();
        let operator = operator_iter.next().unwrap();
        myresult = match operator {
            Operator::PLUS => myresult + operand,
            Operator::MULTIPLY => myresult * operand,
            _ => unreachable!()
        }
    }
}


fn equation_can_be_made_true(eq:&Equation) -> bool {
    for i in 0..eq.operators.len() {
        if eq.operators[i] == Operator::UNKNOWN {
            let mut eq1 = eq.clone();
            eq1.operators[i] = Operator::PLUS;
            if equation_can_be_made_true(&eq1) { return true; }
            let mut eq2 = eq.clone();
            eq2.operators[i] = Operator::MULTIPLY;
            if equation_can_be_made_true(&eq2) { return true; }

            return false;
        }
    }
    // so there is no unknown operator
    return equation_is_true(&eq);
}

#[test]
fn test_equation() {
    let eq = parse_equation("3267: 81 40 27");
    assert_eq!(eq.result, 3267);
    assert_eq!(eq.operands, vec![81,40,27]);
    assert_eq!(eq.operators, vec![Operator::UNKNOWN, Operator::UNKNOWN]);

    let mut eq1 = eq.clone();
    eq1.operators = vec![Operator::PLUS, Operator::MULTIPLY];
    assert_eq!(equation_is_true(&eq1), true);
    let mut eq2 = eq.clone();
    eq2.operators = vec![Operator::PLUS, Operator::PLUS];
    assert_eq!(equation_is_true(&eq2), false);
    let mut eq3 = eq.clone();
    eq3.operators = vec![Operator::MULTIPLY, Operator::MULTIPLY];
    assert_eq!(equation_is_true(&eq3), false);
    let mut eq4 = eq.clone();
    eq4.operators = vec![Operator::MULTIPLY, Operator::PLUS];
    assert_eq!(equation_is_true(&eq4), true);

    assert_eq!(equation_can_be_made_true(&eq), true);

    let eq5 = parse_equation("161011: 16 10 13");
    assert_eq!(equation_can_be_made_true(&eq5), false);

}

fn count_equations_that_can_be_made_true(eqs:&Equations) -> usize {
    eqs.iter().map( |eq| if equation_can_be_made_true(eq) { 1 } else { 0 }).sum()
}

fn sum_equations_that_can_be_made_true(eqs:&Equations) -> Value {
    eqs.iter().map( |eq| if equation_can_be_made_true(eq) {  eq.result } else { 0 }).sum()
}

#[test]
fn test_equations() {
    let input =
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let eqs:Equations = input.split('\n').map(|line| parse_equation(line)).collect();
    assert_eq!(eqs.len(), 9);
    assert_eq!(equation_can_be_made_true(&eqs[0]), true);
    assert_eq!(equation_can_be_made_true(&eqs[1]), true);
    assert_eq!(equation_can_be_made_true(&eqs[2]), false);

    assert_eq!(eqs[1], parse_equation("3267: 81 40 27"));
    assert_eq!(count_equations_that_can_be_made_true(&eqs), 3);
    assert_eq!(sum_equations_that_can_be_made_true(&eqs), 3749);
}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


pub fn puzzle() {
    let file = File::open("input/day7.txt").expect("Could not open input/day7.txt");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map( |line| line.unwrap() ).collect();

    let eqs:Equations = lines.iter().map(|line| parse_equation(line)).collect();
    let count = count_equations_that_can_be_made_true(&eqs);
    let sum = sum_equations_that_can_be_made_true(&eqs);
    println!("Day 7, Part 1: {} of {} equations can be made true, their sum is {}", count, eqs.len(), sum);

}