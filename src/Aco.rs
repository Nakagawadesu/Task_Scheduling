use crate::Ants;
use crate::Graphs::Utils;
pub(crate) struct Aco{
    pub(crate) colonies: i128,
    pub(crate) optimal_schedule: Vec<i128>,
    pub(crate) optimal_time:i128,
}

impl Aco{
    pub fn new(n_tasks : usize ,n_colonies : i128 )->Self{
        Self{
            colonies : n_colonies,
            optimal_schedule: vec![0; n_tasks],
            optimal_time: i128::MAX,

        }
    }
    
    pub fn optimal(
        &mut self,
        graph : &mut Utils, 
        n_tasks : usize,
        n_ants : i128,
        a: f64,
        manager_wisdom : f64 ,  
        randomness : f64,
        intensity : f64, 
        visibility_init: Vec<f64> ,
        counter_stop : i128
    ){
        let mut counter = 0;
        let mut colony = Ants::Army::Colony::new(n_tasks  ,visibility_init, intensity, a, manager_wisdom);
        colony.update_visibility_sum();
        for i in 0..self.colonies{

            let mut worker = Ants::ManagerAnt::new(&graph.remaining_vec,randomness);
            let sequence = worker.work( graph ,n_ants,&mut colony , i , self.colonies);
            println!("iteration : {}",i);
            if worker.time_spent < self.optimal_time{
                self.optimal_time = worker.time_spent;
                self.optimal_schedule = sequence;
            }
            if worker.time_spent == self.optimal_time{
                counter +=1 ;
            }
            else{
                counter =0;
            }
            if counter >= counter_stop{
                break;
            }
            colony.print_pherohormones_vec();
            colony.update_pherohormone_sum();
            //graph.print_graph()
        }
    }
}