use crate::utils::sparse_matrix::SparseMatrix;
use std::collections::HashMap;


pub struct Ant {
    pub visited: HashMap<usize, bool>,
    pub path: Vec<usize>,
}

impl Ant {
    pub fn new() -> Ant {
        Ant { visited: HashMap::new(), path: vec!() }
    }

    pub fn find_circuit(&mut self, start: usize, graph: &SparseMatrix, pheromone: &SparseMatrix, alpha: f64, beta: f64) {
        self.visited.insert(start, true);
        self.path.push(start);

        for _ in 0..(graph.size() - 1) {
            let n = self.find_next(*self.path.last().unwrap(), graph, pheromone, alpha, beta);
            self.visited.insert(n, true);
            self.path.push(n);
        }

        self.path.push(start);
    }

    pub fn find_next(&mut self, v: usize, graph: &SparseMatrix, pheromone: &SparseMatrix, alpha: f64, beta: f64) -> usize {
        let mut n = graph.neighboors_of(v);
        self.remove_visiteds(&mut n);
        if n.len() == 1 {
            return *n.last().unwrap();
        }

        let probs = self.calc_probs(v, &n, graph, pheromone, alpha, beta);

        let r = rand::random::<f64>();
        let first = probs.first().unwrap();
        if r < first.1 {
            return first.0
        }

        let mut i = 0;
        let mut p_acc = first.1;
        for j in 1..probs.len() {
            let &(_, pj) = probs.get(j).unwrap();
            p_acc += pj;

            if r < (p_acc) { break; }

            i += 1;
        }
       

        probs.get(i).unwrap().0
    }

    fn remove_visiteds(&self, neighbors: &mut Vec<usize>) {
        neighbors.retain(|x| !is_included(x, &self.visited))
    }

    fn calc_edge_probability(&self, i: usize, j :usize, graph: &SparseMatrix, pheromone: &SparseMatrix, alpha: f64, beta: f64) -> f64 {
        let d = graph.get(i, j).unwrap();
        let p = pheromone.get(i, j). unwrap();

        p.powf(alpha) * d.recip().powf(beta)
    }

    fn calc_probs(&self, u: usize, n: &Vec<usize>, graph: &SparseMatrix, pheromone: &SparseMatrix, alpha: f64, beta: f64) -> Vec<(usize, f64)> {
        let mut vec: Vec<(usize, f64)> = vec!();

        for v in n.iter() {
            let p = self.calc_edge_probability(u, *v, graph, pheromone, alpha, beta);
            vec.push((*v, p));
        }

        let s = vec.iter().fold(0f64, |acc, (_, p)| acc + p);
        for (_, p) in vec.iter_mut() {
            *p /= s;
        }
        vec.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        vec
    }
}

fn is_included(x: &usize, hm: &HashMap<usize, bool>) -> bool {
    match hm.get(x) {
        Some(true) => true,
        _ => false,
    }
}