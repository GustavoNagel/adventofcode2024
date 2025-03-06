use std::collections::HashMap;
use itertools::Itertools;
use phf::phf_ordered_map;

static INSTRUCTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "U" => (-1, 0),
    "D" => (1, 0),
    "R" => (0, 1),
    "L" => (0, -1),
};

fn get_next_position(previous_position: (usize, usize), current_position: (usize, usize), racetrack_map: &HashMap<usize, HashMap<usize, String>>, obstacle: &String) -> (usize, usize) {
    INSTRUCTIONS_MAP.values().find_map(|(i, j)| {
        let next_i = current_position.0 as isize + i;
        let next_j = current_position.1 as isize + j;
        if next_i >= 0 && next_j >= 0 && previous_position != (next_i as usize, next_j as usize) {
            if let Some(row_map) = racetrack_map.get(&(next_i as usize)) {
                if let Some(position_value) = row_map.get(&(next_j as usize)) {
                    if position_value != obstacle {
                        return Some((next_i as usize, next_j as usize));
                    }
                }
            } 
        }
        None
    }).unwrap()
}

fn get_cheats(racetrack_costs: &HashMap<(usize, usize), isize>) -> Vec<(usize, usize, isize)> {
    let mut cheats = vec![];
    racetrack_costs.iter().for_each(|(&current_position, &current_cost)| {
        INSTRUCTIONS_MAP.values().for_each(|(i, j)| {
            let next_i = current_position.0 as isize + i;
            let next_j = current_position.1 as isize + j;
            if next_i >= 0 && next_j >= 0 && racetrack_costs.get(&(next_i as usize, next_j as usize)).is_none() {
                INSTRUCTIONS_MAP.values().for_each(|(i2, j2)| {
                    let next_i2 = next_i + i2;
                    let next_j2 = next_j + j2;
                    if next_i2 >= 0 && next_j2 >= 0 && (next_i2 as usize, next_j2 as usize) != current_position {
                        if let Some(&cost_without_cheat) = racetrack_costs.get(&(next_i2 as usize, next_j2 as usize)) {
                            if cost_without_cheat > (current_cost + 2) {
                                cheats.push((next_i2 as usize, next_j2 as usize, cost_without_cheat - (current_cost + 2)));
                            }
                        }
                    }
                })
            }
        });
    });
    cheats
}

fn get_cheats_v2(racetrack_costs: &HashMap<(usize, usize), isize>, cheat_max_duration: isize) -> Vec<(usize, usize, isize)> {
    let mut cheats: Vec<(usize, usize, isize)> = vec![];
    racetrack_costs.iter().for_each(|(&current_position, &current_cost)| {
        racetrack_costs.iter().for_each(|(&target_position, &target_previous_cost)| {
            let position_abs_diff = (current_position.0 as isize - target_position.0 as isize).abs() + (current_position.1 as isize - target_position.1 as isize).abs();
            if position_abs_diff <= cheat_max_duration {
                let cost_saved = target_previous_cost - (current_cost + position_abs_diff);
                if cost_saved > 0 {
                    cheats.push((target_position.0, target_position.1, cost_saved));
                }
            }
        });
    });
    cheats
}

pub fn run(contents: String, part: &i8) {
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    let mut racetrack_map: HashMap<usize, HashMap<usize, String>> = HashMap::new();
    contents.lines().enumerate().for_each(|(i, line)| {
        let mut row_map = HashMap::new();
        line.chars().enumerate().for_each(|(j, c)| {
            if c.to_string() == "S" {
                start_position = (i as usize, j);
            } else if c.to_string() == "E" {
                end_position = (i as usize, j);
            }
            row_map.insert(j, c.to_string());
        });
        racetrack_map.insert(i, row_map);
    });

    let mut previous_position = start_position;
    let mut current_position = start_position;
    let mut racetrack_costs = HashMap::new();
    let mut counter = 0 as isize;
    racetrack_costs.insert(start_position, counter);
    while current_position != end_position {
        let next_position = get_next_position(previous_position, current_position, &racetrack_map, &"#".to_string());
        counter += 1;
        racetrack_costs.insert(next_position, counter);
        previous_position = current_position;
        current_position = next_position;
    }

    let mut cheats;
    if *part == 1 {
        cheats = get_cheats(&racetrack_costs);
    } else {
        cheats = get_cheats_v2(&racetrack_costs, 20);
    }
    cheats.sort_by(|a, b| a.2.cmp(&b.2));
    let grouped_cheats = cheats.iter().chunk_by(|x| x.2).into_iter().map(|(key, group)| (key, group.collect::<Vec<_>>())).collect::<Vec<(isize, Vec<&(usize, usize, isize)>)>>();
    grouped_cheats.iter().for_each(|(cost_saved, group)| {
        println!("There are {} cheats that save {} picoseconds", group.len(), cost_saved);
    });
    println!("Response: {:?}", grouped_cheats.iter().filter(|(cost_saved, _)| *cost_saved >= 100).map(|(_, group)| group.len() as i32).sum::<i32>());
}
