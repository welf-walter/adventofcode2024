type Value = u32;

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
fn test_parse() {
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