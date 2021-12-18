// Helper macros to load and use modules in one go
macro_rules! mod_use {
    ($filename:ident) => {
        mod $filename;
        use $filename::*;
    }
}
macro_rules! mod_pub_use {
    ($filename:ident) => {
        mod $filename;
        pub use $filename::*;
    }
}

// Imports
mod_pub_use!(weighted_adjacency_list);
mod_use!(int_or_float);
mod_use!(dijkstra_heap);

use itertools::Itertools;



// Result type for dijkstra(...)
#[derive(Debug)]
pub struct PathWithLength<W> {
    path: Vec<usize>,
    length: W,
}

impl<W: std::fmt::Display> std::fmt::Display for PathWithLength<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
            "Path of length {}: {}",
            self.length,
            self.path.iter().format(" -> ")
        )?;
        return std::fmt::Result::Ok(())
    }
}


// Main algorithm
pub fn dijkstra<W: IntOrFloat>(g: &WeightedAdjacencyList<W>, a: usize, b: usize) -> PathWithLength<W> {
    // Allocate workspace
    let n = g.nv();
    let mut d = DijkstraHeap::new(n);
    let mut from = vec![usize::MAX; n];

    // Run Dijkstra
    d.update(a, W::zero());
    while let Some(v) = d.pop() {
        if v == b { break; }
        let dv = d[v];
        for &(u,w) in g.outgoing(v) {
            if d[u] > dv + w {
                d.update(u, dv+w);
                from[u] = v;
            }
        }
    }

    // Extract shortest path and its length
    let length = d[b];
    let mut path = Vec::new();
    let mut v = b;
    while v != usize::MAX {
        path.push(v);
        v = from[v];
    }
    path.reverse();

    return PathWithLength{ path, length }
}
