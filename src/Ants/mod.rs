
mod Tuple;
mod Worker;

use petgraph::stable_graph::{StableDiGraph, NodeIndex};
use std::collections::BinaryHeap; 
use petgraph::Direction;

use crate::Graphs::Utils;
use Tuple::TaskTuple;
use Worker::WorkerAnt;

pub(crate) struct ManagerAnt {
    pub(crate) time_spent: i128,
    pub(crate) task_heap: BinaryHeap<TaskTuple>,
    pub(crate) letragrega: f64,//falar com o marco
    pub(crate) outraletra: f64
    
}

impl ManagerAnt {
    pub fn new(a: f64 , b: f64) -> Self {
        Self {
            time_spent: 0,
            task_heap: BinaryHeap::new(),
            letragrega : a,
            outraletra : b
        }
    }
    pub fn reduce_neighbors(
        &mut self ,
        graph : &mut  Utils,
          task: &TaskTuple
        ){

        let mut neighbors : Vec<i128> = Vec::new() ;
        let di_graph = &graph.di_graph;
        for neighbor in di_graph.neighbors_directed( task.node, Direction::Outgoing) {
            let neighbor_index = di_graph[neighbor];
            neighbors.push(neighbor_index);
        }
        //println!("task : {} , neghbors :\n",index);

        
        let remaining_vec = &mut graph.remaining_vec;
        for i in neighbors{
            remaining_vec[i as usize] = remaining_vec[i as usize] -1;
            //println!("reduced {}",i);
            //self.print_remaining_vec(&remaining_vec);
            if remaining_vec[i as usize] == 0 {
                let mut inserted : TaskTuple = TaskTuple::new(NodeIndex::new(i as usize) , 0.0);
                //calculate params
                let index = inserted.node.index();
                let cost_ratio = (1.0 -(graph.costs_vec[index] as f64 / graph.max_cost as f64 )) as f64;
                let unlocks_ratio = (graph.unlocks_vec[index] as f64/ graph.max_unlocks as f64) as f64;
                inserted.priority = self.letragrega * cost_ratio + self.outraletra * unlocks_ratio;
                //println!("inserted : {} ,priority {}",inserted.node.index() , inserted.priority);
                self.task_heap.push(inserted);
               // println!(" {} inserted ",i);
                
            }
        }
        
        
    }

    fn choose_task(&mut self, task : &TaskTuple ) -> Option<TaskTuple>{
        if let Some(task) = self.task_heap.pop() {
            //println!("{} popped \n", task.node.index());
            Some(task)
        } else {
            None
        }
    }
    

    pub fn remove_edges(&self ,graph: &mut StableDiGraph<i128,i128>, task: NodeIndex) {
        let neighbors: Vec<NodeIndex> = graph.neighbors(task).collect();
    
        for neighbor in neighbors {
            if let Some(edge) = graph.find_edge(task, neighbor) {
                graph.remove_edge(edge);
            }
        }
    }

    pub fn start_task(
        &mut self,
        graph : &mut  Utils ,
        task: &TaskTuple, 
        sequence : &mut Vec<i128>,
        workers: &mut Vec<WorkerAnt>   ,
        worker : usize
    ){

        self.reduce_neighbors(graph,task);
        self.remove_edges(&mut graph.di_graph, task.node);
        graph.di_graph.remove_node(task.node);
        sequence.push(task.node.index().try_into().unwrap());
        workers[worker].start_task(
            &self.time_spent , 
            &graph.costs_vec[task.node.index() as usize]
        )
    
    }
    

    pub fn greedy_ants(
        &mut self , 
        graph : &mut  Utils,
        n_workers: i128
        )->Vec<i128>{

        let mut sequence :Vec<i128> = Vec::new();    
        let mut task = TaskTuple::new(NodeIndex::new(0),0.0);
        let mut workers: Vec<WorkerAnt> = vec![WorkerAnt::new(0); n_workers as usize];

        self.start_task(graph, &task, &mut sequence,&mut workers, 0 as usize);


        while graph.di_graph.node_count() > 0 {
           for i in 0..n_workers{
                if workers[i as usize].free_at <= self.time_spent {
                    task = match self.choose_task(&task) {
                        Some(task) => task,
                        None => break
                    };
                    self.start_task(graph, &task,&mut sequence,&mut workers ,i as usize);
                    println!("worker :  {} , started at : {} , finishes at : {}", i , self.time_spent ,workers[i as usize].free_at );
                }
           } 
           // print!(" -> {}\n",task.node.index() );
            
           
           self.time_spent += 1;
        }
        //grafo vazio poré podem ter tarefas ainda sendo executadas
        let mut last_finished : i128 = -1;
        for i in 0..n_workers{
            if last_finished < workers[i as usize].free_at{
                last_finished = workers[i as usize].free_at;
                
            }
        }
        self.time_spent = last_finished;    
        sequence
    }

    pub fn print_remaining_vec(&self, remaining_vec: &Vec<i128>) {
        println!(" Remaining:");
        for (index, value) in remaining_vec.iter().enumerate() {
            println!(" {}: {}", index, value);
        }
    }
    
}