
use std::collections::{HashMap, HashSet};
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
    let mut new_empty_vec: Vec<String> = Vec::new();
    loop {
        let next_position = new_guard_position.get_next_position_backwards( &guard_direction);
        if next_position.x >= 0 as usize && next_position.y >= 0 as usize && next_position.x < line_count && next_position.y < line_length {
            if let Some(v) = unvisited_positions.get(&(next_position.x.clone(), next_position.y.clone())) {
                v.push(guard_direction.to_string());
            } else {
                unvisited_positions.insert((next_position.x.clone(), next_position.y.clone()), & mut vec![guard_direction.to_string()]);
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
    println!("Unvisited {:?}", unvisited_positions);
    unvisited_positions
}

pub fn run(contents: String, part: &i8) {
    let mut counter = 0;
    let line_count = contents.lines().count();
    let guard_directions: Vec<&&str> = DIRECTIONS_MAP.keys().collect();
    let mut guard_direction = String::new();
    let mut guard_position = Position { x: 0, y: 0 };
    let mut visited_positions: HashMap<(usize, usize), & mut Vec<String>> = HashMap::new();
    let mut unvisited_positions: HashMap<(usize, usize), & mut Vec<String>> = HashMap::new();
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
    loop {
        visited_positions.insert((guard_position.x.clone(), guard_position.y.clone()), & mut vec![guard_direction]);
        let next_position = guard_position.get_next_position( &guard_direction);
        if next_position.x >= 0 as usize && next_position.y >= 0 as usize && next_position.x < line_count && next_position.y < line_length {
            let new_entry = (next_position.x.clone(), next_position.y.clone());
            let unvisited_vec = unvisited_positions.get(&new_entry).unwrap_or_else(|| &Vec::new());
            let visited_vec = visited_positions.get(&new_entry).unwrap_or_else(|| &Vec::new());
            let possible_next_direction = NEXT_DIRECTION_MAP.get(&guard_direction).unwrap().to_string();
            if unvisited_vec.contains(&possible_next_direction) || visited_vec.contains(&possible_next_direction) {
                println!("{:?}", new_entry);
                counter += 1
            };
            match obstacles.clone().iter().find(|position| position.x == next_position.x && position.y == next_position.y) {
                None => {
                    if let Some(v) = visited_positions.get(&new_entry) {
                        v.push(guard_direction);
                    } else {
                        visited_positions.insert(new_entry, & mut vec![guard_direction]);
                    }
                    guard_position = next_position.clone();
                    // println!("{:?}", guard_position);
                },
                Some(_) => {
                    guard_direction = NEXT_DIRECTION_MAP.get(&guard_direction).unwrap().to_string();
                    // unvisited_positions = unvisited_positions.union(&get_unvisited_positions(&guard_position, &guard_direction, line_count, line_length, obstacles.clone())).cloned().collect();
                    // println!("Position {:?} Direction {:?}", guard_position, guard_direction);
                    // println!("Unvisited out {:?}", unvisited_positions);
                },
            }
        } else {
            println!("{:?}", guard_position);
            break;
        }
    }
    if part == &1 {
        println!("Response: {}", visited_positions.len());

        // obstacles.iter().filter(|&obstacle| {
        //     obstacle.x ==
        // }).for_each(|position| {
        //     visited_positions.insert(value)
        //     counter += 1;
        // });

    } else if part == &2 {
        println!("Response: {}", counter);
        // todo
    }

}
