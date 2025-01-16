
use std::collections::HashMap;
use phf::phf_ordered_map;

static DIRECTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "v" => (1, 0),
    "^" => (-1, 0),
    ">" => (0, 1),
    "<" => (0, -1),
};

static NEXT_DIRECTION_MAP: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "^" => ">",
    ">" => "v",
    "v" => "<",
    "<" => "^",
};

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn get_next_position(&self, direction: &str) -> Self {
        let direction_instruction = DIRECTIONS_MAP.get(&direction).unwrap();
        let new_x = self.x as isize + direction_instruction.0;
        let new_y = self.y as isize + direction_instruction.1;
        Position { x: new_x as usize, y: new_y as usize}
    }

    fn get_next_position_backwards(&self, direction: &str) -> Self {
        let direction_instruction = DIRECTIONS_MAP.get(&direction).unwrap();
        let new_x = self.x as isize - direction_instruction.0;
        let new_y = self.y as isize - direction_instruction.1;
        Position { x: new_x as usize, y: new_y as usize}
    }
}

fn get_unvisited_positions(guard_position: &Position, guard_direction: &str, line_count: usize, line_length: usize, obstacles: Vec<Position>) -> HashMap<(usize, usize), Vec<String>> {
    let mut new_guard_position = guard_position.clone();
    let mut unvisited_positions: HashMap<(usize, usize), Vec<String>> = HashMap::new();
    loop {
        let next_position = new_guard_position.get_next_position_backwards( &guard_direction);
        if next_position.x >= 0 as usize && next_position.y >= 0 as usize && next_position.x < line_count && next_position.y < line_length {
            if let Some(v) = unvisited_positions.get_mut(&(next_position.x.clone(), next_position.y.clone())) {
                v.push(guard_direction.to_string());
            } else {
                unvisited_positions.insert((next_position.x.clone(), next_position.y.clone()), vec![guard_direction.to_string()]);
            }
            match obstacles.iter().find(|position| position.x == next_position.x && position.y == next_position.y) {
                None => {
                    new_guard_position = next_position.clone();
                },
                Some(_) => {
                    break;
                },
            }
        } else {
            break;
        }
    }
    // println!("Unvisited {:?}", unvisited_positions);
    unvisited_positions
}

fn quick_walk(visited_positions: HashMap<(usize, usize), Vec<String>>, current_entry: (usize, usize), next_direction: String, obstacles: Vec<Position>, line_count: usize, line_length: usize) -> bool {
    let mut new_guard_position = Position { x: current_entry.0, y: current_entry.1 };
    let mut visited_positions = visited_positions.clone();
    let mut guard_direction = next_direction.clone();
    loop {
        let next_position = new_guard_position.get_next_position(&guard_direction);
        let new_entry = (next_position.x.clone(), next_position.y.clone());
        if next_position.x >= 0 as usize && next_position.y >= 0 as usize && next_position.x < line_count && next_position.y < line_length {
            match obstacles.iter().find(|position| position.x == next_position.x && position.y == next_position.y) {
                None => {
                    new_guard_position = next_position.clone();
                    if let Some(v) = visited_positions.get_mut(&new_entry) {
                        if v.contains(&guard_direction) {
                            return true;
                        }
                        v.push(guard_direction.clone());
                    } else {
                        visited_positions.insert(new_entry, vec![guard_direction.clone()]);
                    }
                },
                Some(_) => {
                    guard_direction = NEXT_DIRECTION_MAP.get(&guard_direction).unwrap().to_string();
                    if let Some(v) = visited_positions.get(&new_entry) {
                        if v.contains(&guard_direction) {
                            return true;
                        }
                    }
                },
            }
        } else {
            break;
        }
    }
    // println!("Quick walk finished {:?}", current_entry);
    false
}

pub fn run(contents: String, part: &i8) {
    let mut counter = 0;
    let line_count = contents.lines().count();
    let guard_directions: Vec<&&str> = DIRECTIONS_MAP.keys().collect();
    let mut guard_direction = String::new();
    let mut guard_position = Position { x: 0, y: 0 };
    let mut visited_positions: HashMap<(usize, usize), Vec<String>> = HashMap::new();
    let mut unvisited_positions: HashMap<(usize, usize), Vec<String>>;
    let mut obstacles = Vec::new();
    let mut line_length = 0; 
    contents.lines().enumerate().for_each(|(i, line)| {
        if line_length == 0 {
            line_length = line.len();
        }
        line.chars().enumerate().for_each(|(j, c)| {
            if c.to_string() == "#" {
                obstacles.push(Position { x: i, y: j });
            } else if guard_directions.contains(&&c.to_string().as_str()) {
                guard_position = Position { x: i, y: j };
                guard_direction = c.to_string();
            }
        });
    });
    println!("Obstacles {:?}", obstacles);
    println!("Line len {:?}", line_length);
    println!("Line count {:?}", line_count);
    println!("Guard position {:?} and direction {}", guard_position, guard_direction);
    // START WALKING
    unvisited_positions = get_unvisited_positions(&guard_position, &guard_direction, line_count, line_length, obstacles.clone());
    visited_positions.insert((guard_position.x.clone(), guard_position.y.clone()), vec![guard_direction.clone()]);
    loop {
        let current_entry = (guard_position.x.clone(), guard_position.y.clone());
        let unvisited_vec = unvisited_positions.get(&current_entry).unwrap_or(&vec![]).clone();
        let temp_vec = Vec::new();
        let visited_vec = visited_positions.get(&current_entry).unwrap_or(&temp_vec);
        let possible_next_direction = NEXT_DIRECTION_MAP.get(&guard_direction).unwrap().to_string();
        let mut new_obstacles = obstacles.clone();
        // println!("{:?} {:?} -> {}", unvisited_vec, visited_vec, possible_next_direction);
        // visited_positions.insert(current_entry, vec![guard_direction.clone()]);
        let next_position = guard_position.get_next_position( &guard_direction);
        if next_position.x >= 0 as usize && next_position.y >= 0 as usize && next_position.x < line_count && next_position.y < line_length {
            new_obstacles.push(next_position.clone());
            let new_entry = (next_position.x.clone(), next_position.y.clone());
            if !visited_positions.contains_key(&new_entry) {
              if unvisited_vec.contains(&possible_next_direction) || visited_vec.contains(&possible_next_direction) {
                println!("{:?}", current_entry);
                counter += 1
              } else if quick_walk(visited_positions.clone(), current_entry, possible_next_direction, new_obstacles, line_count, line_length) {
                println!("{:?}", current_entry);
                counter += 1
              }
            };
            // visited_positions.insert(current_entry, vec![guard_direction.clone()]);
            match obstacles.clone().iter().find(|position| position.x == next_position.x && position.y == next_position.y) {
                None => {
                    if let Some(v) = visited_positions.get_mut(&new_entry) {
                        v.push(guard_direction.clone());
                        // println!("Visited {:?}", v);
                    } else {
                        visited_positions.insert(new_entry, vec![guard_direction.clone()]);
                    }
                    guard_position = next_position.clone();
                    // println!("Visited again {:?}", visited_positions);
                },
                Some(_) => {
                    guard_direction = NEXT_DIRECTION_MAP.get(&guard_direction).unwrap().to_string();
                    let new_unvisited_positions = get_unvisited_positions(&guard_position, &guard_direction, line_count, line_length, obstacles.clone());
                    new_unvisited_positions.iter().for_each(|(key, value)| {
                        if let Some(v) = unvisited_positions.get_mut(key) {
                            v.extend(value.clone());
                        } else {
                            unvisited_positions.insert(key.clone(), value.clone());
                        }
                    });
                    // unvisited_positions = unvisited_positions.union(&).cloned().collect();
                    println!("Position {:?} Direction {:?}", guard_position, guard_direction);
                    // println!("unvisited out {:?}", unvisited_positions);
                },
            }
        } else {
            visited_positions.insert(current_entry, vec![guard_direction.clone()]);
            println!("{:?}", guard_position);
            break;
        }
    }
    if part == &1 {
        println!("Response: {}", visited_positions.len());

    } else if part == &2 {
        println!("Response: {}", counter);
    }

}
