use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct UndirectedGraph {
    pub num_of_nodes: usize,
    edge_values: Vec<f64>,
    pub mappings: HashMap<String, i32>,
    free_mapping_id: i32,
}

impl UndirectedGraph{
    pub fn new(num_of_nodes: usize) -> Self {
        Self { 
            num_of_nodes,
            edge_values: vec![0.0; (num_of_nodes - 1) * (num_of_nodes) / 2],
            mappings: HashMap::with_capacity(num_of_nodes),
            free_mapping_id: 0,
        }
    }

    pub fn load_graph(file_path: &str, num_of_nodes: i32) -> Self {
        let contents = fs::read_to_string(file_path).expect("Make sure input file exists");
        
        let lines: Vec<&str> = contents.lines().collect();
        
        let mut result = Self::new(num_of_nodes as usize);

        for line in lines {

            let mut info = line.split(',');
            let edge1 = info.next();
            info.next();// skip the country 
            let edge2 = info.next();
            info.next();// skip the country 
            let distance = info.next();

            result.add_edge(edge1.unwrap(), edge2.unwrap(), (distance.unwrap().parse::<i32>().unwrap()) as f64/1000.0);
        }
        
        return result;
    }

    fn add_edge(&mut self, edge1: &str, edge2: &str, val: f64) {
        let e1 = self.get_idx(edge1);
        let e2 = self.get_idx(edge2);

        self.edge_values[encode(e1, e2, self.num_of_nodes as i32)] = val;
    }

    // Given and indexes a and b, and the num of nodes, return the id of a path representing the connection between a and b
    // This is added because routes are not directed and a -> b = b -> a
    
    //get idx by node name, create idx if node name not indexed
    fn get_idx(&mut self, node_name: &str) -> i32 {
        match self.mappings.entry(node_name.to_string()) {
            std::collections::hash_map::Entry::Occupied(e) => *e.get(),
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(self.free_mapping_id);
                self.free_mapping_id += 1;
                self.free_mapping_id - 1
            }
        }
    }
    
    pub fn get_edge_value(&self, a: i32, b: i32) -> f64{
        self.edge_values[encode(a, b, self.num_of_nodes as i32)]
    }
    
    pub fn get_name(&self, node_idx: i32) -> String {
        self.mappings.iter()
            .filter(|(_, v)| **v == node_idx)
            .map(|(k, _)| k.clone())
            .next()
            .unwrap()
    }
}

pub fn encode(a: i32, b: i32, num_of_nodes: i32) -> usize {
    let l: i32;
    let h: i32;
    if a > b {
        h = a;
        l = b;
    } else {
        h = b;
        l = a;
    }
    (l * num_of_nodes - (l + 1) * l / 2 + (h - l - 1)) as usize
}