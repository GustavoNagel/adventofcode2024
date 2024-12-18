use std::collections::{HashMap, HashSet};
use phf::phf_ordered_map;

static DIRECTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "DOWN" => (1, 0),
    "UP" => (-1, 0),
    "RIGHT" => (0, 1),
    "LEFT" => (0, -1),
};

static CORNERS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "DOWN-LEFT" => (1, -1),
    "UP-LEFT" => (-1, -1),
    "DOWN-RIGHT" => (1, 1),
    "UP-RIGHT" => (-1, 1),
};

fn get_region_sides(region: Vec<(usize, usize)>) -> i32 {
    let mut count_edges = 0;
    if region.len() == 1 {
        return 4;
    }
    for (index_row, index_col) in region.iter() {
        let mut same_region = HashSet::new();
        DIRECTIONS_MAP.entries().for_each(|(name, direction)| {
            let index_row = *index_row as isize + direction.0;
            let index_col = *index_col as isize + direction.1;
            if index_row >= 0 && index_col >= 0 &&region.contains(&(index_row as usize, index_col as usize)) {
                same_region.insert(*name);
            }
        });
        if same_region.len() == 1 {
            count_edges += 2
        } else {
            if (!same_region.contains("UP") && !same_region.contains("LEFT"))
                || (!same_region.contains("UP") && !same_region.contains("RIGHT"))
                || (!same_region.contains("DOWN") && !same_region.contains("LEFT"))
                || (!same_region.contains("DOWN") && !same_region.contains("RIGHT")) {
                count_edges += 1;
            }
            CORNERS_MAP.entries().for_each(|(name, direction)| {
                let index_row = *index_row as isize + direction.0;
                let index_col = *index_col as isize + direction.1;
                let name_vec: Vec<&str> = name.split('-').collect();
                if same_region.contains(name_vec[0]) && same_region.contains(name_vec[1]) && !region.contains(&(index_row as usize, index_col as usize)) {
                    count_edges += 1;
                }
            });
        }
    }
    count_edges
}

fn find_region(garden_plots_map: &HashMap<usize, Vec<String>>, visited: &mut Vec<(usize, usize)>, elem: (usize, usize), last_elem: (usize, usize), fence_len: &mut i32) {
    let mut homogeneous_board_count = 0;
    let initial_plant_type_found = garden_plots_map.get(&elem.0).unwrap().get(elem.1).unwrap();
    DIRECTIONS_MAP.values().for_each(|direction| {
        let index_row = elem.0 as isize + direction.0;
        let index_col = elem.1 as isize + direction.1;
        if index_row >= 0 && index_col >= 0 && index_row <= last_elem.0 as isize && index_col <= last_elem.1 as isize {
            let plant_type_found = garden_plots_map.get(&(index_row as usize)).unwrap().get(index_col as usize).unwrap();
            if plant_type_found == initial_plant_type_found {
                homogeneous_board_count += 1;
                if !visited.contains(&(index_row as usize, index_col as usize)) {
                    visited.push((index_row as usize, index_col as usize));
                    find_region(garden_plots_map, visited, (index_row as usize, index_col as usize), last_elem, fence_len);
                }
            }
        }

    });
    *fence_len += 4 - homogeneous_board_count;
}

pub fn run(contents: String, part: &i8) {
    let mut total_fence_price = 0;
    let mut last_elem: (usize, usize) = (0, 0);
    let mut garden_plots_map: HashMap<usize, Vec<String>> = HashMap::new();
    contents.lines().enumerate().for_each(|(i, line)| {
        let v: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        last_elem = (i, v.len() - 1);
        // replace letters for numbers
        garden_plots_map.insert(i, v);
    });
    println!("{:?}", last_elem);
    let mut visited: Vec<(usize, usize)> = vec![];
    for index_row in 0..garden_plots_map.len() {
        let row = garden_plots_map.get(&index_row).unwrap();
        println!("{:?}", row);
        for index_col in 0..row.len() {
            let plant_type_found = row.get(index_col).unwrap();
            if visited.contains(&(index_row, index_col)) {
                continue;
            }
            let mut region: Vec<(usize, usize)> = vec![];
            region.push((index_row, index_col));
            let mut fence_len = 0;
            find_region(&garden_plots_map, &mut region, (index_row, index_col), last_elem, &mut fence_len);
            let area = region.len() as i32;
            println!("Type {} fence len {} area {} region {:?}", plant_type_found, fence_len, area, region);
            visited.extend(region.iter().cloned());
            if part == &1 {
                total_fence_price += area * fence_len;
            } else {
                total_fence_price += area * get_region_sides(region.clone());
            }
        }
    }
    println!("Response: {:?}", total_fence_price);
}
