#![allow(dead_code)]
#![allow(non_camel_case_types)]

use rand::Rng;
use std::collections::HashMap;

enum action_type {
    unweighted(u16),    // Reference to the key to a Node in the graph (HashMap)
    wieghted(u16, u16), // Reference to the key and distance to a Node in the graph (HashMap).
}

enum node_state {
    normal(u16, u8),        // node_state and self_cost
    heuristic(u16, u8, u8), // node_state, self_cost and heuristic
}

// !! This code shall be used laterr to generify the code!!
/*
    struct Node {
        state: node_state,
        actions: Vec<action_type>,
        par: Option<Box<Node>>,
    }
    impl Node {
        fn new(state: node_state, actions: Vec<action_type>) -> Node {
            Node {
                state,
                actions,
                par: None,
            }
        }
    }
*/

// This Node impl is for proof of concept that our core stuff works!!
// Latrr to be replaced by the above impl!!
pub struct Node {
    state: u16,
    pub node_val: u16,
    pub actions: Vec<u16>,
    par: Option<Box<Node>>,
}
impl Node {
    fn new(state: u16, node_val: u16, actions: Vec<u16>) -> Node {
        Node {
            state,
            node_val,
            actions,
            par: None,
        }
    }
}

pub enum graph_type {
    wieghted,
    unweighted,
    maze,
}
pub struct Graph {
    num_Nodes: u16,
    pub graph: HashMap<u16, Node>,
    max_val: u16, // As a suitable goal Node, especially in the hill-climbing algorithm!!
}

impl Graph {
    // Make a simple Hardcoded graph!!
    /*
        let map = HashMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
    */

    fn MakeUnWeiGraph(num_Nodes: u16) -> Graph {
        let mut graph: HashMap<u16, Node> = HashMap::new();
        let mut mx: u16 = 0;

        for i in 1..=num_Nodes {
            let nv = rand::thread_rng().gen_range(0..=100);
            let mut nbrs: Vec<u16> = Vec::new();

            // A fancy for loop to assign random nbrs to a Node!!
            (0..3).for_each(|_| {
                let node_indx = rand::thread_rng().gen_range(1..=num_Nodes);
                nbrs.push(node_indx);
            });

            let node = Node::new(i, nv, nbrs);
            graph.insert(i, node);

            if nv > mx {
                mx = nv
            };
        }

        Graph {
            num_Nodes,
            graph: graph,
            max_val: mx,
        }
    }

    fn MakeWeiGraph(num_Nodes: u16) -> Graph {
        let graph: HashMap<u16, Node> = HashMap::new();

        Graph {
            num_Nodes,
            graph: graph,
            max_val: 0,
        }
    }

    pub fn print_graph(&self) {
        /*
            keys = self.graph.keys()
            for k in keys:
                print("Node_state: ", k, '\nNode_val:', self.graph[k].node_val)
                print("Nbr_st\tValue")
                for nbr in self.graph[k].nbrs:
                    print(self.graph[nbr].state, '\t\t', self.graph[nbr].node_val)
                print('\n')
        */

        // Basic printing for unweighted, directed graph
        println!("Printing Graph!!");
        for key in self.graph.keys() {
            println!("Node_state: {}", self.graph[key].state);
            println!("Node_value: {}", self.graph[key].node_val);
            print!("Neighbours: ");
            for i in &self.graph[key].actions {
                print!("{}\t", *i);
            }
            println!("\n");
        }
        println!("\nEnd of Graph!!\n");
    }

    pub fn get_mx(&self) -> (&HashMap<u16, Node>, u16) {
        (&self.graph, self.max_val)
    }

    pub fn get_graph(gt: graph_type, num_Nodes: u16) -> Graph {
        match gt {
            graph_type::wieghted => Graph::MakeWeiGraph(num_Nodes),
            graph_type::unweighted => Graph::MakeUnWeiGraph(num_Nodes),
            graph_type::maze => todo!(),
        }
    }
}

// Pythoin Code!!
/*
    import random
    class Node:
        def __init__(self, st):
            self.state = st
            self.nbrs = []
            self.node_val = random.randint(100, 501)
    class Graph:
        def __init__(self):
            self.graph = {}
            self.max_val = -1
        def makeGraph(self):
            for i in range(25):
                node = Node(i + 1)
                if self.max_val < node.node_val:
                    self.max_val = node.node_val
                for j in range(3):
                    node.nbrs.append(random.randint(1, 25))
                self.graph[i+1] = node
            print(self.max_val)
            ...
        def printGraph(self):
            keys = self.graph.keys()
            for k in keys:
                print("Node_state: ", k, '\nNode_val:', self.graph[k].node_val)
                print("Nbr_st\tValue")
                for nbr in self.graph[k].nbrs:
                    print(self.graph[nbr].state, '\t\t', self.graph[nbr].node_val)
                print('\n')
*/
