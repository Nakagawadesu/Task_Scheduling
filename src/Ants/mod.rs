
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
    pub(crate) remaining_vec: Vec<i128> 
    
}

impl ManagerAnt {
    pub fn new(wisdom: f64, vec : &Vec<i128> ) -> Self {
        Self {
            time_spent: 0,
            task_heap: BinaryHeap::new(),
            w : wisdom,
            remaining_vec: vec.clone()

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

        
        let remaining_vec = &mut self.remaining_vec;
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

    fn choose_task(&mut self ) -> Option<TaskTuple>{
        if let Some(task) = self.task_heap.pop() {
            println!("{} chosen \n", task.node.index());
            Some(task)
        } else {
            //println!("empty binary heap \n");
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

    pub fn complete_task(
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
        colony.add_pherohormones(iteration , task.node.index() );
    
    }
    

    pub fn work(
        &mut self , 
        graph : &mut Utils,
        n_workers: i128,
        colony : &mut Colony
        )->Vec<i128>{

        let mut sequence :Vec<i128> = Vec::new();    
        let mut task_tuple = TaskTuple::new(NodeIndex::new(0),0.0);
        let mut workers: Vec<WorkerAnt> = vec![WorkerAnt::new(-1); n_workers as usize];
        let mut counter : i128 = 0;
        
        let mut clone = graph.di_graph.clone();

        self.complete_task(graph,&mut clone, &task_tuple, &mut sequence,&mut workers, 0 as usize,colony, counter);


        while clone.node_count() > 0 {
           for i in 0..n_workers{
                //println!("n_worker : {} , current worker : {} ",n_workers, i);
                let index = i as usize;

                //println!("worker: {}, spent: {} , ",i as i128,self.time_spent);
                if workers[index].free_at <= self.time_spent {
                    counter += 1;
                    //liberação do worker
                    if workers[index].free_at <= self.time_spent && workers[index].current_task != -1{
                        task_tuple = TaskTuple::new(NodeIndex::new(workers[index].current_task as usize),0.0);
                        println!("finished: {}",workers[index].current_task );
                        self.complete_task(graph, &mut clone, &task_tuple, &mut sequence, &mut workers, index, colony, counter);
                        workers[index].current_task = -1;
                       
                    }
                    //se liberado botar para trabalhar
                    if  workers[index].current_task == -1 {
                        if let Some(task) = self.choose_task() {
                            workers[index].start_task(
                                &self.time_spent , 
                                &(task.node.index() as i128),
                                &graph.costs_vec[task.node.index() as usize]
                            );
                            println!("worker :  {} , started at : {} , finishes at : {}, task: {}",
                            i,
                            self.time_spent,
                            workers[index].free_at,
                            workers[index].current_task,
                   );
                    } else {
                            println!("worker {} waiting,  current time spent {}", i, self.time_spent);
                        }
                    }
                    
                }
           } 
           //print!("current time : {}",self.time_spent);
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