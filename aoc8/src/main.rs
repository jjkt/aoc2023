use num;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

// derive debug
#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(filename: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    // first line of contents:
    let first_line = contents.lines().next().unwrap();

    // parse line to vector. The line is series of L (left) and R (right) instructions
    // there is no separator between instructions.
    let instructions: Vec<Instruction> = first_line
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction"),
        })
        .collect();

    Ok(instructions)
}

fn parse_mappings(filename: &str) -> Result<HashMap<String, (String, String)>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    // next lines are a mapping table of
    // AAA = (BBB, CCC) lines. Parse them to a hashmap of key -> tuple
    let mut mapping = HashMap::new();
    for line in contents.lines().skip(2) {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();
        let mut value_parts = value.split(", ");
        let left = value_parts.next().unwrap();
        let right = value_parts.next().unwrap();
        // remove front "("
        let left = &left[1..];
        // remove back ")"
        let right = &right[..right.len() - 1];
        mapping.insert(
            key.to_string().to_owned(),
            (left.to_string().to_owned(), right.to_string().to_owned()),
        );
    }

    Ok(mapping)
}

fn calculate_steps_part1(filename: &str) -> Result<u32, Box<dyn Error>> {
    let instructions = parse_instructions(filename)?;

    // next lines are a mapping table of
    // AAA = (BBB, CCC) lines. Parse them to a hashmap of key -> tuple
    let mapping = parse_mappings(filename)?;

    let mut steps = 0;
    let mut current_key = "AAA";
    let mut index = 0;
    while current_key != "ZZZ" {
        let (left, right) = mapping.get(current_key).unwrap();
        let instruction = &instructions[index];
        match instruction {
            Instruction::Left => current_key = left,
            Instruction::Right => current_key = right,
        }
        steps += 1;
        index += 1;
        if index == instructions.len() {
            index = 0;
        }
    }

    Ok(steps)
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
    key: String,
    index: usize,
}

fn determine_steps_to_loop(
    start: &str,
    instructions: &[Instruction],
    mapping: &HashMap<String, (String, String)>,
) -> u64 {
    let mut steps = 0;
    let mut position: Position = Position {
        key: start.to_string(),
        index: 0,
    };

    let mut done = false;
    // loop until we reach the start position again OR we have taken 0 steps
    // OR we reach a position that we have seen before
    while !done {
        let (left, right) = mapping.get(&position.key).unwrap();
        let instruction = &instructions[position.index];
        match instruction {
            Instruction::Left => position.key = left.to_string(),
            Instruction::Right => position.key = right.to_string(),
        }
        steps += 1;
        position.index += 1;
        if position.index == instructions.len() {
            position.index = 0;
        }
        if position.key.ends_with("Z") {
            done = true;
        }
    }

    steps
}

fn calculate_steps_part2(filename: &str) -> Result<u64, Box<dyn Error>> {
    let instructions = parse_instructions(filename)?;
    let mapping = parse_mappings(filename)?;

    // determine starting positions. These are keys in the mapping hashmap
    // that end with a single 'A' letter:
    let mut current_positions = Vec::new();
    mapping.keys().for_each(|key| {
        if key.ends_with('A') {
            current_positions.push(key);
        }
    });

    let mut steps: Vec<u64> = Vec::new();

    for start_position in current_positions.iter() {
        let steps_to_loop = determine_steps_to_loop(&start_position, &instructions, &mapping);
        steps.push(steps_to_loop);
    }
    println!("Steps: {:?}", steps);

    // Calculate LCM of the steps:
    let mut lcm = steps[0];
    for i in 1..steps.len() {
        lcm = num::integer::lcm(lcm, steps[i]);
    }

    Ok(lcm)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1");
    let steps = calculate_steps_part1("input.txt")?;
    println!("Steps: {}", steps);

    let steps2 = calculate_steps_part1("input_full.txt")?;
    println!("Steps: {}", steps2);

    println!("Part 2");

    let steps3 = calculate_steps_part2("input_full.txt")?;
    println!("Steps: {}", steps3);

    Ok(())
}
