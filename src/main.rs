mod Graphs;
mod Ants;
mod Aco;

use petgraph::graph::{ NodeIndex};
use std::time::{Instant, Duration};
fn main() {
    let file_path = "/home/matheus/Projects/Task_Scheduling/Task_Scheduling/Data_sets/100/";
    let graph_name = "rand0161.stg";
    let mut graph = Graphs::Utils::new();
    graph.initialize_graph(file_path,graph_name);
    let n_tasks = graph.di_graph.node_count();
    graph.find_max_cost_unlocks(n_tasks);

    let n_ants = 4;

    let start_time = Instant::now();
/*
    let mut colony = Ants::Army::Colony::new(n_tasks ,0.1);
    let mut worker = Ants::ManagerAnt::new(0.2, &graph.remaining_vec);
    let sequence = worker.work(&mut graph ,n_ants,&mut colony);
    */
    
    let mut aco = Aco::Aco::new(n_tasks, 100);
    
    aco.optimal(&mut graph, n_tasks, n_ants , 0.2, 0.1);//graph,n_tasks, n_processors,wisdom,evaporation
    
    let end_time = Instant::now();
    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);
    // Convert the elapsed time to seconds (or other units)

    let elapsed_seconds = elapsed_time.as_secs();
    let elapsed_millis = elapsed_time.as_millis();
    let elapsed_micros = elapsed_time.as_micros();

    println!(" Sequence:");
        for i in 0..n_tasks {
            //print!(" {}",sequence[i]);
            print!(" {}",aco.optimal_schedule[i]);
        }
    println!(" ");    
    println!(
        "{} Ants spent : {} , computer spent {} micro seconds",
    n_ants,
    //worker.time_spent ,
    aco.optimal_time, 
    elapsed_micros
    );
    graph.write_results_to_file(
        "/home/matheus/Projects/Task_Scheduling/Task_Scheduling/Results/",
        &graph_name ,
        &aco.optimal_schedule , 
        &aco.optimal_time,
        &n_ants
    );


}
