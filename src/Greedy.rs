
use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};
use std::collections::BinaryHeap; 
use petgraph::Direction;

pub(crate) struct  Workers{
    pub(crate) time_spent : i128,
    pub(crate) task_heap : BinaryHeap<i128>
}

impl Workers{
    pub fn new() -> Self {
        Self {
            time_spent : 0,
            task_heap: BinaryHeap::new()
        }
    }
    

    pub fn reduce_neighbors(&mut self ,graph : &mut  StableDiGraph::<i128, i128> , remaining_vec : &mut  Vec::<i128>  , task: NodeIndex){

        let mut neighbors : Vec<i128> = Vec::new() ;
        let index :i128 = task.index() as i128;
        
        for neighbor in graph.neighbors_directed( task, Direction::Outgoing) {
            let neighbor_index = graph[neighbor];
            neighbors.push(neighbor_index);
        }
        //println!("task : {} , neghbors :\n",index);
        for i in neighbors{
            remaining_vec[i as usize] = remaining_vec[i as usize] -1;
            println!("reduced {}",i);
            println!("remaining :");
            self.print_remaining_vec(&remaining_vec);
            if remaining_vec[i as usize] == 0 {
                self.task_heap.push(i);
                println!(" {} inserted ",i);
                
            }
        }
        
        
    }

    pub fn remove_edges(&self ,graph: &mut StableDiGraph<i128, i128>, task: NodeIndex) {
        let neighbors: Vec<NodeIndex> = graph.neighbors(task).collect();
    
        for neighbor in neighbors {
            if let Some(edge) = graph.find_edge(task, neighbor) {
                graph.remove_edge(edge);
            }
        }
    }
/*
    pub fn calculate_priority(&self, graph : &mut  DiGraph::<i128, i128> , task : i128){

    }
*/  fn choose_task(&mut self ) -> Option<i128>{
    if let Some(task) = self.task_heap.pop() {
        println!("{} popped \n", task);
        Some(task)
    } else {
        None
    }
}
    pub fn complete_task(&mut self, graph : &mut  StableDiGraph::<i128, i128> ,remaining_vec : &mut  Vec::<i128> , task: NodeIndex){


        self.reduce_neighbors(graph,remaining_vec ,task);
        self.remove_edges(graph, task);
        graph.remove_node(task);
    
    }
    

    pub fn simple_greedy_ants(&mut self ,graph : &mut  StableDiGraph::<i128, i128>,remaining_vec : &mut  Vec::<i128> ){

        let mut task : i128 ;
        let mut task_node_index = NodeIndex::new(0 as usize);
        self.complete_task(graph, remaining_vec, task_node_index);

        while graph.node_count() > 0 {
            task = match self.choose_task() {
                
                Some(task) => task,
                None => break
            };
            print!(" -> {}\n",task);    
            task_node_index = NodeIndex::new(task as usize);
            self.complete_task(graph, remaining_vec, task_node_index);
        }
    }
    pub fn print_remaining_vec(&self,remaining_vec : &Vec::<i128>) {
        println!(" Remainig: :");
        for i in remaining_vec {
            print!(" {}",remaining_vec[*i as usize]);
        }
        print!("\n");
    }
    
}