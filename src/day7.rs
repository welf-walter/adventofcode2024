type Value = u32;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    PLUS,
    MULTIPLY,
    UNKNOWN
}

struct Equation {
    result:Value,
    operands:Vec<Value>,
    operators:Vec<Operator>
}

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
    for operator_index in 0..operands.len()-1 {
        operators.push(Operator::UNKNOWN);
    }
    Equation { result, operands, operators }
}

#[test]
fn test_parse() {
    let eq = parse_equation("3267: 81 40 27");
    assert_eq!(eq.result, 3267);
    assert_eq!(eq.operands, vec![81,40,27]);
    assert_eq!(eq.operators, vec![Operator::UNKNOWN, Operator::UNKNOWN]);
}