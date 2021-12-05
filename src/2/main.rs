#[derive(Debug)]
struct BasicPosition {
    x: i32,
    depth: i32,
}

impl From<&Vec<Instruction>> for BasicPosition {
    fn from(instructions: &Vec<Instruction>) -> Self {
        let mut initial = BasicPosition { x: 0, depth: 0 };
        for instruction in instructions {
            match instruction {
                Instruction::Forward(x) => initial.x += x,
                Instruction::Down(x) => initial.depth += x,
                Instruction::Up(x) => initial.depth -= x,
            }
        }
        initial
    }
}

#[derive(Debug)]
struct HeadingPosition {
    x: i32,
    depth: i32,

    aim: i32,
}

impl From<&Vec<Instruction>> for HeadingPosition {
    fn from(instructions: &Vec<Instruction>) -> Self {
        let mut initial = HeadingPosition {
            x: 0,
            depth: 0,
            aim: 0,
        };
        for instruction in instructions {
            match instruction {
                Instruction::Forward(x) => {
                    initial.x += x;
                    initial.depth += initial.aim * x;
                }
                Instruction::Down(x) => initial.aim += x,
                Instruction::Up(x) => initial.x -= x,
            }
        }

        initial

    }
}

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_line(line: &str) -> Instruction {
    let (instruction, value) = line.split_once(" ").expect("invalid instruction");
    let value = value.parse::<i32>().expect("invalid instruction");

    match instruction {
        "forward" => Instruction::Forward(value),
        "down" => Instruction::Down(value),
        "up" => Instruction::Up(value),
        _ => panic!("invalid instruction"),
    }
}

fn main() {
    let input = std::fs::read_to_string("src/2/input.txt").expect("failed to read input");
    let instructions = input.lines().map(parse_line).collect::<Vec<Instruction>>();

    let p1 = BasicPosition::from(&instructions);
    println!("Part 1: {:?} => {}", p1, p1.x * p1.depth);

    let p2 = HeadingPosition::from(&instructions);
    println!("Part 2: {:?} => {}", p2, p2.x * p2.depth);
}
