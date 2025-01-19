const VERBOSE : bool = false;

type Register = u32;

#[derive(PartialEq, Debug, Clone)]
struct ComputerState {
    a:Register,
    b:Register,
    c:Register,
    ip:usize
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

use core::panic;

use Opcode::*;

impl Opcode {
    fn from_int(int:u32) -> Opcode {
        match int {
            0 => ADV,
            1 => BXL,
            2 => BST,
            3 => JNZ,
            4 => BXC,
            5 => OUT,
            6 => BDV,
            7 => CDV,
            other => panic!("unexpected opcode {}", other)
        }
    }
}

type Operand = u32;

const BASE2: u32 = 2;

impl ComputerState {
    fn execute_instruction(&mut self, opcode:Opcode, operand:Operand) -> Option<Register> {
        self.ip += 1;
        match opcode {
            ADV => { self.a = self.a / BASE2.pow(combo(operand, self)); }
            BXL => { self.b = self.b ^ operand; },
            BST => { self.b = combo(operand, self) % 8;  },
            JNZ => { if self.a > 0 { self.ip = (operand as usize) / 2 }},
            BXC => { self.b = self.b ^ self.c },
            OUT => { return Some(combo(operand, self) % 8) },
            BDV => { self.b = self.a / BASE2.pow(combo(operand, self)); },
            CDV => { self.c = self.a / BASE2.pow(combo(operand, self)); }
        }
        None
    }
}

#[test]
fn test_programs() {
    let mut state1 = ComputerState{a:0, b:0, c:9, ip:0};
    let program1 = program_from_str("2,6");
    let (opcode11, operand11) = program1[state1.ip];
    state1.execute_instruction(opcode11, operand11);
    assert_eq!(state1.b, 1);

    let state2 = ComputerState{a:10, b:0, c:0, ip:0};
    let program2 = program_from_str("5,0,5,1,5,4");
    let output2 = run_program(&program2, state2);
    assert_eq!(output2, vec![0,1,2]);

    let state3 = ComputerState{a:2024, b:0, c:0, ip:0};
    let program3 = program_from_str("0,1,5,4,3,0");
    let output3 = run_program(&program3, state3);
    assert_eq!(output3, vec![4,2,5,6,7,7,7,7,3,1,0]);

    let mut state4 = ComputerState{a:0, b:29, c:0, ip:0};
    let program4 = program_from_str("1,7");
    let (opcode41, operand41) = program4[state4.ip];
    state4.execute_instruction(opcode41, operand41);
    assert_eq!(state4.b, 26);

    let mut state5 = ComputerState{a:0, b:2024, c:43690, ip:0};
    let program5 = program_from_str("4,0");
    let (opcode51, operand51) = program5[state5.ip];
    state5.execute_instruction(opcode51, operand51);
    assert_eq!(state5.b, 44354);

}


type Program = Vec<(Opcode,Operand)>;


fn combo(operand: Operand, state:&ComputerState) -> Register {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        7 => panic!("Combo operand 7 is reserved and will not appear in valid programs"),
        other => panic!("{} is not a valid operand", other)
    }
}

type Output = Vec<Register>;

fn output_to_string(output:&Output) -> String {
    let output_string_vec = output.iter().map(|&i| i.to_string()).collect::<Vec<_>>();
    output_string_vec.join(",")
}

fn run_program(program:&Program, inital_state:ComputerState) -> Output {
    let mut state = inital_state.clone();
    let mut outputs:Output = Output::new();
    loop {
        if VERBOSE { println!("State: {:?}", state);}
        if state.ip >= program.len() {
            return outputs;
        }
        let (opcode, operand) = program[state.ip];
        if let Some(output) = state.execute_instruction(opcode, operand) {
            outputs.push(output);
        }
    }
}

fn run_program_check_output(program:&Program, inital_state:ComputerState, expected_output:Output) -> bool {
    let mut state = inital_state.clone();
    let mut output_iter = expected_output.iter();
    loop {
        if VERBOSE { println!("State: {:?}", state);}
        if state.ip >= program.len() {
            let is_all_expected_consumed = output_iter.next().is_none();
            return is_all_expected_consumed;
        }
        let (opcode, operand) = program[state.ip];
        if let Some(output) = state.execute_instruction(opcode, operand) {
            if let Some(&expected_output) = output_iter.next() {
                if expected_output != output {
                    return false;
                }
            }
        }
    }
}


fn program_from_vec(vec:Vec<Register>) -> Option<Program> {
    let mut j = vec.into_iter();
    let mut program:Program = Program::new();
    loop {
        if let Some(opcode) = j.next() {
            let operando = j.next();
            if operando.is_none() { return None; }
            let operand = operando.unwrap();
            let opcode = Opcode::from_int(opcode);
            program.push((opcode,operand));
        } else {
            return Some(program);
        }
    }
}

// str like "0,1,5,4,3,0"
fn program_from_str(str:&str) -> Program {
    let mut j = str.split(',');
    let mut program:Program = Program::new();
    loop {
        if let Some(opcode_str) = j.next() {
            let operand_str = j.next().unwrap();
            let opcode = Opcode::from_int(opcode_str.parse().unwrap());
            let operand = operand_str.parse().unwrap();
            program.push((opcode,operand));
        } else {
            return program;
        }
    }
}

fn read_register(line:&str) -> Register {
    // Register A: 729
    let mut i = line.split(' ');
    i.next().unwrap();
    i.next().unwrap();
    i.next().unwrap().parse().unwrap()
}

fn read_program(line:&str) -> Program {
    let mut i = line.split(' ');
    i.next().unwrap();
    program_from_str(i.next().unwrap())
}

fn read_input<'a>(lines:impl Iterator<Item=&'a str> + Clone) -> (ComputerState, Program) {
    let mut lines = lines.clone();
    let register_a = read_register(lines.next().unwrap());
    let register_b = read_register(lines.next().unwrap());
    let register_c = read_register(lines.next().unwrap());
    let newline = lines.next().unwrap(); assert!(newline.len() == 0);
    let program = read_program(lines.next().unwrap());
    (ComputerState{a:register_a, b:register_b, c:register_c, ip:0}, program)
}

fn is_program_cloning_itself(a:Register, program:&Program) -> bool {
    let state = ComputerState{a, b:0, c:0, ip:0};
    let output = run_program(&program, state);
    if VERBOSE {println!("Output: {:?}", output);}
    if let Some(generated_program) = program_from_vec(output) {
        if VERBOSE { println!("generated program: {:?}", program)}
        generated_program == *program
    } else {
        if VERBOSE { println!("could not generate program")}
        false
    }
}

fn find_first_cloning_a(program:&Program) -> Register {
    for a in 1.. {
        println!("Check start value for a = {}", a);
        if is_program_cloning_itself(a, program) {
            return a;
        }
    }
    unreachable!();
}

#[cfg(test)]
fn input1() -> &'static str {
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
}

#[cfg(test)]
fn input2() -> &'static str {
"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
}

#[test]
fn test_example1() {
    let input = input1();
    let (state,program) = read_input(input.split('\n'));
    assert_eq!(state, ComputerState{a:729, b:0, c:0, ip:0});
    assert_eq!(program, vec![(ADV, 1), (OUT, 4), (JNZ, 0)]);

    let output = run_program(&program, state.clone());
    if VERBOSE {println!("Output: {:?}", output);}
    assert_eq!(output, vec![4,6,3,5,6,3,5,2,1,0]);
    assert_eq!(output_to_string(&output), "4,6,3,5,6,3,5,2,1,0");

    assert!(run_program_check_output(&program, state, vec![4,6,3,5,6,3,5,2,1,0]));
}

#[test]
fn test_example2() {

    let input = input2();
    let (_state,program) = read_input(input.split('\n'));

    assert!(is_program_cloning_itself(117440, &program));

    let state2 = ComputerState{a:117440, b:0, c:0, ip:0};
    assert!(run_program_check_output(&program, state2, vec![0,3,5,4,3,0]));

    let a = find_first_cloning_a(&program);
    assert_eq!(a, 117440);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day17.txt");
    let (initial_state,program) = read_input(lines.iter().map(|line| line.as_str()));

    if VERBOSE {println!("Day 16, Debug program = {:?}", program);}
    let output1 = run_program(&program, initial_state.clone());
    println!("Day 16, Part 1: Output of program is {:?}", output_to_string(&output1));

    let a = find_first_cloning_a(&program);
    println!("Day 16, Part 2: Register A value {} leads to cloning", a);
}
