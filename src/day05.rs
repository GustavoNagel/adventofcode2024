use std::collections::HashMap;

pub fn run(contents: String, part: &i8) {
    let mut counter = 0;
    let mut order_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut updates: Vec<Vec<&str>> = Vec::new();
    contents.lines().for_each(|line| {
        let v: Vec<&str> = line.split("|").collect();
        if v.len() == 2 {
            let mut rules = order_map.get(v[0]).cloned().unwrap_or(Vec::new());
            rules.push(v[1]);
            order_map.insert(v[0], rules);
        } else if v.len() == 1 && !v[0].is_empty() {
            updates.push(v[0].split(",").collect());
        }
    });
    println!("{:?}", order_map);
    println!("{:?}", updates);
    for update in updates {
        let mut is_valid = true;
        if part == &1 {
            for (i, page) in update.iter().enumerate() {
                if let Some(rules) = order_map.get(page) {
                    if !rules.iter().all(|higher_page| {
                       update.iter().position(|&p| p == *higher_page).unwrap_or(99999) > i
                    }) {
                        is_valid = false;
                        break;
                    }
                }
            }
            if is_valid {
                counter += update[(update.len() - 1)/2].parse::<i32>().unwrap();
            }
        } else if part == &2 {
            let mut old_update = update.clone();
            let mut fixed_updates: Vec<&str> = Vec::new();
            while let Some(page) = old_update.pop() {
                if let Some(rules) = order_map.get(page) {
                    if let Some(found_position) = fixed_updates.iter().position(|x| rules.contains(x)) {
                        fixed_updates.insert(found_position, page);
                    } else {
                        fixed_updates.push(page);
                    }
                } else {
                    fixed_updates.push(page);
                }
            }
            if fixed_updates != update {
                println!("Fixed{:?}", fixed_updates);
                counter += fixed_updates[(fixed_updates.len() - 1)/2].parse::<i32>().unwrap();
            }
        }
    }
    println!("Response: {}", counter);
}
