use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};

pub(crate) struct Colony{
    n_ants: i128 ,
    pub(crate) pherormones: StableDiGraph<i128, f64>
}

impl Colony{

    pub fn new(n : i128 ){
        n_ants : n,
        pherormones : StableDiGraph::new()
    }

    pub fn initialize(){
 //copiar o grafo porem com ferormonios zerados
    }

    pub fn add_pherormones(){
 //toda task completada insere um vertice com peso proposcional a 1/n_tasks       
    }

    pub fn evaporate_pherormones()
    {
 // reduz o valor total de pherormonios em todas as rotas 
    }
}