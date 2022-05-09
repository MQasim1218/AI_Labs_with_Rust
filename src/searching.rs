#![allow(unused_variables)]
#![allow(dead_code)]

pub mod informedSeaching {
    pub mod BreadthFirstSearch {
        pub fn BFS(graph: &crate::graphs::Graph, st_node: u16) {
            let mut curr = st_node;
            let (graph, goal) = graph.get_graph_vars();
            println!("The Gaol value is {}", goal);

            let mut frontier: Vec<u16> = Vec::new();
            let mut exp: Vec<u16> = Vec::new();

            frontier.push(curr);
            while frontier.len() > 0 {
                curr = frontier.remove(0);
                exp.push(curr);

                if graph[&curr].node_val == goal {
                    println!(
                        "The goal Node has been found!! => {}",
                        &graph[&curr].node_val
                    );
                    break;
                } else {
                    for nbr in &graph[&curr].actions {
                        let nbr = match *nbr {
                            crate::graphs::act_type::unweighted(indx) => indx,
                            crate::graphs::act_type::wieghted(indx, _) => indx,
                        };
                        if !(exp.contains(&nbr) || frontier.contains(&nbr)) {
                            frontier.push(nbr); // Push the neighbour on to the frontier
                        }
                    }
                }
                println!("Explored: {:?}", exp);
                println!("Frontier: {:?}", frontier);
                println!();
            }
        }
    }

    pub mod DepthFirstSearch {
        pub fn DFS(graph: &crate::graphs::Graph, st_node: u16) {
            let mut curr = st_node;
            let (graph, goal) = graph.get_graph_vars();
            println!("The Gaol value is {}", goal);

            let mut frontier: Vec<u16> = Vec::new();
            let mut exp: Vec<u16> = Vec::new();

            frontier.push(curr);
            while frontier.len() > 0 {
                curr = frontier.pop().unwrap();
                exp.push(curr);

                if graph[&curr].node_val == goal {
                    println!(
                        "The goal Node has been found!! => {}",
                        &graph[&curr].node_val
                    );
                    break;
                } else {
                    for nbr in &graph[&curr].actions {
                        let nbr = match *nbr {
                            crate::graphs::act_type::unweighted(indx) => indx,
                            crate::graphs::act_type::wieghted(indx, _) => indx,
                        };
                        if !(exp.contains(&nbr) || frontier.contains(&nbr)) {
                            frontier.push(nbr); // Push the neighbour on to the frontier
                        }
                    }
                }
                println!("Explored: {:?}", exp);
                println!("Frontier: {:?}", frontier);
                println!();
            }
        }
    }

    pub mod UninformCostSearch {}
}

mod UninformedSearching {
    mod AStar {}

    mod BestFirstSearch {}

    mod HillClimbingWithRnadomRestart {}
}
