#![allow(non_snake_case)]

mod graphs;
mod searching;

fn main() {
    println!("Hello, world!");
    let gr = graphs::Graph::get_graph(graphs::graph_type::unweighted, 12);
    gr.print_graph();

    println!("Breadth_First_Search!!");
    searching::informedSeaching::BreadthFirstSearch::BFS(&gr, 5);
    println!("\n\n");
    println!("Depth_First_Search!!");
    searching::informedSeaching::DepthFirstSearch::DFS(&gr, 5);
    println!("\n\n");
}
