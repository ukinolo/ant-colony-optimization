mod graph;
mod aco;

use crate::{graph::UndirectedGraph, aco::Aco};

fn main() {
    let file_path = "./input.txt";
    
    let graph = UndirectedGraph::load_graph(file_path, 20);

    let alpha = 1.0; // foctor of distance
    let beta = 2.0; //factor of pheromones
    let evaporation_rate = 0.2;
    let number_of_ants = 200;
    let number_of_iterations = 1000;

    let mut aco = Aco::new(graph.num_of_nodes as i32, alpha, beta, evaporation_rate, number_of_ants, number_of_iterations, graph);
    
    // aco.run();
    aco.run_parallel(8);

}
