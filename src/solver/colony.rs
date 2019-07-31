use std::collections::HashMap;
use crate::utils::sparse_matrix::SparseMatrix;
use crate::solver::ant::Ant;


pub struct Colony {
    alpha: f64,
    beta: f64,
    rho: f64,
    ants: Vec<Ant>,
    iters: usize,
    graph: SparseMatrix,
    pheromone: SparseMatrix,
}

impl Colony{
    pub fn new(instance: tsplib::Instance, n_ants: usize, iters: usize, alpha: f64, beta: f64, rho: f64) -> Colony {
        let graph = SparseMatrix::new_from_instace(instance);

        let s = graph.size();
        let mut pheromone = SparseMatrix::new(s, s);
        for i in 0..s {
            for j in 0..(i+1) {
                pheromone.set(i, j, 1f64);
            }
        }

        let mut ants: Vec<Ant> = vec!();
        for _ in 0..n_ants {
            ants.push(Ant::new());
        }

        Colony { alpha, beta, rho, graph, pheromone, ants, iters }
    }

    pub fn run(&mut self, start: usize) -> HashMap<usize, Vec<f64>> {
        let mut results: HashMap<usize, Vec<f64>> = HashMap::new(); 

        for i in 0..self.iters {
            info!("Iter: {}", i);

            let mut distances: Vec<f64> = vec!();
            for ant in self.ants.iter_mut() {
                ant.path.clear();
                ant.visited.clear();
                ant.find_circuit(start, &self.graph, &self.pheromone, self.alpha, self.beta);
                trace!("ant done");
            }
            trace!("all ants are at home now");

            for ant in self.ants.iter() {
                let d = calc_distance(&self.graph, &ant.path);
                distances.push(d);
                update_pheromone(&mut self.pheromone, &ant.path, self.rho);
            }
            trace!("pheromone matrix updated");

            results.insert(i, distances);
        }

        results
    }
}

fn update_pheromone(pheromone: &mut SparseMatrix, path: &Vec<usize>, rho: f64) {
    for i in 1..path.len() {
        let &u = path.get(i).unwrap();
        let &v = path.get(i-1).unwrap();

        pheromone.set(u, v, pheromone.get(u, v).unwrap() + 1f64);
    }
    
    for i in 0..pheromone.size() {
        for j in 0..(i+1) {
            pheromone.set(i, j, pheromone.get(i, j).unwrap() * rho);
        }
    }
}

fn calc_distance(graph: &SparseMatrix, path: &Vec<usize>) -> f64 {
    let mut sum = 0f64;
    for i in 1..path.len() {
        let &u = path.get(i).unwrap();
        let &v = path.get(i-1).unwrap();

        sum += graph.get(u, v).unwrap();
    }

    sum
}