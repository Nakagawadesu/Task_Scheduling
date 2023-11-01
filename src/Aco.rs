use crate::Ants;
use crate::Graphs::Utils;
pub(crate) struct Aco{
    pub(crate) colonies: i128,
    pub(crate) optimal_schedule: Vec<i128>,
    pub(crate) optimal_time:i128
}

impl Aco{
    pub fn new(n_tasks : usize ,n_colonies : i128 )->Self{
        Self{
            colonies : n_colonies,
            optimal_schedule: vec![0; n_tasks],
            optimal_time: i128::MAX
        }
    }
    pub fn optimal(&mut self,graph : &mut Utils, n_tasks : i128,n_ants : i128,manager_wisdom : f64 ,  evaporation : f64,  ){
       
        let mut colony = Ants::Army::Colony::new(n_tasks as usize ,evaporation);
        for i in 0..self.colonies{
            
            let mut worker = Ants::ManagerAnt::new(manager_wisdom);
            let sequence = worker.work(&mut graph ,n_ants,&mut colony);
            if worker.time_spent < self.optimal_time{
                self.optimal_time = worker.time_spent;
                self.optimal_schedule = sequence;
            }
        }
    }
}