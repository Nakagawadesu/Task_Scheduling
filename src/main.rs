mod Graphs;
mod AntColony;
use petgraph::graph::{ NodeIndex};
fn main() {
    let mut graph = Graphs::Utils::new();
    graph.initialize_graph("/home/matheus/Projects/Task/Data_Sets/50/atest.stg");
    let n_tasks = graph.di_graph.node_count();
    
    graph.print_graph();
    graph.print_vecs(n_tasks);

    let mut worker = AntColony::Workers::new();
    /*
    let task_node_index = NodeIndex::new(0 as usize);
    
    worker.complete_task(&mut graph.di_graph,&mut graph.remaining_vec ,task_node_index);
    graph.print_graph();
    */
    worker.simple_worker_ants(&mut graph.di_graph ,&mut  graph.remaining_vec,&graph.costs_vec);
    graph.print_graph();
    println!("time spent : {}",worker.time_spent);


}
