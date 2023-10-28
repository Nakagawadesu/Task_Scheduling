use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};

pub(crate) struct Colony{
    pub(crate) pherohormones: Vec<f64>,
    pub(crate) step : f64,
    pub(crate) evaporation : f64
}

impl Colony{

    pub fn new(n_tasks : usize , evap : f64)-> Self{
        Self {
            pherohormones : vec![0.0; n_tasks ],
            step : 1.0/(n_tasks as f64),
            evaporation : evap
        }
        

    }

    pub fn add_pherohormones(
        &mut self,
        iteration : i128,
        task: usize
    ){ 
 //toda task completada insere um vertice com peso proposcional a 1/n_tasks 
        self.pherohormones[task as usize] = 1.0 - (iteration as f64 * self.step);
    }

    pub fn evaporate_pherormones(&mut self)
    {
        for i in 0..self.pherohormones.len(){
            if self.pherohormones[i] >= self.evaporation 
            {
                self.pherohormones[i] = self.pherohormones[i] - self.evaporation;
            }
            
        }
 // reduz o valor total de pherormonios em todas as rotas 
    }
    pub fn print_pherohormones_vec(&self){
        for i in 0..self.pherohormones.len(){
           
             println!("node {} : {}   ",i,self.pherohormones[i]);
            
            
        }
    }
}