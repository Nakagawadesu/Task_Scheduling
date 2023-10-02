mod Graphs;
mod Greedy;
use petgraph::graph::{ NodeIndex};
fn main() {
    let mut graph = Graphs::Utils::new();
    graph.initialize_graph("/home/matheus/Projects/Task/Data_Sets/50/atest.stg");
    let n_tasks = graph.di_graph.node_count();
    graph.update_edge_weights();
    graph.print_graph();
    graph.print_vecs(n_tasks);

    let mut worker = Greedy::Workers::new();
    
    let task_node_index = NodeIndex::new(0 as usize);
    worker.complete_task(&mut graph.di_graph,&mut graph.remaining_vec ,task_node_index);
    graph.print_graph();
    //worker.simple_greedy_ants(&mut graph.di_graph ,&mut  graph.remaining_vec);
    //graph.print_graph();

}
