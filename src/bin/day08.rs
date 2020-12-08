use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_offset(offset: &str) -> i64 {
    let unsigned: i64 = offset[1..].parse().unwrap();
    if offset.chars().nth(0).unwrap() == '+' {
        unsigned
    } else {
        unsigned * -1
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split(' ');
    let instruction = parts.next().unwrap();
    let offset = parse_offset(parts.next().unwrap());

    match instruction {
        "acc" => Instruction::Acc(offset),
        "jmp" => Instruction::Jmp(offset),
        "nop" => Instruction::Nop(offset),
        _ => panic!("unknown instruction {:?}", instruction),
    }
}

#[derive(Debug)]
enum ExecuteResult {
    Terminate(i64),
    InfiniteLoop(i64),
}

fn execute(instructions: &[Instruction]) -> ExecuteResult {
    let mut visited_lines = HashSet::new();
    let mut instruction_pointer: usize = 0;
    let mut accumulator: i64 = 0;

    loop {
        if instruction_pointer == instructions.len() {
            return ExecuteResult::Terminate(accumulator);
        }
        let next_instruction = &instructions[instruction_pointer];

        let not_already_visited = visited_lines.insert(instruction_pointer);
        if !not_already_visited {
            return ExecuteResult::InfiniteLoop(accumulator);
        }

        match next_instruction {
            Instruction::Jmp(offset) => {
                instruction_pointer = (instruction_pointer as i64 + offset) as usize
            }
            Instruction::Acc(offset) => {
                accumulator += offset;
                instruction_pointer += 1
            }
            Instruction::Nop(_) => instruction_pointer += 1,
        }
    }
}

fn main() {
    let content = read_to_string("./inputs/day08.txt").expect("file not found");

    let instructions: Vec<Instruction> = content.lines().map(parse_instruction).collect();

    // Task 1
    // Result: 1394
    let result = execute(&instructions);
    println!("{:?}", result);

    let mut instructions = instructions;
    // Task 2
    for i in 0..instructions.len() {
        let mut revert_instruction = None;
        match instructions[i] {
            Instruction::Nop(offset) => {
                std::mem::replace(&mut instructions[i], Instruction::Jmp(offset));
                revert_instruction = Some(Instruction::Nop(offset));
            }
            Instruction::Jmp(offset) => {
                std::mem::replace(&mut instructions[i], Instruction::Nop(offset));
                revert_instruction = Some(Instruction::Jmp(offset));
            }
            Instruction::Acc(_) => continue,
        };
        if let ExecuteResult::Terminate(acc) = execute(&instructions) {
            println!("found termination!");
            // Result: 1626
            println!("{}", acc);
            break;
        }

        match revert_instruction {
            Some(instruction) => {
                std::mem::replace(&mut instructions[i], instruction);
            }
            _ => {}
        }
    }
}
