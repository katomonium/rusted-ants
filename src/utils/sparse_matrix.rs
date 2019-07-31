use std::collections::HashMap;

#[derive(Debug)]
pub struct SparseMatrix {
    n: usize,
    m: usize,
    data: HashMap<(usize, usize), f64>
}

#[allow(dead_code)]
impl SparseMatrix {
    pub fn new(n: usize, m: usize) -> SparseMatrix {
        let data = HashMap::new();

        SparseMatrix { n, m, data }
    }

    pub fn get(&mut self, i: usize, j: usize) -> Option<&f64> {
        if i > j {
            self.data.get(&(i, j))
        } else {
            self.data.get(&(j, i))
        }
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) -> Option<f64> {
        if i > j {
            self.data.insert((i, j), value)
        } else {
            self.data.insert((j, i), value)
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn neighboors_of(&self, index: usize) -> Vec<usize> {
        // TODO: Implementar para matrix nao completas
        let mut v: Vec<usize> = (0..self.n).collect();
        v.retain(|&x| x != index);
        v
    }

    pub fn new_from_instace(instace: tsplib::Instance) -> SparseMatrix {
        use tsplib::EdgeWeightType;

        let n = instace.dimension;
        let mut matrix: SparseMatrix = SparseMatrix::new(n, n);

        match instace.edge_weight_type.unwrap() {
            EdgeWeightType::Explicit => matrix.parse_explicit(instace.edge_weight.unwrap()),
            _ => {},
        };

        matrix
    }

    fn parse_explicit(&mut self, edges: tsplib::EdgeWeight) {
        use tsplib::EdgeWeight;

        match edges {
            EdgeWeight::LowerDiagRow(d) => self.parse_lower_diag_row(d),
            _ => {}
        };
    }

    fn parse_lower_diag_row(&mut self, data: Vec<usize>) {
        let mut x = 0;

        for i in 0..self.n {
            for j in 0..(i+1) {
                self.set(i, j, *data.get(x).unwrap() as f64);

                x += 1;
            }
        }
    }
}