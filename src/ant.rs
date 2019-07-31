use crate::utils::sparse_matrix::SparseMatrix;
use std::collections::HashMap;


pub struct Ant<'a> {
    graph: &'a SparseMatrix,
    visited: HashMap<usize, bool>,
    pub path: Vec<usize>,
}

impl<'a> Ant<'a> {
    pub fn new(graph: &'a SparseMatrix) -> Ant {
        Ant { graph, visited: HashMap::new(), path: vec!() }
    }

    pub fn find_circuit(&mut self, start: usize) {
        self.visited.insert(start, true);
        self.path.push(start);

        for _ in 0..(self.graph.size() - 1) {
            let n = self.find_next(*self.path.last().unwrap());
            self.visited.insert(n, true);
            self.path.push(n);
        }
    }

    pub fn find_next(&mut self, vertex: usize) -> usize {
        let mut n = self.graph.neighboors_of(vertex);
        self.remove_visiteds(&mut n);

        let i: usize = (rand::random::<u64>() as usize) % n.len();

        *n.get(i).unwrap()
    }

    fn remove_visiteds(&self, neighbors: &mut Vec<usize>) {
        neighbors.retain(|x| !is_included(x, &self.visited))
    }
}

fn is_included(x: &usize, hm: &HashMap<usize, bool>) -> bool {
    match hm.get(x) {
        Some(true) => true,
        _ => false,
    }
}