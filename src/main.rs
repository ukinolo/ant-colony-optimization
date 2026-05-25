mod graph;
mod aco;

use std::time::Instant;

use crate::{graph::UndirectedGraph, aco::Aco};

fn main() {
    let mut times: Vec<i32> = Vec::with_capacity(20);
    for _ in 0..20{
        let file_path = "./input_20_cities.txt";
        
        let graph = UndirectedGraph::load_graph(file_path, 20);
    
        let alpha = 1.0; // factor of distance
        let beta = 2.0; // factor of pheromones
        let evaporation_rate = 0.2;
        let number_of_ants = 200;
        let number_of_iterations = 1000;
    
        let mut aco = Aco::new(graph.num_of_nodes as i32, alpha, beta, evaporation_rate, number_of_ants, number_of_iterations, graph);
        
        let instant = Instant::now();
    
        // aco.run();
        aco.run_parallel(8);
    
        let time = instant.elapsed();
        println!("It took {:} ms to run!", time.as_millis());
        times.push(time.as_millis() as i32);
    }
    println!("Average run took {:} ms to run!", times.iter().sum::<i32>()/20);
}
