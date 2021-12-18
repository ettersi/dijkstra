use dijkstra::*;

fn main() {
    let mut g = WeightedAdjacencyList::new(4);
    g.add_edge(0,1,3);
    g.add_edge(0,2,4);
    g.add_edge(1,2,2);
    g.add_edge(2,3,4);
    println!("{}", g);

    let p = dijkstra(&g,0,3);
    println!("{}", p);
}