
use std::fs;
use std::io::{BufRead, BufReader,Write, Error};
use std::fs::OpenOptions;

use petgraph::Direction;
use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};

pub(crate) struct Utils {
    pub(crate) di_graph: StableDiGraph<i128, i128>,
    pub(crate) remaining_vec: Vec<i128>,
    pub(crate) unlocks_vec: Vec<i128>,
    pub(crate) costs_vec: Vec<i128>,
    pub(crate) max_cost: i128,
    pub(crate) max_unlocks: i128
}

impl Utils {
    pub fn new() -> Self {
        Self {
            di_graph: StableDiGraph::<i128, i128>::new(),
            remaining_vec: Vec::new(),
            unlocks_vec: Vec::new(),
            costs_vec: Vec::new(),
            max_cost: 0 ,
            max_unlocks: 0
        }
    }
    pub fn show_content(file_path: &str) {
        //println!("In file {}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        println!("With text:\n{}", contents);
    }

    pub fn initialize_graph(&mut self, file_path: &str, task_graph : &str) {
        let path = format!("{}{}", file_path, task_graph);
        if let Ok(file) = fs::File::open(path) {
            let reader = BufReader::new(file);
            let mut count: i128 = 0;
            let mut task: i128 = 0;
            let mut line_count: i128 = 0;
            let mut n_tasks: i128 = 0;
            for line in reader.lines() {
                let line = line.expect("Failed to read line from file");
                //println!("{}", line);
                if line.starts_with("#") {
                    break;
                }
                let parsed_vec: Vec<i128> = line
                    .split_whitespace()
                    .map(|s| s.trim().parse::<i128>().expect("Invalid integer"))
                    .collect();

                for i in &parsed_vec {
                    if count == 0 {
                        n_tasks = *i + 2;
                        if line_count == 0 {
                            self.remaining_vec = vec![0; n_tasks as usize];
                            self.costs_vec = vec![0; n_tasks as usize];
                            self.unlocks_vec = vec![0; n_tasks as usize];
                            for j in 0..n_tasks {
                                self.di_graph.add_node(j);
                            }
                        } else {
                            //println!("Task: {}", i);
                            task = *i;
                            count += 1;
                        }
                    } else if count == 1 {
                        //println!("Cost: {}", i);
                        self.costs_vec[task as usize] = *i;
                        count += 1;
                    } else if count == 2 {
                        //println!("Degree: {}", i);
                        count += 1;
                    } else {
                        //println!(" {}", i);use std::io::Write
                        self.remaining_vec[task as usize] += 1;
                        self.di_graph.add_edge(
                            NodeIndex::new(*i as usize),
                            NodeIndex::new(task as usize),
                            0,
                        );
                    }
                }
                count = 0;
                line_count += 1;
            }
        } else {
            eprintln!("Error opening the file");
        }
        self.update_weights_unlocks();
    }

    pub fn update_weights_unlocks(&mut self) {
        let edge_indices: Vec<EdgeIndex> = self.di_graph.edge_indices().collect();

        for  edge in edge_indices {
            let (source, target) = self.di_graph.edge_endpoints(edge).unwrap();

            let target_index = target.index();

            let outgoing_edges =  self.di_graph.neighbors_directed(source, Direction::Outgoing).count();

            self.unlocks_vec[source.index()as usize] = outgoing_edges as i128;

            if let Some(&weight) = self.costs_vec.get(target_index) {
                self.di_graph.update_edge(source, target, weight);
            }
        }
    }
    pub fn print_graph(&self) {
        println!("Nodes in the graph:");
        for node in self.di_graph.node_indices() {
            println!("Node {}: {:?}", node.index(), self.di_graph[node]);
        }

        println!("Edges in the graph:");
        for edge in self.di_graph.edge_indices() {
            let (source, target) = self.di_graph.edge_endpoints(edge).unwrap();
            let weight = self.di_graph[edge];
            println!(
                "Edge from {} to {} with weight {}",
                source.index(),
                target.index(),
                weight
            );
        }
    }
    pub fn print_vecs(&self, n_tasks: usize) {
        println!("Task: \t Remainig: \t Cost : \t Unlocks :");
        for i in 0..n_tasks {
            println!(
                "{}        \t {}       \t{}       \t{}",
                i, self.remaining_vec[i], self.costs_vec[i], self.unlocks_vec[i]
            );
        
        }

        println!("max_cost :{} , max_unlocks :{}\n",self.max_cost, self.max_unlocks);
    }
    pub fn print_remaining_vec(&self, n_tasks: usize) {
        println!(" Remainig: :");
        for i in 0..n_tasks {
            println!(" {}",self.remaining_vec[i]);
        }
    }
    pub fn find_max_cost_unlocks(&mut self, n_tasks : usize) {
        let mut max_cost : i128 = -1;
        let mut max_unlocks : i128 = -1;
        for i in 0..n_tasks {
            if max_cost < self.costs_vec[i ]{
                max_cost = self.costs_vec[i];
            }
            if max_unlocks < self.unlocks_vec[i]{
                if i > 0{
                    max_unlocks = self.unlocks_vec[i];
                }
                    
            }
        }
        self.max_cost = max_cost;
        self.max_unlocks = max_unlocks;
    }
    
   

    pub fn write_results_to_file(
        &self,
        file_path: &str,
        graph_name: &str,
        sequence: &Vec<i128>,
        time_spent: &i128,
        n_ants: &i128,
    ) -> Result<(), Error> {
        let size = sequence.len();
        let path = format!("{}/{}{}", file_path, size, graph_name);
    
        let time_str = time_spent.to_string();
        let ants = n_ants.to_string();
    
        let content = format!(
            "\nnumber of processors: {}, number of tasks: {}\ntime spent: {}",
            ants, size, time_str
        );
    
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .write(true) 
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
    
        f.write_all(content.as_bytes())?;
        f.flush()?;
    
        Ok(())
    }
}
