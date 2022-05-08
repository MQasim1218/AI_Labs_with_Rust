#![allow(non_snake_case)]

mod graphs;
mod searching;

fn main() {
    println!("Hello, world!");
    let gr = graphs::Graph::get_graph(graphs::graph_type::unweighted, 12);
    gr.print_graph();
    searching::BestFirstSearch::
}
