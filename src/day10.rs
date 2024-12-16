use std::collections::{HashMap, HashSet};


fn scale_up(index_row: usize, index_col: usize, topographic_map: &HashMap<usize, Vec<usize>>, limits: (usize, usize)) -> Vec<(usize, usize)> {
    let origin_height = *topographic_map.get(&index_row).unwrap().get(index_col).unwrap() as i32;
    let mut peaks: Vec<(usize, usize)> = vec![];
    let diffs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    diffs.iter().for_each(|(i, j)| {
        let i = index_row as i32 + i;
        let j = index_col as i32 + j;
        if i >= 0 && j >= 0 && i < limits.0 as i32 && j < limits.1 as i32 {
            let row = topographic_map.get(&(i as usize)).unwrap();
            let height_found = *row.get(j as usize).unwrap() as i32;
            // println!("Found a height: {} origin_height {} i: {} j: {} limits {:?}", height_found, origin_height, i, j, limits);
            if height_found - origin_height == 1 {
                if height_found == 9 {
                    peaks.push((i as usize, j as usize));
                }
                peaks.extend(scale_up(i as usize, j as usize, topographic_map, limits).iter());
            }
        }
    });
    return peaks;
}

pub fn run(contents: String, part: &i8) {
    let mut topographic_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut score = 0;
    let lines_count = contents.lines().count();
    let line_length = contents.lines().nth(0).unwrap().chars().count();
    let limits = (lines_count, line_length);
    contents.lines().enumerate().for_each(|(i, line)| {
        let v: Vec<usize> = line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
        println!("{:?}", v);
        topographic_map.insert(i, v);
    });
    println!("{:?}", topographic_map);
    for index_row in 0..topographic_map.len() {
        let row = topographic_map.get(&index_row).unwrap();
        //     println!("{:?}", row);
        for index_col in 0..row.len() {
            let height_found = row.get(index_col).unwrap();
            if height_found == &0 {
                println!("Found a zero at row: {}, col: {}", index_row, index_col);
                let peaks = scale_up(index_row, index_col, &topographic_map, limits);
                if part == &1 {
                    let mut peaks_set: HashSet<(usize, usize)> = HashSet::new();
                    // transform the peaks into a set to count unique tuples
                    peaks.iter().for_each(|value | {
                        peaks_set.insert(*value);
                    });
                    println!("peaks_set {:?}", peaks_set);
                    score += peaks_set.len();
                } else if part == &2 {
                    score += peaks.len();
                    
                }
            }
        }
    }
    println!("Response: {}", score);
}