
use std::collections::HashMap;
use phf::phf_ordered_map;

static DIRECTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "v" => (1, 0),
    "^" => (-1, 0),
    ">" => (0, 1),
    "<" => (0, -1),
};

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}


pub fn run(contents: String, part: &i8) {
    let mut counter = 0;
    let guard_directions: Vec<&&str> = DIRECTIONS_MAP.keys().collect();
    let mut guard_direction = String::new();
    let mut guard_position = Position { x: 0, y: 0 };
    let mut obstacles = Vec::new();
    contents.lines().enumerate().for_each(|(i, line)| {
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
    println!("Guard position {:?} and direction {}", guard_position, guard_direction);
    if part == &1 {
        // START WALKING
        let direction_instruction = DIRECTIONS_MAP.get(&guard_direction).unwrap();
        obstacles.iter().filter(|&obstacle| {
            // obstacle.x ==
        }).for_each(|_| {
            counter += 1;
        });

    } else if part == &2 {
        // todo
    }
    println!("Response: {}", counter);

}
