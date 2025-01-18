type Register = u32;

#[derive(PartialEq, Debug)]
struct ComputerState {
    a:Register,
    b:Register,
    c:Register
}

#[derive(PartialEq, Debug)]
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

type Program = Vec<(Opcode,Operand)>;

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
    (ComputerState{a:register_a, b:register_b, c:register_c}, program)
}

#[cfg(test)]
fn input1() -> &'static str {
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
}

#[test]
fn test_example1() {
    let input = input1();
    let (state,program) = read_input(input.split('\n'));
    assert_eq!(state, ComputerState{a:729, b:0, c:0});
    assert_eq!(program, vec![(ADV, 1), (OUT, 4), (JNZ, 0)]);
}