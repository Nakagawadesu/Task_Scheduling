use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};
use petgraph::Direction;
pub(crate) struct Colony{
    pub(crate) pherohormones: StableDiGraph::<i128, f64>,
    pub(crate) visibility: StableDiGraph::<i128, f64>,
    pub(crate) pherohormones_intensity:  f64,
    pub(crate) evaporation : f64
}

impl Colony{

    pub fn new(
        n_tasks : usize ,
        pherohormones_init : StableDiGraph::<i128, f64>,
        visibility_graph: StableDiGraph::<i128, f64>,
        intensity : f64
    )-> Self{
        Self {
            pherohormones : pherohormones_init ,
            visibility : visibility_graph,
            evaporation :   intensity * -(0.5_f64).exp(),
            pherohormones_intensity : intensity
        }
    }

    pub fn add_pherohormones(
        &mut self,
        iteration : i128,
        source: usize,
        target: usize,
        n_tasks : usize
    ){ 
        //a formiga depositará ferormonios conforme a funão 1*e^(-x/(n/2)) 
        //onde x é ai ésima tarefa sendo feita e n é o numero de tarefas
        //edge = EdgeIndex::new(task);
        let deposit = self.pherohormones_intensity * (-(iteration as f64 ) / (n_tasks as f64 / 2.0)).exp();

        //se ja a adjacencia ja existe

        let source_node = NodeIndex::new(source);
        let target_node = NodeIndex::new(target);

        if let Some(edge) = self.pherohormones.find_edge(source_node, target_node) {
            let weight = self.pherohormones.edge_weight(edge).unwrap_or(&0.0);
            let new_weight = &((*weight as f64 + deposit) as f64);
            self.pherohormones.update_edge(source_node, target_node, *new_weight);

        } else {
            
            self.pherohormones.add_edge(
                source_node,
                target_node,
                deposit,
            );
        }
        
    }

    pub fn evaporate_pherormones(&mut self)
    {
        // o ultimo vertice acessado sempre tem que zerar em pherormonios
        let edge_indices: Vec<EdgeIndex> = self.pherohormones.edge_indices().collect();

        for  edge in edge_indices {
            let (source, target) = self.pherohormones.edge_endpoints(edge).unwrap();

            let target_index = target.index();

            let outgoing_edges =  self.pherohormones.neighbors_directed(source, Direction::Outgoing).count();
            if let Some(weight) = self.pherohormones.edge_weight(edge) {
                let new_value = weight - self.evaporation;
                self.pherohormones.update_edge(source, target, new_value);
            }
            
        }
    }
    
}