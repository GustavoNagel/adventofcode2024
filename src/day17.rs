use std::collections::HashMap;
use regex::Regex;

fn get_combo_operand(computer_registers: &HashMap<String, i64>, operand: u8) -> i64 {
    if operand <= 3 {
        return operand as i64;
    } else if operand == 7 {
        panic!("Operand 7 not implemented");
    } else {
        return computer_registers.get(&format!("{}", (61 + operand) as u8 as char)).unwrap().clone();
    }
}

// Perform division
fn adv(computer_registers: &mut HashMap<String, i64>, operand: u8) {
    let numerator = *computer_registers.get("A").unwrap() as f64;
    let denominator = 2.0_f64.powf(get_combo_operand(&computer_registers, operand) as f64);
    let result = numerator / denominator;
    // println!("Numerator {} , Denominator {} , Result: {}", numerator, denominator, result);
    computer_registers.insert("A".to_string(), result as i64);
}

// Perform BitXor
fn bxl(computer_registers: &mut HashMap<String, i64>, operand: u8) {
    *computer_registers.get_mut("B").unwrap() ^= operand as i64;
}

// Perform Modulo
fn bst(computer_registers: &mut HashMap<String, i64>, operand: u8) {
    computer_registers.insert("B".to_string(), get_combo_operand(&computer_registers, operand) % 8);
}

// Perform Jump if A is not zero
fn jnz(computer_registers: &mut HashMap<String, i64>, operand: u8) -> Option<u8> {
    if *computer_registers.get("A").unwrap() != 0 {
        return Some(operand);
    }
    return None;
}

// Perform BitXor
fn bxc(computer_registers: &mut HashMap<String, i64>, _operand: u8) {
    *computer_registers.get_mut("B").unwrap() ^= *computer_registers.get("C").unwrap() as i64;
}

fn out(computer_registers: &mut HashMap<String, i64>, operand: u8) -> i64{
    let output = get_combo_operand(&computer_registers, operand) % 8;
    output
}

fn bdv(computer_registers: &mut HashMap<String, i64>, operand: u8) {
    let numerator = *computer_registers.get("A").unwrap() as f64;
    let denominator = 2.0_f64.powf(get_combo_operand(&computer_registers, operand) as f64);
    let result = numerator / denominator;
    computer_registers.insert("B".to_string(), result as i64);
}

fn cdv(computer_registers: &mut HashMap<String, i64>, operand: u8) {
    let numerator = *computer_registers.get("A").unwrap() as f64;
    let denominator = 2.0_f64.powf(get_combo_operand(&computer_registers, operand) as f64);
    let result = numerator / denominator;
    computer_registers.insert("C".to_string(), result as i64);
}

fn evaluate_program(computer_registers: &mut HashMap<String, i64>, program_instructions: Vec<u8>) -> Vec<String> {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < program_instructions.len() {
        let instruction = program_instructions[instruction_pointer];
        let operand = program_instructions[instruction_pointer + 1];
        // println!("Instruction: {}, Operand: {}", instruction, operand);
        match instruction {
            0 => adv(computer_registers, operand),
            1 => bxl(computer_registers, operand),
            2 => bst(computer_registers, operand),
            3 => {
                if let Some(new_pointer) = jnz(computer_registers, operand) {
                    instruction_pointer = new_pointer as usize;
                    continue;
                }
            },
            4 => bxc(computer_registers, operand),
            5 => {
                output.push(out(computer_registers, operand).to_string());
            },
            6 => bdv(computer_registers, operand),
            7 => cdv(computer_registers, operand),
            _ => panic!("Instruction not implemented"),
        }
        instruction_pointer += 2;
    }
    output
}

pub fn run(contents: String, part: &i8) {
    let mut computer_registers = HashMap::new();
    let re_register = Regex::new(r"Register (?<name>[ABC]{1})\: (?<value>\d*)").unwrap();
    let re_program = Regex::new(r"Program: (?<instructions>[\d\,]*)").unwrap();
    let mut lines_iterator = contents.lines();
    loop {
        let line = lines_iterator.next().unwrap();
        if line == "" {
            break;
        }
        let caps = re_register.captures(line).unwrap();
        let register_name = caps.name("name").unwrap().as_str();
        let register_value: i64 = caps.name("value").unwrap().as_str().parse().unwrap();
        computer_registers.insert(register_name.to_string(), register_value);
    }
    let program_line = lines_iterator.next().unwrap();
    let program_instructions: Vec<u8> = re_program.captures(program_line).unwrap().name("instructions").unwrap().as_str().split(",").map(|s| s.parse().unwrap()).collect();
    println!("Computer Registers: {:?}", computer_registers);
    println!("Program Instructions: {:?}", program_instructions);
    if part == &1 {
        let output = evaluate_program(&mut computer_registers, program_instructions);
        println!("Computer Registers: {:?}", computer_registers);
        println!("Output: {}", output.join(","));
    } else if part == &2 {
        let mut computer_registers_clone = computer_registers.clone();
        let mut reversed_instructions = program_instructions.clone();
        reversed_instructions.reverse();
        let mut diffs = Vec::new();
        diffs.push(0);
        'outer: for j in 0..reversed_instructions.len() {
            let mut next_diffs = Vec::new();
            for diff in diffs.into_iter() {
                let start = 8i64.pow(j as u32);
                let reference = start + (diff * 8);
                for i in 0..8 {
                    computer_registers_clone.insert("A".to_string(), reference + i);
                    let output = evaluate_program(&mut computer_registers_clone, program_instructions.clone());
                    // println!("i {}, ref + i {}, Output: {}", i, reference + i , output.join(","));
                    if reversed_instructions[j] == output[output.len() - (j + 1) as usize].parse().unwrap() {
                        next_diffs.push((reference + i) - start);
                        if j == reversed_instructions.len() - 1 {
                            print!("Response: {}", reference + i);
                            break 'outer;
                        }
                    }
                }
            }
            diffs = next_diffs;
        }
    }
}

// Add test to bxl
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() {
        let mut computer_registers = HashMap::new();
        computer_registers.insert("A".to_string(), 729);
        adv(&mut computer_registers, 3);
        assert_eq!(computer_registers.get("A").unwrap(), &91);
    }

    #[test]
    fn test_bxl() {
        let mut computer_registers = HashMap::new();
        computer_registers.insert("B".to_string(), 29);
        bxl(&mut computer_registers, 7);
        assert_eq!(computer_registers.get("B").unwrap(), &26);
    }

    #[test]
    fn test_bst() {
        let mut computer_registers = HashMap::new();
        computer_registers.insert("C".to_string(), 9);
        bst(&mut computer_registers, 6);
        assert_eq!(computer_registers.get("B").unwrap(), &1);
    }

    #[test]
    fn test_bdv() {
        let mut computer_registers = HashMap::new();
        computer_registers.insert("A".to_string(), 729);
        computer_registers.insert("B".to_string(), 0);
        bdv(&mut computer_registers, 3);
        assert_eq!(computer_registers.get("B").unwrap(), &91);
    }

    #[test]
    fn test_cdv() {
        let mut computer_registers = HashMap::new();
        computer_registers.insert("A".to_string(), 729);
        computer_registers.insert("C".to_string(), 0);
        cdv(&mut computer_registers, 3);
        assert_eq!(computer_registers.get("C").unwrap(), &91);
    }

    #[test]
    fn test_use_case_1() {
        let mut computer_registers = HashMap::from([
            ("A".to_string(), 10),
        ]);
        let program_instructions = vec![5,0,5,1,5,4];
        let output = evaluate_program(&mut computer_registers, program_instructions);
        assert_eq!(output.join(","), "0,1,2".to_string());
    }

    #[test]
    fn test_use_case_2() {
        let mut computer_registers = HashMap::from([
            ("A".to_string(), 2024),
        ]);
        let program_instructions = vec![0,1,5,4,3,0];
        let output = evaluate_program(&mut computer_registers, program_instructions);
        assert_eq!(output.join(","), "4,2,5,6,7,7,7,7,3,1,0".to_string());
    }

    #[test]
    fn test_use_case_3() {
        let mut computer_registers = HashMap::from([
            ("A".to_string(), 729),
            ("B".to_string(), 0),
            ("C".to_string(), 0),
        ]);
        let program_instructions = vec![0,1,5,4,3,0];
        let output = evaluate_program(&mut computer_registers, program_instructions);
        assert_eq!(output.join(","), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}