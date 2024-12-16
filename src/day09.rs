use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Clone)]
struct MemorySlot {
    slot: Vec<i32>,
    is_file: bool,
    empty_space: usize,
    index: i32,
    order: i32,
}

fn compact_disk_map(memory_map: &[MemorySlot], moved_file: &Vec<i32>) -> Vec<i32> {
    let mut compacted_disk_map: Vec<i32> = Vec::new();
    memory_map.iter().for_each(|mem_slot| {
        if mem_slot.is_file {
            if !moved_file.contains(&mem_slot.index) {
                compacted_disk_map.extend(mem_slot.slot.iter());
            } else {
                compacted_disk_map.extend(vec![0; mem_slot.slot.len()].iter());
            }
        } else {
            compacted_disk_map.extend(mem_slot.slot.iter());
            if mem_slot.empty_space > 0 {
                // if mem_slot.empty_space > 0 add zeros to the compacted_disk_map to ignore in the check sum
                compacted_disk_map.extend(vec![0; mem_slot.empty_space].iter());
            }
        }
    });
    compacted_disk_map
}
/// Disk Fragmenter main method
pub fn run(contents: String, part: &i8) {
    let disk_map_input: Vec<String> = contents.chars().map(|c| c.to_string()).collect();
    println!("{:?}", disk_map_input);
    if part == &1 {
        let mut disk_map: VecDeque<i32> = disk_map_input.iter().enumerate().map(|(i, c)| {
            if i % 2 == 0 {
                return vec![(i / 2) as i32].repeat(c.parse().unwrap());
            } else {
                return vec![-1i32].repeat(c.parse().unwrap());
            }
        }).into_iter().flatten().collect();
        let mut compacted_disk_map: Vec<i32> = Vec::new();
        while let Some(value) = disk_map.pop_front() {
            if value < 0 {
                while let Some(value_to_move) = disk_map.pop_back() {
                    if value_to_move >= 0 {
                      compacted_disk_map.push(value_to_move);
                      break;
                    }
                }
            } else {
                compacted_disk_map.push(value);
            }
        }
        let sum_checksum: i64 = compacted_disk_map.iter().enumerate().map(|(i, e)| (i as i32 * e) as i64).sum();
        println!("Response: {}", sum_checksum);

    } else if part == &2 {
        let mut flag_file_0 = false;
        let mut disk_map: Vec<Vec<i32>> = Vec::new();
        disk_map_input.iter().enumerate().for_each(|(i, c)| {
            if i % 2 == 0 {
                if c == "0" {
                    flag_file_0 = true;
                } else {
                    disk_map.push(vec![(i / 2) as i32; c.parse().unwrap()]);
                }
            } else {
                if flag_file_0 {
                    flag_file_0 = false;
                    let size = disk_map.len();
                    disk_map[size - 1].extend(vec![-1i32; c.parse().unwrap()]);
                } else {
                    disk_map.push(vec![-1i32; c.parse().unwrap()]);
                }
            }
        });
        let mut memory_map: Vec<MemorySlot> = disk_map.iter().enumerate().map(|(i, file)| {
            if file.len() == 0 {
                return MemorySlot {
                    is_file: false,
                    slot: file.clone(),
                    empty_space: 0,
                    index: -1i32,
                    order: i as i32,
                };
            } else {
                return MemorySlot {
                    is_file: file[0] >= 0,
                    slot: if file[0] < 0 { vec![] } else { file.clone() },
                    empty_space: if file[0] < 0 { file.len() } else { 0 },
                    index: file[0],
                    order: i as i32,
                };
            }
        }).collect();
        let mut moved_file: Vec<i32> = Vec::new();
        let memory_map_copy = memory_map.clone();
        memory_map_copy.iter().rev().filter(|mem_slot| mem_slot.is_file).for_each(|file| {
            let file_len = file.slot.len();
            let empty_memory = memory_map.iter_mut().filter(|mem_slot| mem_slot.order < file.order).find(|mem_slot| mem_slot.is_file == false && mem_slot.empty_space >= file_len);
            match empty_memory {
                Some(empty_mem) => {
                    empty_mem.slot.extend(file.slot.iter());
                    empty_mem.empty_space -= file_len;
                    moved_file.push(file.index);
                    // Visualize the compacted disk map being created (40 iterations)
                    if moved_file.len() < 40 {
                        let example = compact_disk_map(&memory_map, &moved_file);
                        println!("example : {:?}", example.iter().take(40).collect::<Vec<&i32>>());
                    }
                },
                None => {}
            }
        });
        let compacted_disk_map = compact_disk_map(&memory_map, &moved_file);
        // println!("compacted_disk_map : {:?}", compacted_disk_map);
        let sum_checksum: i64 = compacted_disk_map.iter().enumerate().map(|(i, e)| (i as i32 * e) as i64).sum();
        println!("Response: {}", sum_checksum);
    }
}