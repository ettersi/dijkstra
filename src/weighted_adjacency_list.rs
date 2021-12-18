// Adjacency list representation of a graph with edge weights
#[derive(Debug)]
pub struct WeightedAdjacencyList<W> {
    v2uw_vec: Vec<Vec<(usize,W)>>,
}

impl<W> WeightedAdjacencyList<W> {
    pub fn new(nv: usize) -> WeightedAdjacencyList<W> {
        let mut v2uw_vec = Vec::with_capacity(nv);
        for _ in 0..nv {
            v2uw_vec.push(Vec::new());
        }
        return WeightedAdjacencyList{v2uw_vec};
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        self.v2uw_vec[from].push((to,weight));
    }

    pub fn nv(&self) -> usize { return self.v2uw_vec.len(); }

    pub fn outgoing(&self, v: usize) -> &Vec<(usize,W)> {
        return &self.v2uw_vec[v];
    }
}

use itertools::Itertools;
impl<W: std::fmt::Display> std::fmt::Display for WeightedAdjacencyList<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Weighted adjacency list:")?;
        for (v,uw_vec) in self.v2uw_vec.iter().enumerate() {
            writeln!(f,
                "  {:>3} -> {}",
                v,
                uw_vec.iter().format_with(", ", |(u,w),f| f(&format_args!("{:>3} | {:>3}", u,w)))
            )?;
        }
        return std::fmt::Result::Ok(())
    }
}