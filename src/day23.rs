use std::collections::HashSet;
use graph_builder::prelude::*;
use itertools::Itertools;

fn bron_kerbosch_algorithm(graph: &UndirectedCsrGraph<usize>, r: Vec<usize>, mut p: Vec<usize>, mut x: Vec<usize>, mut cliques: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }
    while !p.is_empty() {
        let mut p_clone = p.clone();
        let v = p_clone.pop().unwrap();
        let neighbors: Vec<usize> = graph.neighbors(v).copied().collect();
        let mut r_new = r.clone();
        r_new.push(v);
        let p_new = p_clone.iter().filter(|&index| neighbors.contains(index)).copied().collect();
        let x_new = x.iter().filter(|&index| neighbors.contains(index)).copied().collect();
        cliques = bron_kerbosch_algorithm(graph, r_new, p_new, x_new, cliques);
        p = p.iter().filter(|&index| *index != v).copied().collect();
        x.push(v);
    }
    cliques
}

pub fn run(contents: String, part: &i8) {
    let mut visited = Vec::new();
    let mut counter: usize = 0;
    let network_links = contents.lines().map(|line| {
        let splitted: Vec<&str> = line.split("-").collect();
        let num_splitted: Vec<usize> = splitted.iter().map(|&node| {
            if let Some(index) = visited.iter().position(|&x| x == node) {
                return index;
            } else {
                visited.push(node);
                counter += 1;
                return counter - 1;
            }
        }).collect();
        (num_splitted[0], num_splitted[1])
    }).collect::<Vec<(usize,usize)>>();
    println!("{:?}", &network_links);
    println!("{:?}", &visited);
    let graph: UndirectedCsrGraph<usize> = GraphBuilder::new()
      .csr_layout(CsrLayout::Sorted)
      .edges(network_links)
      .build();
    println!("{:?}", &graph.edge_count());
    if part == &1 {
        let mut triangles = HashSet::new();  
        visited.iter().enumerate().filter(|(index1, &node1)| node1.starts_with("t") && graph.degree(*index1) >= 2).for_each(|(index1, _)| {
            graph.neighbors(index1).filter(|&index2| *index2 != index1 && graph.degree(*index2) >= 2).for_each(|index2| {
                graph.neighbors(*index2).filter(|&&index3| index3 != index1 && index3 != *index2).for_each(|index3| {
                    if graph.neighbors(*index3).contains(&index1) {
                        let mut indexes = vec![index1, *index2, *index3];
                        indexes.sort();
                        triangles.insert((indexes[0], indexes[1], indexes[2]));
                    }
                })
            });
        });
        triangles.iter().for_each(|(index1, index2, index3)| {
            println!("{} {} {}", visited[*index1], visited[*index2], visited[*index3]);
        });
        println!("Response part 1 {:?}", &triangles.len());
    } else {
        let mut cliques = bron_kerbosch_algorithm(&graph, Vec::new(), (0..graph.node_count()).collect(), Vec::new(), Vec::new());
        cliques.sort_by(|clique1, clique2| clique1.len().cmp(&clique2.len()));
        println!("cliques max {:?}", cliques[cliques.len() - 1]);
        let mut visited: Vec<&str> = cliques[cliques.len() - 1].iter().map(|index| visited[*index]).collect();
        visited.sort();
        let password = visited.join(",");

        println!("Response part 2 {:?}", password);
    }
}
