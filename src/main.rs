use petgraph::graph::{DiGraph, NodeIndex};
use std::fs;
use std::io::{BufRead, BufReader};

fn show_content() {
    println!(
        "In file {}",
        "/home/matheus/Projects/Task/Data_Sets/50/atest.stg"
    );

    let contents = fs::read_to_string("/home/matheus/Projects/Task/Data_Sets/50/atest.stg")
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}

fn initialize_graph(file_path: &str) -> (DiGraph<i128, i128>, Vec<i128>,Vec<i128> ) {
    let mut graph = DiGraph::<i128, i128>::new();
    let mut remaining_vec: Vec<i128> = Vec::new();
    let mut costs_vec: Vec<i128> = Vec::new();


    if let Ok(file) = fs::File::open(file_path) {
        let reader = BufReader::new(file);
        let mut count: i128 = 0;
        let mut task: i128 = 0;
        let mut degree: i128 = 0;
        let mut line_count: i128 = 0;
        let mut n_tasks: i128 = 0 ;
        for line in reader.lines() {
            let line = line.expect("Failed to read line from file");
            //println!("{}", line);
            if line.starts_with("#") {
                break;
            }
            let parsed_vec: Vec<i128> = line
                .split_whitespace()
                .map(|s| s.trim().parse::<i128>().expect("Invalid integer"))
                .collect();

            for i in &parsed_vec {
                if count == 0 {
                    n_tasks = *i+2;
                    if line_count == 0 {
                        remaining_vec = vec![0; n_tasks as usize];
                        costs_vec = vec![0; n_tasks as usize];
                        for j in 0..n_tasks{
                            graph.add_node(j);
                        }
                    } else {
                        println!("Task: {}", i);
                        task = *i;
                        count += 1;
                    }
                } else if count == 1 {
                    println!("Cost: {}", i);
                    costs_vec[task as usize]  = *i;
                    count += 1;
                } else if count == 2 {
                    println!("Degree: {}", i);
                    degree = *i;
                    count += 1;
                } else {
                    println!(" {}", i);
                    remaining_vec[task as usize] += 1;
                    graph.add_edge(NodeIndex::new(*i as usize), NodeIndex::new(task as usize), 0);
                }
            }
            count = 0;
            line_count += 1;
        }
    } else {
        eprintln!("Error opening the file");
    }
    (graph, remaining_vec, costs_vec)
}

fn update_edge_weights(graph: &mut DiGraph<i128, i128>, costs_vec: &[i128]) {


    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let target_index = target.index();
        if let Some(&weight) = costs_vec.get(target_index) {
            graph.update_edge(source, target, weight);
        }
    }
}
fn print_graph(graph: &DiGraph<i128, i128>) {
    println!("Nodes in the graph:");
    for node in graph.node_indices() {
        println!("Node {}: {:?}", node.index(), graph[node]);
    }

    println!("Edges in the graph:");
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let weight = graph[edge]; // Get the weight of the edge
        println!("Edge from {} to {} with weight {}", source.index(), target.index(), weight);
    }
}
fn print_vecs(remaining_vec : &Vec<i128>, costs_vec: &Vec<i128>,  n_tasks: usize) {
    

    println!("Task: \t Remainig: \t Cost :");
    for i in 0..n_tasks{
        println!("{}        \t {}       \t{}", i, remaining_vec[i], costs_vec[i]);
    }
}

fn main() {
    let (mut graph, remaining_vec, costs_vec) = initialize_graph("/home/matheus/Projects/Task/Data_Sets/50/atest.stg");
    let n_tasks = graph.node_count();
    update_edge_weights(&mut graph, &costs_vec);
    print_graph(&graph);
    print_vecs(&remaining_vec, &costs_vec, n_tasks);

}
