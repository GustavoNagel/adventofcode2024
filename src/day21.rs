use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use phf::phf_ordered_map;

static DIRECTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "v" => (-1, 0),
    "^" => (1, 0),
    ">" => (0, 1),
    "<" => (0, -1),
};

#[derive(Debug, PartialEq, Clone)]
struct KeyPad {
    current_key: String,
    pointer_position: (isize, isize),
    key_map: HashMap<String, (isize, isize)>,
}

impl KeyPad {
    fn move_pointer(& mut self, direction: &str) -> bool{
        let direction_instruction = DIRECTIONS_MAP.get(&direction).unwrap();
        let new_x = self.pointer_position.0 as isize + direction_instruction.0;
        let new_y = self.pointer_position.1 as isize + direction_instruction.1;
        if let Some(found) = self.key_map.iter().find(|(_key, &position)| position == (new_x, new_y)) {
            self.pointer_position = (new_x, new_y);
            self.current_key = found.0.clone();
            true
        } else {
            false
        }
    }

    fn move_pointer_to(& mut self, target: &str) {
        self.pointer_position = *self.key_map.get(target).unwrap();
        self.current_key = target.to_string();
    }

    fn get_diffs(&self, target: &str) -> (isize, isize) {
        let target_position = *self.key_map.get(target).unwrap();
        (target_position.0 - self.pointer_position.0, target_position.1 - self.pointer_position.1)
    }
}

// fn print_num_keypad(keypad: &KeyPad) {
//     let mut new_key_map = HashMap::new();
//     keypad.key_map.iter().for_each(|(key, &position)| {
//         let value = if position == keypad.pointer_position { "*".to_string() } else { " ".to_string() };
//         new_key_map.insert(key.clone(), value);
//     });
//     println!("
// +---+---+---+
// | 7{}| 8{}| 9{}|
// +---+---+---+
// | 4{}| 5{}| 6{}|
// +---+---+---+
// | 1{}| 2{}| 3{}|
// +---+---+---+
//     | 0{}| A{}|
//     +---+---+",
//     new_key_map.get("7").unwrap(),
//     new_key_map.get("8").unwrap(),
//     new_key_map.get("9").unwrap(),
//     new_key_map.get("4").unwrap(),
//     new_key_map.get("5").unwrap(),
//     new_key_map.get("6").unwrap(),
//     new_key_map.get("1").unwrap(),
//     new_key_map.get("2").unwrap(),
//     new_key_map.get("3").unwrap(),
//     new_key_map.get("0").unwrap(),
//     new_key_map.get("A").unwrap());
// }

// fn print_dir_keypad(keypad: &KeyPad) {
//     let mut new_key_map = HashMap::new();
//     keypad.key_map.iter().for_each(|(key, &position)| {
//         let value = if position == keypad.pointer_position { "*".to_string() } else { " ".to_string() };
//         new_key_map.insert(key.clone(), value);
//     });
//     println!("
//     +---+---+
//     | ^{}| A{}|
// +---+---+---+
// | <{}| v{}| >{}|
// +---+---+---+",
//     new_key_map.get("^").unwrap(),
//     new_key_map.get("A").unwrap(),
//     new_key_map.get("<").unwrap(),
//     new_key_map.get("v").unwrap(),
//     new_key_map.get(">").unwrap());
// }

fn get_movement_instructions(diffs: (isize, isize)) -> VecDeque<Vec<String>> {
    let mut instructions = VecDeque::new();
    if diffs.0 > 0 {
        instructions.push_back(vec!["^".to_string(); diffs.0 as usize]);
    } else if diffs.0 < 0 {
        instructions.push_back(vec!["v".to_string(); (- diffs.0) as usize]);
    }
    if diffs.1 > 0 {
        instructions.push_back(vec![">".to_string(); diffs.1 as usize]);
    } else if diffs.1 < 0 {
        instructions.push_front(vec!["<".to_string(); (- diffs.1) as usize]);
    }
    instructions
}

// fn translate_keypad_instructions(keypad: &mut KeyPad, codes: Vec<String>, num: bool, mem: &mut HashMap<String, Vec<String>>) -> Vec<String> {
//     let mut sequence_movements = Vec::new();
//     codes.iter().for_each(|c| {
//         let target = c.to_string();
//         // println!("{:?}", target);
//         let key = format!("{}-{}", keypad.current_key, target);
//         if let Some(result_saved) = mem.get(key.as_str()) {
//             sequence_movements.extend(result_saved.clone());
//             keypad.move_pointer_to(target.as_str());
//         } else {
//             let mut temp_sequence_number = Vec::new();
//             let diffs = keypad.get_diffs(target.as_str());
//             let mut movement_instructions = get_movement_instructions(diffs);
//             // println!("{:?}", movement_instructions);
//             while let Some(instruction_vec) = movement_instructions.pop_front() {
//                 let initial_pointer_position = keypad.pointer_position.clone();
//                 let mut inner_instruction_vec = instruction_vec.clone();
//                 for instruction in instruction_vec.clone().into_iter() {
//                     if !keypad.move_pointer(instruction.as_str()) {
//                         movement_instructions.push_back(instruction_vec.clone());
//                         keypad.pointer_position = initial_pointer_position;
//                         inner_instruction_vec.clear();
//                         break;
//                     }
//                 }
//                 temp_sequence_number.extend(inner_instruction_vec);
//             }
//             temp_sequence_number.push("A".to_string());
//             mem.insert(key, temp_sequence_number.clone());
//             sequence_movements.extend(temp_sequence_number);
//         }
//         // if num { print_num_keypad(keypad) } else { print_dir_keypad(keypad) };        
//     });
//     sequence_movements
// }

fn translate_keypad_instructions_v2(keypad: &mut KeyPad, codes: HashMap<String, usize>) -> HashMap<String, usize> {
    let mut sequence_movements: HashMap<String, usize> = HashMap::new();
    codes.iter().for_each(|(c, &counter)| {
        let splitted_key = c.split("-").map(|k| k.to_string()).collect::<Vec<String>>();
        let target = splitted_key[1].clone();
        keypad.move_pointer_to(splitted_key[0].clone().as_str());
        target.chars().for_each(|c| {
            let target = c.to_string();
            // let key = format!("{}-{}", keypad.current_key, target);
            let mut temp_sequence_number = String::new();
            let diffs = keypad.get_diffs(target.as_str());
            let mut movement_instructions = get_movement_instructions(diffs);
            // println!("{:?}", movement_instructions);
            while let Some(instruction_vec) = movement_instructions.pop_front() {
                let initial_pointer_position = keypad.pointer_position.clone();
                let mut inner_instruction_vec = instruction_vec.clone();
                for instruction in instruction_vec.clone().into_iter() {
                    if !keypad.move_pointer(instruction.as_str()) {
                        movement_instructions.push_back(instruction_vec.clone());
                        keypad.pointer_position = initial_pointer_position;
                        inner_instruction_vec.clear();
                        break;
                    }
                }
                temp_sequence_number.extend(inner_instruction_vec);
            }
            temp_sequence_number.push_str("A");
            // mem.insert(key, temp_sequence_number.clone());
            let new_key = format!("{}-{}", splitted_key[0].clone(), temp_sequence_number);
            let old_value = sequence_movements.get(&new_key).unwrap_or(&0);
            sequence_movements.insert(new_key, old_value + (1 * counter));
        });
        // if num { print_num_keypad(keypad) } else { print_dir_keypad(keypad) };        
    });
    sequence_movements
}

pub fn run(contents: String, part: &i8) {   
    let numeric_map: HashMap<String, (isize, isize)> = HashMap::from([
        ("0".to_string(), (0, 1)),
        ("A".to_string(), (0, 2)),
        ("1".to_string(), (1, 0)),
        ("2".to_string(), (1, 1)),
        ("3".to_string(), (1, 2)),
        ("4".to_string(), (2, 0)),
        ("5".to_string(), (2, 1)),
        ("6".to_string(), (2, 2)),
        ("7".to_string(), (3, 0)),
        ("8".to_string(), (3, 1)),
        ("9".to_string(), (3, 2)),
    ]);
    let directional_map: HashMap<String, (isize, isize)> = HashMap::from([
        ("<".to_string(), (0, 0)),
        ("v".to_string(), (0, 1)),
        (">".to_string(), (0, 2)),
        ("^".to_string(), (1, 1)),
        ("A".to_string(), (1, 2)),
    ]);

    let sum_complexities: i64 = contents.lines().map(|line| {
        let dir_key_pad_base = KeyPad { current_key: "A".to_string(), pointer_position: (1, 2), key_map: directional_map.clone()};
        let mut num_key_pad = KeyPad { current_key: "A".to_string(), pointer_position: (0,2), key_map: numeric_map.clone()};
        let codes: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        let num_as_string = String::from(codes.iter().take(3).join(""));
        let num_ref = num_as_string.parse::<i64>().unwrap();

        // -------------------------- V1 --------------------------

        // let mut mem: HashMap<String, Vec<String>> = HashMap::new();
        // let mut sequence_movements: Vec<String> = translate_keypad_instructions(&mut num_key_pad, codes.clone(), true, &mut mem);
        // println!("Seq Numbers <num>: {:?}", sequence_movements);
        // println!("Seq Numbers len <num>: {:?}", sequence_movements.len());
        // let iterations_number = if part == &1 { 2 } else { 25 };
        // for i in 0..iterations_number {
        //     println!("Iteration: {}", i + 1);
        //     let mut dir_key_pad_generic = dir_key_pad_base.clone();
        //     sequence_movements = translate_keypad_instructions(&mut dir_key_pad_generic, sequence_movements.clone(), false, &mut mem);
        //     // println!("Seq Numbers <num{}>: {:?}", i + 1, sequence_movements);
        //     // println!("Seq Numbers len <num{}>: {:?}", i + 1, sequence_movements.len());
        // }
        // // println!("Num: {:?}; Min Len: {:?}", num_ref, sequence_movements.len());
        // sequence_movements.len() as i64 * num_ref

        // -------------------------- V2 --------------------------
        let codes = format!("A-{}", codes.iter().join(""));
        let initial_map: HashMap<String, usize> = [(codes, 1)].iter().cloned().collect();
        println!("Initial Map: {:?}", initial_map);
        let mut sequence_movements: HashMap<String, usize> = translate_keypad_instructions_v2(&mut num_key_pad, initial_map);
        println!("Seq Numbers <num>: {:?}", sequence_movements);
        println!("Seq Numbers len <num>: {:?}", sequence_movements.values().sum::<usize>());
        let iterations_number = if part == &1 { 2 } else { 25 };
        for i in 0..iterations_number {
            println!("Iteration: {}", i + 1);
            let mut dir_key_pad_generic = dir_key_pad_base.clone();
            sequence_movements = translate_keypad_instructions_v2(&mut dir_key_pad_generic, sequence_movements.clone());
            println!("Seq Numbers <num{}>: {:?}", i + 1, sequence_movements);
        }
        println!("Num: {:?}; Min Len: {:?}", num_ref, sequence_movements.len());
        sequence_movements.iter().map(|(c, &n)| {
            c.split("-").collect::<Vec<&str>>()[1].len() * n
        }).sum::<usize>() as i64 * num_ref
        // -------------------------- V1 --------------------------
    }).sum();

    println!("Response: {}", sum_complexities);
}