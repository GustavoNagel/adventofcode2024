use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Node {
    position: (isize, isize),
    from_position: (isize, isize),
    f_cost: isize, // total cost
    g_cost: isize, // calculated cost
    h_cost: isize, // Heuristic cost
}

fn get_neighbors(position: (isize, isize), max_size: isize) -> Vec<(isize, isize)>{
    let neighbors = vec![
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ];
    neighbors.into_iter().filter(|neighbor| {
        let (x, y) = (neighbor.0, neighbor.1);
        x <= max_size && y <= max_size && x >= 0 && y >= 0
    }).map(|neighbor| (neighbor.0, neighbor.1)).collect::<Vec<(isize, isize)>>()
}

fn find_min_cost_with_a_star(obstacles_positions: Vec<(isize, isize)>) -> isize {
    let max_size: isize = 70;
    let target = (max_size, max_size);

    let mut open_vec: Vec<Node> = vec![Node { position: (0, 0), from_position: (-1, -1), f_cost: 140, g_cost: 0, h_cost: 140 }];
    let mut closed_hash = HashMap::new();

    while !open_vec.is_empty() {
        let current_node = open_vec.pop().unwrap();
        // println!("Current node: {:?}", current_node);
        closed_hash.insert(current_node.position, current_node.clone());
        if current_node.position == target {
            println!("FOUND PATH: {:?}", current_node.f_cost);
            return current_node.f_cost;
        }

        let neighbors = get_neighbors(current_node.position, max_size).into_iter().filter(|neighbor| {
            !obstacles_positions.contains(&neighbor) && closed_hash.get(&(neighbor.0, neighbor.1)).is_none()
        })
          .map(|neighbor| {
              let g_cost = current_node.g_cost + 1;
              let h_cost = target.0 - neighbor.0 + target.1 - neighbor.1;
              let f_cost = g_cost + h_cost;
              Node { position: neighbor, from_position: current_node.position, f_cost, g_cost, h_cost }
          }).collect::<Vec<Node>>();

        // println!("Neighbors: {:?}", neighbors);

        neighbors.iter().for_each(|neighbor| {
            if let Some(found_node) = open_vec.iter_mut().find(|node| node.position == neighbor.position) {
                if neighbor.f_cost < found_node.f_cost {
                    found_node.f_cost = neighbor.f_cost;
                    found_node.g_cost = neighbor.g_cost;
                    found_node.h_cost = neighbor.h_cost;
                    found_node.from_position = neighbor.from_position;
                }
            } else {
                open_vec.push(neighbor.clone());
            }
        });
        
        open_vec.sort_by(|node1, node2| node2.f_cost.cmp(&node1.f_cost));
        // println!("Open Vec{:?}", open_vec);
    }
    0
}

pub fn run(contents: String, part: &i8) {
    let mut obstacles_positions: Vec<(isize, isize)> = contents.lines().map(|line| {
        let position: (isize, isize) = {
            let mut line_iterator = line.split(",").map(|number| number.parse::<isize>());
            (line_iterator.next().unwrap().unwrap(), line_iterator.next().unwrap().unwrap())
        };
        position
    }).collect();
    // println!("Obstacles: {:?}", obstacles_positions);
    if part == &1 {
        obstacles_positions.resize(1024, (-1, -1));
        let path_cost_found: isize = find_min_cost_with_a_star(obstacles_positions);
        println!("Response: {}", path_cost_found);
    } else if part == &2 {
        let limit_num = obstacles_positions.len();
        let response: usize = (0..limit_num).collect::<Vec<_>>().partition_point(|&x| {
            let mut cloned_obstacles_positions = obstacles_positions.clone();
            cloned_obstacles_positions.resize(x, (-1, -1));
            let path_cost_found: isize = find_min_cost_with_a_star(cloned_obstacles_positions);
            println!("Trying with: {} . Path cost: {}", x, path_cost_found);
            path_cost_found > 0 
        });
        println!("Response: {:?}", obstacles_positions[response - 1]);
    }

}