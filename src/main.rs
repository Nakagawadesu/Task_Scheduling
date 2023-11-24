mod Graphs;
mod Ants;
mod Aco;

use petgraph::graph::{ NodeIndex};
use std::time::{Instant, Duration};
fn main() {
    let file_path = "/home/matheus/Projects/Task_Scheduling/Task_Scheduling/Data_sets/1250/";
    let graph_name = "rand0000.stg";
    let mut graph = Graphs::Utils::new();
    graph.initialize_graph(file_path,graph_name);
    let n_tasks = graph.di_graph.node_count();
    graph.find_max_cost_unlocks(n_tasks);
    let visibility_init = graph.update_visibility(n_tasks);
    graph.print_visibility(n_tasks , &visibility_init);

    let n_ants = 2;
    let a = 1.001;
    let manager_wisdom = 0.0;
    let pherohormones_intensity = 0.001;
    let randomness  =  0.5;
    let n_colonies = 5000;
    let stop_counter = 5;
    let start_time = Instant::now();
/*
    let mut colony = Ants::Army::Colony::new(n_tasks ,0.1);
    colony.update_visibility_sum();
    let mut worker = Ants::ManagerAnt::new(0.2, &graph.remaining_vec);
    let sequence = worker.work(&mut graph ,n_ants,&mut colony);
*/   
    
    let mut aco = Aco::Aco::new(n_tasks, n_colonies);
    
   aco.optimal(&mut graph, n_tasks, n_ants ,a ,manager_wisdom, randomness, pherohormones_intensity , visibility_init, stop_counter);
  // aco.optimal_with_counter(&mut graph, n_tasks, n_ants ,a ,manager_wisdom, randomness, pherohormones_intensity , visibility_init,stop_counter);
    
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
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
