use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn compute_gate_operations(mut wire_values: HashMap<String, u8>, gates: Vec<(String, &str, &str, &str)>) -> HashMap<String, u8> {
    let mut pending_operations = gates.clone();
    while !pending_operations.is_empty() {
        let mut next_pending_operations = vec![];
        while let Some(pending_operation) = pending_operations.pop() {
            let (gate_type, gate_input1, gate_input2, gate_output) = pending_operation;
            if !wire_values.contains_key(gate_input1) || !wire_values.contains_key(gate_input2) {
                next_pending_operations.push((gate_type.to_string(), gate_input1, gate_input2, gate_output));
            } else {
                let input1 = wire_values.get(gate_input1).unwrap();
                let input2 = wire_values.get(gate_input2).unwrap();
                let output = match gate_type.as_str() {
                    "and" => *input1 & *input2,
                    "or" => *input1 | *input2,
                    "xor" => *input1 ^ *input2,
                    _ => panic!("Gate type not implemented")
                };
                wire_values.insert(gate_output.to_string(), output);
            }
        }
        pending_operations = next_pending_operations;
    }
    wire_values
}

pub fn run(contents: String, part: &i8) {
    let mut wire_values = HashMap::new();
    let wire_values_re = Regex::new(r"(?<name>\w[\d]{2})\: (?<value>\d*)").unwrap();
    let gates_re = Regex::new(r"(?<first>[\w\d]{3}) (?<gate>[AND|OR|XOR]*) (?<second>[\w\d]{3}) -> (?<output>[\w\d]{3})").unwrap();
    let mut lines_iterator = contents.lines();
    loop {
        let line = lines_iterator.next().unwrap();
        if line == "" {
            break;
        }
        let caps = wire_values_re.captures(line).unwrap();
        let wire_name = caps.name("name").unwrap().as_str();
        let wire_value: u8 = caps.name("value").unwrap().as_str().parse().unwrap();
        wire_values.insert(wire_name.to_string(), wire_value);
    }
    let gates = lines_iterator.map(|line| {
        let caps = gates_re.captures(line).unwrap();
        let gate_type = caps.name("gate").unwrap().as_str().to_lowercase();
        let gate_input1 = caps.name("first").unwrap().as_str();
        let gate_input2 = caps.name("second").unwrap().as_str();
        let gate_output = caps.name("output").unwrap().as_str();
        (gate_type, gate_input1, gate_input2, gate_output)
    }).collect::<Vec<(String, &str, &str, &str)>>();
    println!("Wire Values: {:?}", wire_values);
    println!("Gates Operations: {:?}", gates);

    if part == &1 {
        wire_values = compute_gate_operations(wire_values, gates);

        let binary_values = wire_values.iter().filter(|(k, _)| k.starts_with("z")).map(|(k, v)| {
            (k[1..].parse::<isize>().unwrap(), v)
        }
        ).sorted_by_key(|(k, _)| (*k) * (-1)).map(|(_, v)| *v as i64).collect::<Vec<_>>();
        println!("Response Binary: {:?}", binary_values);
        let decimal_number = binary_values.iter().fold(0, |acc, x| acc * 2 + x);
        println!("Response Decimal: {}", decimal_number);
    } else if part == &2 {
        // TODO: find pairs to swap
        let wire_pairs = vec![vec!["aaa","eee"],vec!["ooo","z99"],vec!["bbb","ccc"],vec!["aoc","z24"]];

        println!("Response: {:?}", wire_pairs.into_iter().flatten().sorted().join(","));
    }
}
