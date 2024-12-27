use std::collections::{HashMap, HashSet, VecDeque};
use phf::phf_ordered_map;
use graph_builder::prelude::*;


static INSTRUCTIONS_MAP: phf::OrderedMap<&'static str, (i32, i32)> = phf_ordered_map! {
    "U" => (-1, 0),
    "D" => (1, 0),
    "R" => (0, 1),
    "L" => (0, -1),
};

fn get_height(direction: &str) -> String {
    if direction == "R" || direction == "L" {
        return "I".to_string();
    } else {
        return "S".to_string();
    }
}

fn get_next_available_directions(reindeer_map: &HashMap<usize, HashMap<usize, String>>, current_position: &(usize, usize), current_direction: &str) -> Vec<((usize, usize), String, String)> {
    let mut available_directions: Vec<((usize, usize), String, String)> = Vec::new();
    let (i, j) = current_position;
    let (di, dj) = INSTRUCTIONS_MAP[current_direction];
    let next_position = ((*i as i32 + di) as usize, (*j as i32 + dj) as usize);
    INSTRUCTIONS_MAP.entries().for_each(|(direction, (ndi, ndj))| {
        if *ndi == -di && *ndj == -dj {
            return;
        }
        if let Some(new_position) = reindeer_map.get(&((next_position.0 as i32 + *ndi) as usize)).and_then(|row_map| row_map.get(&((next_position.1 as i32 + *ndj) as usize))) {
            if *new_position == "." || *new_position == "E" {
                available_directions.push((next_position, direction.to_string(), new_position.to_string()));
            }
        }
    });
    available_directions
}

fn dijkstra_shortest_path(graph: &UndirectedCsrGraph<i32, (), i32>, start_node: i32, end_node: i32) -> Vec<i32> {
    let mut visited = vec![false; graph.node_count() as usize];
    let mut shortest_distance = vec![std::i32::MAX; graph.node_count() as usize];
    shortest_distance[start_node as usize] = 0;
    let mut previous = vec![std::i32::MAX; graph.node_count() as usize];
    let mut queue = std::collections::BinaryHeap::new();
    queue.push((0, start_node));
    while let Some((_, node)) = queue.pop() {
        if visited[node as usize] {
            continue;
        }
        let node_distance = shortest_distance[node as usize];
        visited[node as usize] = true;
        println!("node: {:?} and targets {:?}", node, graph.neighbors_with_values(node));
        for target in graph.neighbors_with_values(node) {
            let next_node = target.target;
            let next_distance = node_distance + target.value;
            if next_node != node && next_distance < shortest_distance[next_node as usize] {
                shortest_distance[next_node as usize] = next_distance;
                previous[next_node as usize] = node;
                queue.push((std::i32::MAX - next_distance, next_node));
            }
        }
        // println!("visited: {:?}", visited);
        // println!("shortest_distance: {:?}", shortest_distance);
        // println!("previous: {:?}", previous);
    }
    let mut path = vec![];
    let mut current_node = end_node;
    while current_node != std::i32::MAX {
        path.push(current_node);
        current_node = previous[current_node as usize];
        // println!("path: {:?}", path);
    }
    path.reverse();
    println!("minimal cost to reach end: {:?}", shortest_distance[end_node as usize]);
    path
}

fn get_paths(node: i32, previous: Vec<Vec<i32>>, path: Option<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut _path = path.unwrap_or(vec![]).clone();
    let mut current_node = node;
    while current_node != std::i32::MAX {
        _path.push(current_node);
        let current_nodes_vec = previous[current_node as usize].clone();
        if current_nodes_vec.len() == 1 {
            current_node = current_nodes_vec[0];
        } else {
            let mut paths = vec![];
            current_nodes_vec.into_iter().for_each  (|n| {
                let alternative_paths = get_paths(n, previous.clone(), Some(_path.clone()));
                paths.extend(alternative_paths);
            });
            return paths;
        }
    }
    vec![_path]
}

fn get_shortest_paths(graph: &UndirectedCsrGraph<i32, (), i32>, start_node: i32, end_node: i32) -> Vec<Vec<i32>> {
    let mut visited = vec![false; graph.node_count() as usize];
    let mut shortest_distance = vec![std::i32::MAX; graph.node_count() as usize];
    shortest_distance[start_node as usize] = 0;
    let mut previous = vec![vec![std::i32::MAX]; graph.node_count() as usize];
    let mut queue = std::collections::BinaryHeap::new();
    queue.push((0, start_node));
    while let Some((_, node)) = queue.pop() {
        if visited[node as usize] {
            continue;
        }
        let node_distance = shortest_distance[node as usize];
        visited[node as usize] = true;
        println!("node: {:?} and targets {:?}", node, graph.neighbors_with_values(node));
        for target in graph.neighbors_with_values(node) {
            let next_node = target.target;
            let next_distance = node_distance + target.value;
            if next_node != node && next_distance <= shortest_distance[next_node as usize] {
                if next_distance == shortest_distance[next_node as usize] {
                    previous[next_node as usize].push(node);
                } else {
                    shortest_distance[next_node as usize] = next_distance;
                    previous[next_node as usize] = vec![node];
                }
                queue.push((std::i32::MAX - next_distance, next_node));
            }
        }
        // println!("visited: {:?}", visited);
        // println!("shortest_distance: {:?}", shortest_distance);
    }
    let paths = get_paths(end_node, previous.clone(), None);
    // println!("previous: {:?}", previous);
    // let mut path = vec![];
    // let mut current_node = end_node;
    // while current_node != std::i32::MAX {
    //     path.push(current_node);
    //     current_node = previous[current_node as usize];
        // println!("path: {:?}", path);
    // }
    // path.reverse();
    // println!("minimal cost to reach end: {:?}", shortest_distance[end_node as usize]);
    // path
    paths
}

pub fn run(contents: String, part: &i8) {
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    let mut reindeer_map: HashMap<usize, HashMap<usize, String>> = HashMap::new();
    contents.lines().enumerate().for_each(|(i, line)| {
        let mut row_map = HashMap::new();
        line.chars().enumerate().for_each(|(j, c)| {
            if c.to_string() == "S" {
                start_position = (i as usize, j);
            } else if c.to_string() == "E" {
                end_position = (i as usize, j);
            }
            row_map.insert(j, c.to_string());
        });
        reindeer_map.insert(i, row_map);
    });
    println!("{:?}", &reindeer_map);
    // walk to create graph
    let mut graph_node_index = 3;
    let mut graph_nodes: HashMap<((usize, usize), String), i32> = HashMap::new();
    let mut graph_edges: Vec<(i32, i32, i32)> = Vec::new();
    let mut graph_edges_map: HashMap<(i32, i32), HashSet<(usize, usize)>> = HashMap::new();
    let mut walking_deque = VecDeque::new();
    let mut visited_vec = Vec::new();
    graph_nodes.insert((end_position, "I".to_string()), 0);
    graph_nodes.insert((start_position, "I".to_string()), 1);
    graph_nodes.insert((start_position, "S".to_string()), 2);
    graph_edges.push((1, 2, 1000));
    graph_edges_map.insert((1, 2), vec![start_position.clone()].into_iter().collect());
    get_next_available_directions(&reindeer_map, &(start_position.0, start_position.1 - 1), "R")
      .iter().for_each(|(current_position, current_direction, _)| {
        walking_deque.push_back((current_position.clone(), current_direction.clone()));
      });
    // walking_deque.push_back((start_position, "R".to_string()));
    // walking_deque.push_back((start_position, "U".to_string()));
    while let Some((initial_position, initial_direction)) = walking_deque.pop_front() {
        // let (i, j) = initial_position;
        if visited_vec.contains(&(initial_position, initial_direction.clone())) {
            continue;
        }
        visited_vec.push((initial_position, initial_direction.clone()));
        let current_start_index: i32;
        let current_end_index: i32;
        let initial_height = get_height(&initial_direction.clone());
        current_start_index = graph_nodes[&(initial_position, initial_height)];
        let mut current_position = initial_position;
        let mut current_direction = initial_direction.to_string();
        // let mut current_char = reindeer_map.get(&current_position.0).unwrap().get(&current_position.1).unwrap().to_string();
        let mut cost: i32 = 0;
        let mut current_edges: Vec<(i32, i32, i32)> = Vec::new();
        let mut temp_vec_positions: HashSet<(usize, usize)> = HashSet::new();
        temp_vec_positions.insert(current_position.clone());
        // current edge is a tuple of (start_node, end_node, cost)
        loop {
            let available_directions = get_next_available_directions(&reindeer_map, &current_position, &current_direction);
            println!("{:?}", available_directions);
            let old_current_direction = current_direction.clone();
            if available_directions.len() == 0 {
                break;
            } else if available_directions.len() == 1 {
                cost += 1;
                let current_char: String;
                (current_position, current_direction, current_char) = available_directions[0].clone();
                if old_current_direction != current_direction {
                    cost += 1000;
                }
                temp_vec_positions.insert(current_position.clone());
                if current_char == "E" {
                    cost += 1;
                    current_edges.push((current_start_index, 0, cost));
                    graph_edges_map.insert((current_start_index, 0), temp_vec_positions.clone());
                    break;
                }
            } else if available_directions.len() >= 2  {
                cost += 1;
                let mut next_position: (usize, usize) = (0, 0);
                available_directions.iter().for_each(|(n_p, n_d, _)| {
                    if next_position == (0, 0) {
                        next_position = *n_p;
                    }
                    walking_deque.push_back((*n_p, n_d.to_string()));
                });
                if !graph_nodes.contains_key(&(next_position.clone(), "I".to_string())) {
                    graph_nodes.insert((next_position.clone(), "I".to_string()), graph_node_index);
                    graph_nodes.insert((next_position.clone(), "S".to_string()), graph_node_index + 1);
                    current_edges.push((graph_node_index, graph_node_index + 1, 1000));
                    graph_edges_map.insert((graph_node_index, graph_node_index + 1), vec![end_position.clone()].into_iter().collect());
                    graph_node_index += 2;
                }
                let current_height = get_height(&current_direction);
                current_end_index = graph_nodes[&(next_position, current_height)];
                if !graph_edges.contains(&(current_end_index, current_start_index, cost)) {
                    current_edges.push((current_start_index, current_end_index, cost));
                }
                temp_vec_positions.insert(current_position.clone());
                graph_edges_map.insert((current_start_index, current_end_index), temp_vec_positions.clone());
                break;
            }
        }
        temp_vec_positions.clear();
        if !current_edges.is_empty() {
            graph_edges.extend(current_edges.into_iter());
        }
    }
    // sort graph nodes based on value
    let mut graph_nodes_sorted: Vec<(((usize, usize),String), i32)> = graph_nodes.into_iter().collect();
    graph_nodes_sorted.sort_by(|a, b| a.1.cmp(&b.1));
    graph_nodes_sorted.iter().for_each(|(k, v)| {
        println!("{:?} -> {:?}", v, k);
    });
    // println!("{:?}", &graph_nodes.iter().collect());
    println!("{:?}", &graph_edges);

    let graph: UndirectedCsrGraph<i32, (), i32> = GraphBuilder::new()
      .csr_layout(CsrLayout::Sorted)
      .edges_with_values(graph_edges)
      .build();

    if part == &1 {
        let result = dijkstra_shortest_path(&graph, 1, 0);
        println!("Result {:?}", result);
    } else {
        println!("Part 2 ->"); 
        graph_edges_map.iter().for_each(|(k, v)| {
            println!("{:?} -> {:?}", k, v);
        });
        let paths = get_shortest_paths(&graph, 1, 0);
        let mut best_spots_to_sit = HashSet::new();
        paths.iter().for_each(|path| {
            let mut _path = path.clone();
            _path.reverse();
            println!("path: {:?}", _path);
            _path.windows(2).for_each(|w| {
                let (start, end) = (w[0], w[1]);
                println!("{:?} -> {:?}", start, end);
                graph_edges_map.get(&(start, end)).unwrap_or_else(|| graph_edges_map.get(&(end, start)).unwrap()).iter().for_each(|spot| {
                    best_spots_to_sit.insert(spot);
                });
            });
            println!("{:?}", best_spots_to_sit);
        });
        println!("{:?}", best_spots_to_sit);
        println!("Count of best spots to sit: {:?}", best_spots_to_sit.len());
    }

}
