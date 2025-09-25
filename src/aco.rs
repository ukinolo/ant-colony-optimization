use std::thread;

use crate::graph::{encode, UndirectedGraph};

use rand::{distr::weighted::WeightedIndex, prelude::*};

const PH_DEF: f64 = 1.0;

#[derive(Debug)]
pub struct Aco{
    pheromones: Vec<f64>,
    num_of_nodes: i32,
    alpha: f64,
    beta: f64,
    evaporation_rate: f64,
    num_of_ants: i32,
    num_of_iterations: i32,
    graph: UndirectedGraph,
}

impl Aco{
    pub fn new(num_of_nodes: i32, alpha: f64, beta: f64, evaporation_rate: f64, number_of_ants: i32, number_of_iterations:i32, graph: UndirectedGraph) -> Self {
        Self { 
            num_of_nodes,
            pheromones: vec![PH_DEF; ((num_of_nodes - 1) * (num_of_nodes) / 2) as usize],
            alpha,
            beta,
            evaporation_rate,
            num_of_ants: number_of_ants,
            num_of_iterations: number_of_iterations,
            graph,
        }
    }

    // runs iteration of one ant starting from start_id node
    // returns the order of nodes and length
    pub fn run_ant(&self, start_id: i32) -> AntPath{
        let mut res = vec![start_id; (self.num_of_nodes + 1) as usize];
        
        let mut rng = rand::rng();
        
        let mut to_visit: Vec<i32> = (0..self.num_of_nodes).filter(|id| *id != start_id).collect();

        let mut length = 0.0;
        let mut prev_id = start_id;

        for i in 1..self.num_of_nodes {

            let mut e: Vec<f64> = vec![]; //e[j] = edge value between i and j
            let mut p: Vec<f64> = vec![]; //p[j] = pheromone value between i and j
            
            for &j in to_visit.iter(){
                let edge_value = self.graph.get_edge_value(prev_id, j);
                let pheromone_value = self.pheromones[encode(prev_id, j, self.num_of_nodes)];
                e.push(edge_value);
                p.push(pheromone_value);
            }
            
            // println!("Pheromone values are: {:?}", p);
            // println!("Path values are: {:?}", e);
            
            //Select a value
            let probabilities: Vec<f64> = e.iter().zip(p.iter()).map(|(&edge, &pheromone)| self.combine_values(edge, pheromone)).collect();

            // println!("{:?}", probabilities);
            let dist = WeightedIndex::new(&probabilities).unwrap();
            let index = dist.sample(&mut rng);
            let next_node_id = to_visit[index];
            prev_id = next_node_id;

            to_visit.remove(index);
            length += e[index];
            res[i as usize] = next_node_id;
        }
        length += self.graph.get_edge_value(prev_id, start_id);
        return AntPath::new(res, length);
    }

    pub fn get_pheromones_diff(&self, ant_run: AntPath) -> Vec<f64> {
        let pheromone_to_deposit = 1.0 / ant_run.length.powf(6.0) * 2000.0;
        // println!("{:}: length: {:}", pheromone_to_deposit, ant_run.length);
        let mut pheromone_diff = vec![0.0; ((self.num_of_nodes - 1) * (self.num_of_nodes) / 2) as usize];
        for window in ant_run.node_ids.windows(2){
            pheromone_diff[encode(window[0], window[1], self.num_of_nodes)] = pheromone_to_deposit;
        }
        pheromone_diff
    }

    pub fn update_pheromone(&mut self, pheromone_diffs: Vec<Vec<f64>>) {
        for diff in pheromone_diffs{
            for (i, v) in diff.into_iter().enumerate(){
                self.pheromones[i] += v;
            }
        }
    }

    fn evaporate_pheromones(&mut self) {
        for v in self.pheromones.iter_mut(){
            *v *= 1.0 - self.evaporation_rate;
        }
    }

    fn reset_pheromones(&mut self) {
            for v in self.pheromones.iter_mut(){
            *v *= PH_DEF;
        }
    }

    pub fn run(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..self.num_of_iterations{
            let mut paths: Vec<AntPath> = Vec::with_capacity(self.num_of_ants as usize);
            for _ in 0..self.num_of_ants{
                paths.push(self.run_ant(rng.random_range(0..self.num_of_nodes)));
            }

            let pheromone_diffs: Vec<Vec<f64>> = paths.into_iter().map(|path| self.get_pheromones_diff(path)).collect();

            self.evaporate_pheromones();
            self.update_pheromone(pheromone_diffs);
            self.get_pheromone_path();
        }
    }

    pub fn run_parallel(&mut self, num_of_threads: i32) where Self: Sync
    {
        
        for _ in 0..self.num_of_iterations{
            let mut all_pheromone_diffs: Vec<Vec<Vec<f64>>> =
                Vec::with_capacity(num_of_threads as usize);
            
            let self_ref: &Self = &*self;

            thread::scope(|scope| {
    
                let mut handles = Vec::new();
                
                for i in 0..num_of_threads{
                    let idx = i;
                    handles.push(scope.spawn(move || {
                        let mut local_paths: Vec<AntPath> = Vec::new();
                        let mut j = idx;
                        let mut rng = rand::rng();
                        while j < self_ref.num_of_ants {
                            local_paths.push(
                                self_ref.run_ant(rng.random_range(0..self_ref.num_of_nodes))
                            );
                            j += num_of_threads;
                        }
                        
                        local_paths
                            .into_iter()
                            .map(|path| self_ref.get_pheromones_diff(path))
                            .collect::<Vec<Vec<f64>>>()
                    }));
                }
                
                for handle in handles.into_iter() {
                    all_pheromone_diffs.push(handle.join().unwrap());
                }
            });
    
            self.evaporate_pheromones();
            for pheromone_diff in all_pheromone_diffs{
                self.update_pheromone(pheromone_diff);
            }
            self.get_pheromone_path();
        }
    }

    // gets the path with the most pheromone on it
    fn get_pheromone_path(&self) {
        let mut res = vec![0; (self.num_of_nodes + 1) as usize];

        let mut to_visit: Vec<i32> = (1..self.num_of_nodes).collect();
        let mut length = 0.0;
        let mut prev_id = 0;

        for i in 1..self.num_of_nodes {
            let mut greatest_pheromone = 0.0;
            let mut greatest_pheromone_id = 0;

            for (index, &j) in to_visit.iter().enumerate() {
                let pheromone_value = self.pheromones[encode(prev_id, j, self.num_of_nodes)];
                if pheromone_value > greatest_pheromone {
                    greatest_pheromone = pheromone_value;
                    greatest_pheromone_id = index;
                }
            }

            let next_node_id = to_visit[greatest_pheromone_id as usize];
            
            to_visit.remove(greatest_pheromone_id);
            length += self.graph.get_edge_value(prev_id, next_node_id);
            prev_id = next_node_id;
            res[i as usize] = next_node_id;
        }

        println!("The fastest current path is {:?}, with length {:}", res, length);
    }
    
    fn combine_values(&self, edge_value: f64, pheromone_value: f64) -> f64 {
        (1.0 / edge_value).powf(self.beta) * pheromone_value.powf(self.alpha)
    }

    fn print_pheromons(&self){
        for i in 0..self.num_of_nodes - 1{
            print!("\n[ ");
            for _ in 0..=i{
                print!(", {:.3}", 0.0);
            }
            for j in i+1..self.num_of_nodes{
                print!(", {:.3}", self.pheromones[encode(i, j, self.num_of_nodes)]);
            }
            print!("]")
        }
        println!();
    }
}

pub struct AntPath {
    pub node_ids: Vec<i32>,
    pub length: f64,
}

impl AntPath {
    pub fn new(node_ids: Vec<i32>, length: f64) -> Self {
        Self { node_ids, length }
    }
}
