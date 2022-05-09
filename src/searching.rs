#![allow(unused_variables)]
#![allow(dead_code)]

pub mod informedSeaching {
    use crate::graphs::Graph;
    pub mod BreadthFirstSearch {
        pub fn BFS(graph: &super::Graph, st_node: u16) {
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
        pub fn DFS(graph: &super::Graph, st_node: u16) {
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

    pub mod UninformCostSearch {
        use crate::graphs::{act_type, node_state, Node};

        pub fn UFS(graph: super::Graph, st_node: u16) {
            let mut curr_indx = st_node;
            let (graph, goal) = graph.get_graph_vars();
            println!("The value of goal Node is {}", goal);

            let mut frontier_pq: Vec<&Node> = Vec::new();
            let mut exp: Vec<u16> = Vec::new();

            let mut curr = &graph[&curr_indx];
            // frontier_pq.push(curr);

            while frontier_pq.len() > 0 {
                curr = frontier_pq.remove(0);
                let curr_cost = match curr.state {
                    node_state::normal(_, cost) => cost,
                    _ => 0,
                };

                if curr.node_val == goal {
                    println!("The goal Node has been found!! => {}", curr.node_val);
                } else {
                    for nbr_action in &curr.actions {
                        let nbr = match *nbr_action {
                            act_type::wieghted(indx, dist) => (indx, dist),
                            act_type::unweighted(nbr_indx) => (nbr_indx, 0),
                        };

                        let nbr_node = &graph[&nbr.0];
                        if !(exp.contains(&nbr.0)) {
                            if frontier_pq.contains(&nbr_node) {
                                let curr_nbr_cost = match nbr_node.state {
                                    node_state::normal(_, cost) => cost,
                                    _ => 0,
                                };

                                if curr_nbr_cost > curr_cost + nbr.1 {
                                    // nbr_node.state = node_state::normal(nbr.0, curr_cost + nbr.1)
                                }
                            } else {
                                frontier_pq.push(&graph[&nbr.0]);
                                // &graph[&nbr.0].par = curr_indx;
                            }
                        }
                    }
                }
            }
            /*
                Algorithm:
                1. Start with the current Node check if its the goal Node
                2. If not the goal Node, add to explored and and explore its children
                3. Add children to a Priority queue based on their self_cost value
                4. Lower self_cost is added first
                5. Readjust the Parent Node of the
            */
        }
    }
}

mod UninformedSearching {
    mod AStar {}

    mod BestFirstSearch {}

    mod HillClimbingWithRnadomRestart {}
}
