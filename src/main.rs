mod Graphs;

fn main() {
    let mut graph = Graphs::Info::new();
    graph.initialize_graph("/home/matheus/Projects/Task/Data_Sets/50/atest.stg");
    let n_tasks = graph.di_graph.node_count();
    graph.update_edge_weights();
    graph.print_graph();
    graph.print_vecs(n_tasks);
}
