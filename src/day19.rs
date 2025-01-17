use std::collections::HashMap;

fn organize_towels(available_towel_patterns: Vec<&str>) -> HashMap<String, Vec<&str>> {
    let available_colors = vec!["r", "b", "g", "w", "u"];
    let mut organized_towels: HashMap<String, Vec<&str>> = HashMap::new();
    available_colors.iter().for_each(|&color| {
        organized_towels.insert(color.to_string(), available_towel_patterns.iter().filter(|x| x.starts_with(color)).map(|x| *x).collect::<Vec<&str>>());
    });
    organized_towels
}

fn recursive_search(part: &i8, organized_towels: HashMap<String, Vec<&str>>, current_design_line: String, historical_data: &mut HashMap<String, usize>) -> usize {
    if current_design_line.len() == 0 {
        return 1;
    }
    if let Some(&historical_response) = historical_data.get(&current_design_line) {
        return historical_response;
    }
    let mut design_line_iterator = current_design_line.chars();
    let design_line_first_char = design_line_iterator.next().unwrap();
    let possible_patterns = organized_towels.get(&design_line_first_char.to_string()).unwrap().clone().iter().filter(|&x| current_design_line.starts_with(x)).map(|x| *x).collect::<Vec<&str>>();
    // println!("Possible patterns: {:?}", possible_patterns);
    let mut counter = 0;
    for pattern in possible_patterns {
        let next_design_line = current_design_line.strip_prefix(pattern).unwrap().to_string();
        // println!("Next design line: {}", next_design_line);
        // println!("Pattern: {}", pattern);
        let recursive_response = recursive_search(part, organized_towels.clone(), next_design_line.clone(), historical_data);
        historical_data.insert(next_design_line, recursive_response);
        if recursive_response > 0 {
            if part == &1 {
                return 1;
            } else {
                counter += recursive_response;
            }
        }
    }
    counter
}

pub fn run(contents: String, part: &i8) {
    let mut counter_possible = 0;
    let mut lines_iterator = contents.lines();
    let first_line = lines_iterator.next().unwrap();
    let available_towel_patterns = first_line.split(", ").collect::<Vec<&str>>();
    let organized_towels = organize_towels(available_towel_patterns.clone());
    println!("Organized: {:?}", organized_towels);
    println!("Patterns: {:?}", available_towel_patterns);
    lines_iterator.next();
    while let Some(design_line) = lines_iterator.next() {
        println!("Design: {}", design_line);
        let mut historical_data: HashMap<String, usize> = HashMap::new();
        let counter = recursive_search(part, organized_towels.clone(), design_line.to_string(), &mut historical_data);
        if counter > 0 {
            println!("Possible! {}", counter);
            counter_possible += counter;
        }
    }
    println!("Response: {}", counter_possible);
}