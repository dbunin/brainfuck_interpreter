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
    JumpForward,
    JumpBackwards,
}

// TODO Change to enum, with operand only being there for jump back?
#[derive(Debug, Clone)]
struct Instruction(Operator, u32);

fn interpret(input: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut pointer = 0;
    let mut jump_stack: Vec<u32> = Vec::new();

    for c in input.chars() {
        match c {
            '>' => instructions.push(Instruction(Operator::IncDataPointer, 0)),
            '<' => instructions.push(Instruction(Operator::DecDataPointer, 0)),
            '+' => instructions.push(Instruction(Operator::IncValue, 0)),
            '-' => instructions.push(Instruction(Operator::DecValue, 0)),
            '.' => instructions.push(Instruction(Operator::Output, 0)),
            ',' => instructions.push(Instruction(Operator::Input, 0)),
            '[' => {
                instructions.push(Instruction(Operator::JumpForward, 0));
                jump_stack.push(pointer)
            }
            ']' => {
                if jump_stack.len() == 0 {
                    panic!("Compilation error")
                }
                let jump_pointer = jump_stack[jump_stack.len() - 1];
                jump_stack.remove(jump_stack.len() - 1);
                instructions.push(Instruction(Operator::JumpBackwards, jump_pointer));
                let Instruction(operator, _) = &instructions[jump_pointer as usize];
                instructions[jump_pointer as usize] = Instruction(operator.clone(), pointer.clone())
            }
            _ => pointer = pointer - 1, // ignore comments
        }
        pointer = pointer + 1
    }

    return instructions;
}

fn execute(instructions: Vec<Instruction>) {
    let mut tape: [i16; 66666] = [0; 66666];
    let mut data_pointer = 0;

    let mut pointer = 0;
    while pointer < instructions.len() {
        let instruction = &instructions[pointer];

        match instruction {
            Instruction(Operator::IncDataPointer, _) => {
                data_pointer = data_pointer + 1;
            }
            Instruction(Operator::DecDataPointer, _) => {
                data_pointer = data_pointer - 1;
            }
            Instruction(Operator::IncValue, _) => {
                let value = tape[data_pointer];
                tape[data_pointer] = value + 1;
            }
            Instruction(Operator::DecValue, _) => {
                let value = tape[data_pointer];
                tape[data_pointer] = value - 1;
            }
            Instruction(Operator::Output, _) => {
                let character = char::from_u32(tape[data_pointer] as u32);
                match character {
                    Some(c) => print!("{}", c),
                    None => panic!("Unexpected u32 {}", tape[data_pointer]),
                }
            }
            Instruction(Operator::Input, _) => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                tape[data_pointer] = input[0] as i16;
            }
            Instruction(Operator::JumpForward, operand) => {
                if tape[data_pointer] == 0 {
                    pointer = *operand as usize;
                }
            }
            Instruction(Operator::JumpBackwards, operand) => {
                if tape[data_pointer] > 0 {
                    pointer = *operand as usize;
                }
            }
        }
        pointer = pointer + 1;
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

    execute(instructions);
}
