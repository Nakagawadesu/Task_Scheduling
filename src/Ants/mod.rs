
pub mod Tuple;
pub mod Worker;
pub mod Army;

use petgraph::stable_graph::{StableDiGraph, NodeIndex};
use std::collections::BinaryHeap; 
use petgraph::Direction;
use rand::Rng;

use crate::Graphs::Utils;
use Tuple::TaskTuple;
use Worker::WorkerAnt;
use Army::Colony;

pub(crate) struct ManagerAnt {
    pub(crate) time_spent: i128,
    pub(crate) task_heap: BinaryHeap<TaskTuple>,
    pub(crate) w: f64,//falar com o marco
    
}

impl ManagerAnt {
    pub fn new(wisdom: f64 ) -> Self {
        Self {
            time_spent: 0,
            task_heap: BinaryHeap::new(),
            w : wisdom
        }
    }
    pub fn reduce_neighbors(
        &mut self ,
        graph : &mut  Utils,
        clone : &mut StableDiGraph<i128,i128>,
        task: &TaskTuple,
        colony : &Colony
        ){

        let mut neighbors : Vec<i128> = Vec::new() ;
        for neighbor in clone.neighbors_directed( task.node, Direction::Outgoing) {
            let neighbor_index = clone[neighbor];
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
                
                //randomness
                let mut rng = rand::thread_rng();
                let random_float: f64 = rng.gen_range(0.0..0.7);
                //calculate params
                let index = inserted.node.index();
                let cost_ratio = (1.0 -(graph.costs_vec[index] as f64 / graph.max_cost as f64 )) as f64;
                let unlocks_ratio = (graph.unlocks_vec[index] as f64/ graph.max_unlocks as f64) as f64;
                inserted.priority = self.w * (cost_ratio + unlocks_ratio )+ random_float + (self.w/5.0)* colony.pherohormones[index];
                
                println!("inserted : {} ,priority {}, parameters  {}, randomness {}, pherohormones {}",
                 index,
                 inserted.priority,
                 self.w * (cost_ratio + unlocks_ratio) ,
                 random_float,
                 colony.pherohormones[index]
                );
                self.task_heap.push(inserted);
               // println!(" {} inserted ",i);
                
            }
        }
        
        
    }

    fn choose_task(&mut self, task : &TaskTuple ) -> Option<TaskTuple>{
        if let Some(task) = self.task_heap.pop() {
            println!("{} chosen \n", task.node.index());
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
        clone : &mut StableDiGraph<i128,i128>,
        task: &TaskTuple, 
        sequence : &mut Vec<i128>,
        workers: &mut Vec<WorkerAnt>   ,
        worker : usize,
        colony : &mut Colony,
        iteration : i128
    ){

        self.reduce_neighbors(graph,clone,task,&colony);
        self.remove_edges(clone, task.node);
        clone.remove_node(task.node);
        sequence.push(task.node.index().try_into().unwrap());
        workers[worker].start_task(
            &self.time_spent , 
            &graph.costs_vec[task.node.index() as usize]
        );
        colony.add_pherohormones(iteration , task.node.index() );
    
    }
    

    pub fn work(
        &mut self , 
        graph : &mut  Utils,
        n_workers: i128,
        colony : &mut Colony
        )->Vec<i128>{

        let mut sequence :Vec<i128> = Vec::new();    
        let mut task = TaskTuple::new(NodeIndex::new(0),0.0);
        let mut workers: Vec<WorkerAnt> = vec![WorkerAnt::new(-1); n_workers as usize];
        let mut counter : i128 = 0;
        
        let mut clone = graph.di_graph.clone();

        self.start_task(graph,&mut clone, &task, &mut sequence,&mut workers, 0 as usize,colony, counter);


        while clone.node_count() > 0 {
           for i in 0..n_workers{
            //println!("i : {}",i as i128);
                if workers[i as usize].free_at < self.time_spent {
                    counter += 1;
                    task = match self.choose_task(&task) {
                        Some(task) => task,
                        None => break
                    };
                    self.start_task(graph,&mut clone, &task,&mut sequence,&mut workers ,i as usize, colony,counter);
                    println!("worker :  {} , started at : {} , finishes at : {}, task: {}",
                     i ,
                     self.time_spent ,
                     workers[i as usize].free_at,
                     task.node.index() as i128 
                    );
                }
           } 
            
           
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