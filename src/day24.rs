use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use graph_builder::prelude::*;

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
        let mut graph_edges = vec![];
        let mut numeric_graph_edges = vec![];
        let mut nodes = vec![];
        gates.iter().for_each(|(_gate_type, gate_input1, gate_input2, gate_output)| {
            if !nodes.contains(&gate_input1) {
                nodes.push(gate_input1);
            }
            if !nodes.contains(&gate_input2) {
                nodes.push(gate_input2);
            }
            if !nodes.contains(&gate_output) {
                nodes.push(gate_output);
            }
            let output_index = nodes.iter().position(|&x| x == gate_output).unwrap() as i32;
            let input1_index = nodes.iter().position(|&x| x == gate_input1).unwrap() as i32;
            let input2_index = nodes.iter().position(|&x| x == gate_input2).unwrap() as i32;
            let (ix, color) = match _gate_type.as_str() {
                "and" => (1, "red"),
                "or" => (2, "blue"),
                "xor" => (3, "green"),
                _ => (4, "black")
            };
            let input1_name = format!("{}_{}", gate_input1, input1_index);
            let input2_name = format!("{}_{}", gate_input2, input2_index);
            let output_name = format!("{}_{}", gate_output, output_index);
            numeric_graph_edges.push((input1_index, output_index, ix));
            numeric_graph_edges.push((input2_index, output_index, ix));
            graph_edges.push((input1_name, output_name.clone(), color));
            graph_edges.push((input2_name, output_name, color));
        });
        let graph: DirectedCsrGraph<i32, (), i32> = GraphBuilder::new()
            .csr_layout(CsrLayout::Sorted)
            .edges_with_values(numeric_graph_edges)
            .build();

        println!("The following data can be used to build and visualize the graph:");
        println!("graph G {{");
        graph_edges.iter().for_each(|(input, output, color)| {
            println!("  {} -- {} [color={}]", input, output, color);
        });
        println!("}}");
        let mut patterns: HashMap<String, Vec<&str>> = HashMap::new();
        gates.iter().for_each(|(_, _, _, gate_output)| {
            let output_index = nodes.iter().position(|&x| x == gate_output).unwrap() as i32;
            let mut outgoing: Vec<i32> = graph.out_neighbors_with_values(output_index).map(|t| t.value).sorted().collect();
            let mut ingoing: Vec<i32> = graph.in_neighbors_with_values(output_index).map(|t| t.value).sorted().collect();
            let sub_outgoing: Vec<i32> = graph.out_neighbors_with_values(output_index).map(|t| graph.out_neighbors_with_values(t.target).map(|t| t.value).collect::<Vec<i32>>()).flatten().sorted().collect();
            let sub_ingoing: Vec<i32> = graph.in_neighbors_with_values(output_index).map(|t| graph.in_neighbors_with_values(t.target).map(|t| t.value).collect::<Vec<i32>>()).flatten().sorted().collect();

            outgoing.extend(sub_outgoing);
            ingoing.extend(sub_ingoing);
            println!("{} -> outgoing: {:?} -> ingoing: {:?}", gate_output, outgoing, ingoing);
            if patterns.contains_key(&format!("o{}i{}", outgoing.iter().join(","), ingoing.iter().join(","))) {
                let current = patterns.get_mut(&format!("o{}i{}", outgoing.iter().join(","), ingoing.iter().join(","))).unwrap();
                current.push(gate_output);
            } else {
                patterns.insert(format!("o{}i{}", outgoing.iter().join(","), ingoing.iter().join(",")), vec![gate_output]);
            }
        });
        patterns.iter().for_each(|(k, v)| {
            if v.len() < 10 {
                println!("Not usual Pattern: {} -> {:?}", k, v);
            }
        });

        // Response found from visual analysis: 118 e 41, 251 e 209, 151 e 168, 170 e 235
        let wire_pairs = vec![vec!["qnw","z15"],vec!["cqr","z20"],vec!["ncd","nfj"],vec!["vkg","z37"]];

        println!("Response: {:?}", wire_pairs.into_iter().flatten().sorted().join(","));
    }
}
