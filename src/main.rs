use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
enum Operator {
    IncDataPointer,
    DecDataPointer,
    IncValue,
    DecValue,
    Output,
    Input,
    Loop(Vec<Operator>),
}

fn interpret(input: String) -> Vec<Operator> {
    let mut instructions = Vec::new();
    let mut jump_stack: Vec<usize> = Vec::new();

    for c in input.chars() {
        match c {
            '>' => instructions.push(Operator::IncDataPointer),
            '<' => instructions.push(Operator::DecDataPointer),
            '+' => instructions.push(Operator::IncValue),
            '-' => instructions.push(Operator::DecValue),
            '.' => instructions.push(Operator::Output),
            ',' => instructions.push(Operator::Input),
            '[' => {
                jump_stack.push(instructions.len());
            }
            ']' => {
                let jump_pointer = jump_stack.pop().expect("Compilation error");
                let loop_instructions = instructions[jump_pointer..].to_vec();
                instructions.push(Operator::Loop(loop_instructions));
            }
            _ => (), // comments are ignored
        }
    }

    return instructions;
}

fn execute(tape: &mut [u8; 1024], data_pointer: &mut usize, operators: Vec<Operator>) {
    for operator in operators.iter() {
        match operator {
            Operator::IncDataPointer => *data_pointer += 1,
            Operator::DecDataPointer => *data_pointer -= 1,
            Operator::IncValue => tape[*data_pointer] += 1,
            Operator::DecValue => tape[*data_pointer] -= 1,
            Operator::Output => {
                print!("{}", tape[*data_pointer] as char);
            }
            Operator::Input => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                tape[*data_pointer] = input[0];
            }
            Operator::Loop(loop_operators) => {
                while tape[*data_pointer] != 0 {
                    execute(tape, data_pointer, loop_operators.to_vec());
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // Read file
    let mut file = File::open(filename).expect("program file not found");
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read program file");

    let instructions = interpret(source);

    let mut tape = [0; 1024];
    let mut data_pointer = 124;
    execute(&mut tape, &mut data_pointer, instructions);
}
