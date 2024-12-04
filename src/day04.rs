
use std::collections::HashMap;
use phf::phf_ordered_map;

static INSTRUCTIONS_MAP: phf::OrderedMap<&'static str, (isize, isize)> = phf_ordered_map! {
    "N" => (1, 0),
    "S" => (-1, 0),
    "E" => (0, 1),
    "W" => (0, -1),
    "NE" => (1, 1),
    "NW" => (1, -1),
    "SE" => (-1, 1),
    "SW" => (-1, -1),
};

static WORD_XMAS: &str = "XMAS";

fn check_xmas_pattern(xmas_map: &HashMap<usize, Vec<String>>, index_row: usize, index_col: usize, v: &(isize, isize)) -> bool {
    for i in 1..4 {
        if let Some(row) = xmas_map.get(&((index_row as isize + v.0 * i) as usize)) {
            if let Some(char) = row.get((index_col as isize + v.1 * i) as usize) {
                if WORD_XMAS.chars().nth(i as usize).unwrap().to_string() != *char {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_2xmas_pattern(xmas_map: &HashMap<usize, Vec<String>>, index_row: usize, index_col: usize) -> bool {
    let mut m_counter = 0;
    let mut s_counter = 0;
    let mut order_found: Vec<&str> = Vec::new();
    for v in &INSTRUCTIONS_MAP.values().collect::<Vec<_>>()[4..8] {
        if let Some(row) = xmas_map.get(&((index_row as isize + v.0) as usize)) {
            if let Some(char) = row.get((index_col as isize + v.1) as usize) {
                if WORD_XMAS.chars().nth(1 as usize).unwrap().to_string() == *char {
                    order_found.push("M");
                    m_counter += 1;
                } else if WORD_XMAS.chars().nth(3 as usize).unwrap().to_string() == *char {
                    order_found.push("S");
                    s_counter += 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    m_counter == 2 && s_counter == 2 && order_found[0] != order_found[3]
}
pub fn run(contents: String, part: &i8) {
    let mut xmas_map: HashMap<usize, Vec<String>> = HashMap::new();
    let mut counter = 0;
    contents.lines().enumerate().for_each(|(i, line)| {
        let v: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        println!("{:?}", v);
        xmas_map.insert(i, v);
    });
    for index_row in 0..xmas_map.len() {
        let row = xmas_map.get(&index_row).unwrap();
        println!("{:?}", row);
        for index_col in 0..row.len() {
            let char_found = row.get(index_col).unwrap();
            if part == &1 {
                if *char_found == WORD_XMAS.chars().nth(0).unwrap().to_string() {
                    counter += INSTRUCTIONS_MAP.values().map(|v| {
                        return check_xmas_pattern(&xmas_map, index_row, index_col, v) as isize;
                    }).filter(|&x| x == 1).sum::<isize>();
                }
            } else if part == &2 {
                if *char_found == WORD_XMAS.chars().nth(2).unwrap().to_string() {
                    counter += check_2xmas_pattern(&xmas_map, index_row, index_col) as isize;
                }
            }
        }
    }
    println!("Response: {}", counter);

}
