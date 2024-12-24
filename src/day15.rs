use std::collections::{HashMap, VecDeque};

use phf::phf_ordered_map;

static DIRECTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "v" => (1, 0),
    "^" => (-1, 0),
    ">" => (0, 1),
    "<" => (0, -1),
};

fn print_warehouse_map(warehouse_map: &HashMap<usize, HashMap<usize, String>>) -> isize {
    let mut gps_sum: isize = 0;
    for i in 0..warehouse_map.len() {
        if let Some(row_map) = warehouse_map.get(&i) {
            let mut line_as_str = String::new();
            for j in 0..row_map.len() {
                if let Some(elem) = row_map.get(&j) {
                    line_as_str += elem.to_string().as_str();
                    if elem == &"O".to_string() || elem == &"[".to_string() {
                        gps_sum += i as isize * 100 + j as isize;
                    }
                } else {
                    break;
                }
            }
            print!("{}\n", line_as_str);
        } else {
            break;
        }
    }
    gps_sum
}

pub fn run(contents: String, part: &i8) {
    let mut robot_position = (0, 0);
    let mut lines_iterator = contents.lines();
    let mut warehouse_map: HashMap<usize, HashMap<usize, String>> = HashMap::new();
    if part == &1 {
        for i in 0.. {
            let line = lines_iterator.next().unwrap();
            if line == "" {
                break;
            }
            let mut row_map = HashMap::new();
            line.chars().enumerate().for_each(|(j, c)| {
                if c.to_string() == "@" {
                    robot_position = (i as usize, j);
                }
                row_map.insert(j, c.to_string());
            });
            warehouse_map.insert(i, row_map);
        }
    } else if part == &2 {
        for i in 0.. {
            let line = lines_iterator.next().unwrap();
            if line == "" {
                break;
            }
            let mut row_map = HashMap::new();
            line.chars().enumerate().for_each(|(j, c)| {
                if c.to_string() == "@" {
                    robot_position = (i as usize, 2 * j);
                    row_map.insert(2 * j, c.to_string());
                    row_map.insert(2 * j + 1, ".".to_string());
                } else if c.to_string() == "O" {
                    row_map.insert(2 * j, "[".to_string());
                    row_map.insert(2 * j + 1, "]".to_string());
                } else {
                    row_map.insert(2 * j, c.to_string());
                    row_map.insert(2 * j + 1, c.to_string());
                    
                }
            });
            warehouse_map.insert(i, row_map);
        }
    }
    print_warehouse_map(&warehouse_map);
    println!("Robot position {:?}", robot_position);
    let mut instructions = String::new();

    while let Some(instruction_set) = lines_iterator.next() {
        println!("Movement Instructions {:?}", instruction_set);

        for instruction in instruction_set.chars() {
            println!("Instruction {:?}", instruction);
            let (dx, dy) = DIRECTIONS_MAP[&instruction.to_string()];
            if part == &1 {
                let mut movement_deque: VecDeque<String> = VecDeque::new();
                for i in 1.. {
                    let (new_x, new_y) = (robot_position.0 as isize + dx * i, robot_position.1 as isize + dy * i);
                    // println!("New Position {:?}", (new_x, new_y));
                    let found_elem = warehouse_map.get(&(new_x as usize)).unwrap().get(&(new_y as usize)).unwrap().clone();
                    // println!("Found Element {:?}", found_elem);
                    if found_elem == "#".to_string() {
                        // println!("Wall found");
                        break;
                    } else if found_elem == "O".to_string() {
                        movement_deque.push_back(found_elem);
                    } else if found_elem == ".".to_string() {
                        movement_deque.push_front("@".to_string());
                        movement_deque.push_front(found_elem);
                        movement_deque.iter().enumerate().for_each(|(j, elem)| {
                            // println!("elem {} {:?}", j, elem);
                            warehouse_map.get_mut(&((robot_position.0 as isize + dx * j as isize) as usize))
                              .unwrap().insert((robot_position.1 as isize + dy * j as isize) as usize, elem.clone());
                        });
                        robot_position = ((robot_position.0 as isize + dx) as usize, (robot_position.1 as isize + dy) as usize);
                        println!("Robot Position updated to {:?}", robot_position);
                        break;
                    }
                }
                movement_deque.clear();
            } else if part == &2 {
                let mut movement_deques: Vec<((usize, usize), VecDeque<String>)> = Vec::new();
                let mut flag_stop = false;
                movement_deques.push(((robot_position.0, robot_position.1), VecDeque::from(vec!["@".to_string()])));
                for m in 0.. {
                    let mut additional_movement_deques: Vec<((usize, usize), VecDeque<String>)> = Vec::new();
                    if let Some((initial_position, movement_deque)) = movement_deques.get_mut(m) {
                        let (x, y) = initial_position;
                        for i in 1.. {
                            let (new_x, new_y) = (*x as isize + dx * i, *y as isize + dy * i);
                            // println!("New Position {:?}", (new_x, new_y));
                            let found_elem = warehouse_map.get(&(new_x as usize)).unwrap().get(&(new_y as usize)).unwrap().clone();
                            // println!("Found Element {:?}", found_elem);
                            if found_elem == "#".to_string() {
                                // println!("Wall found");
                                flag_stop = true;
                                movement_deques.clear();
                                break;
                            } else if found_elem == "]".to_string() || found_elem == "[".to_string() {
                                movement_deque.push_back(found_elem.clone());
                                if dx != 0 && found_elem == "[".to_string() {
                                    additional_movement_deques.push(((new_x as usize, new_y as usize + 1), VecDeque::from(vec!["]".to_string()])));
                                } else if dx != 0 && found_elem == "]".to_string() {
                                    additional_movement_deques.push(((new_x as usize, new_y as usize - 1), VecDeque::from(vec!["[".to_string()])));
                                }
                            } else if found_elem == ".".to_string() {
                                movement_deque.push_front(found_elem);
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                    if flag_stop {
                        break;
                    }
                    movement_deques.extend(additional_movement_deques);
                }
                println!("Movement Deques {:?}", movement_deques);
                let mut moved_boxes = Vec::new();
                movement_deques.iter().for_each(|(initial_position, movement_deque)| {
                    if moved_boxes.contains(initial_position) {
                        return;
                    }
                    let (x, y) = initial_position;
                    movement_deque.iter().enumerate().for_each(|(j, elem)| {
                        warehouse_map.get_mut(&((*x as isize + dx * j as isize) as usize))
                          .unwrap().insert((*y as isize + dy * j as isize) as usize, elem.clone());
                        moved_boxes.push(((*x as isize + dx * j as isize) as usize, (*y as isize + dy * j as isize) as usize));
                    });
                    if initial_position == &robot_position {
                        robot_position = ((*x as isize + dx) as usize, (*y as isize + dy) as usize);
                        println!("Robot Position updated to {:?}", robot_position);
                    }
                });
            }
            let gps_sum = print_warehouse_map(&warehouse_map);
            println!("GPS Sum {:?}", gps_sum);
        }

        instructions.push_str(instruction_set);
    }
}