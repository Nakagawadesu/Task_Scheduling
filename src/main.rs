mod Graphs;
mod Ants;
use petgraph::graph::{ NodeIndex};
use std::time::{Instant, Duration};

fn main() {
    let mut graph = Graphs::Utils::new();
    graph.initialize_graph("/home/matheus/Projects/Task_Scheduling/Task_Scheduling/Data_sets/50/rand0157.stg");
    let n_tasks = graph.di_graph.node_count();
    let  max_cost : i128 ;
    let  max_unlocks : i128 ;
    //worker ants per colony
    let  n_ants:i128 = 2;
    //graph.print_graph();
    graph.find_max_cost_unlocks(n_tasks);
    //graph.print_vecs(n_tasks);

    
    let mut worker = Ants::ManagerAnt::new(0.3 , 0.7);
    let start_time = Instant::now();
    let sequence = worker.greedy_ants(&mut graph ,n_ants);
    
    let end_time = Instant::now();
    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);
    // Convert the elapsed time to seconds (or other units)
    let elapsed_seconds = elapsed_time.as_secs();
    let elapsed_millis = elapsed_time.as_millis();
    let elapsed_micros = elapsed_time.as_micros();

    graph.print_graph();
    println!(" Sequence:");
        for i in 0..n_tasks {
            print!(" {}",sequence[i]);
        }
    println!(" ");    
    println!(
        "{} Ants spent : {} , computer spent {} micro seconds",
    n_ants,worker.time_spent ,
     elapsed_micros
    );


}
