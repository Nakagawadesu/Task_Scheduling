use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};

pub(crate) struct Colony{
    pub(crate) pherohormones: Vec<f64>,
    pub(crate) visibility :Vec<f64>,
    pub(crate) pherohormones_intensity:  f64,
    pub(crate) evaporation : f64,
    pub(crate) n_tasks: usize,
    pub(crate) visibility_sum: f64,
    pub(crate) pherohormones_sum:f64,
    pub(crate) a: f64,
    pub(crate) w: f64
}

impl Colony{

    pub fn new(
        n : usize ,
        visibility_init: Vec<f64>,
        intensity : f64,
        alfa : f64,
        wisdom : f64
    )-> Self{
        Self {
            pherohormones : vec![0.0; n] ,
            visibility : visibility_init.clone() ,
            evaporation :   intensity * -(0.2_f64).exp(),
            pherohormones_intensity : intensity,
            n_tasks : n,
            visibility_sum: 0.0,
            pherohormones_sum: 0.0,
            a : alfa,
            w : wisdom
        }
    }
    pub fn  update_visibility_sum(&mut self){
        for i in 0..self.visibility.len(){
            self.visibility_sum += self.visibility[i];
        }
    }
    pub fn update_pherohormone_sum(&mut self){
        for i in 0..self.pherohormones.len(){
            self.pherohormones_sum += self.pherohormones[i];
        }
    }
    pub fn add_pherohormones(
        &mut self,
        iteration : i128,
        task: usize
    ){ 
 //toda task completada insere um vertice com peso proposcional a 1/n_tasks 
    let deposit = self.pherohormones_intensity * (-(iteration as f64 ) / (self.n_tasks as f64 / 5.0)).exp();
    println!("adding pherohormone {} iteration {}",deposit,iteration);
    self.pherohormones[task] += deposit;
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