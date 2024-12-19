use std::collections::{HashMap, HashSet};
use itertools::Itertools;


pub fn run(contents: String, part: &i8) {
    let mut last_elem: (usize, usize) = (0, 0);
    let mut antennas_map: HashMap<usize, Vec<String>> = HashMap::new();
    let mut antinode_locations = HashSet::new();
    contents.lines().enumerate().for_each(|(i, line)| {
        let v: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        last_elem = (i, v.len() - 1);
        antennas_map.insert(i, v);
    });
    let mut antennas_location: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    for index_row in 0..antennas_map.len() {
        let row = antennas_map.get(&index_row).unwrap();
        println!("{:?}", row);
        for index_col in 0..row.len() {
            let location = row.get(index_col).unwrap();
            if location != "." {
                if !antennas_location.contains_key(location) {
                    antennas_location.insert(location.to_string(), vec![]);
                }
                antennas_location.get_mut(&location.to_string()).unwrap().push((index_row, index_col));
            }
        }
    }
    for (antenna_frequency, positions) in antennas_location.iter() {
        for (first, last) in positions.into_iter().tuple_combinations() {
            println!("Antennas: {:?} {:?}", first, last);
            let row_diff = last.0 as i32 - first.0 as i32;
            let col_diff = last.1 as i32 - first.1 as i32;
            if part == &1 {
                let first_antinode = (first.0 as i32 - row_diff, first.1 as i32 - col_diff);
                let last_antinode = (last.0 as i32 + row_diff, last.1 as i32 + col_diff);
                println!("Antinodes: {:?} {:?}", first_antinode, last_antinode);
                if first_antinode.0 >= 0 && first_antinode.1 >= 0 && first_antinode.0 <= last_elem.0 as i32 && first_antinode.1 <= last_elem.1 as i32 {
                    // println!("Antinode is within the board");
                    antinode_locations.insert(first_antinode);
                }
                if last_antinode.0 >= 0 && last_antinode.1 >= 0 && last_antinode.0 <= last_elem.0 as i32 && last_antinode.1 <= last_elem.1 as i32 {
                    // println!("Antinode is within the board");
                    antinode_locations.insert(last_antinode);
                }
            } else if part == &2 {
                for (node, signal) in vec![(first, -1), (last, 1)] {
                    for i in 0.. {
                        let antinode = (node.0 as i32 + row_diff * i * signal, node.1 as i32 + col_diff * i * signal);
                        println!("Antinode: {:?}", antinode);
                        if !(antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 <= last_elem.0 as i32 && antinode.1 <= last_elem.1 as i32) {
                            break;
                        }
                        // println!("Antinode is within the board");
                        antinode_locations.insert(antinode);
                    }
                }
            }
        }
        println!("Antenna: {} {:?}", antenna_frequency, positions);
    }
    println!("Response: Antinodes count: {}", antinode_locations.len());
}