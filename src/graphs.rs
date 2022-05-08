#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::collections::HashMap;

enum graph_type {
    wieghted,
    unweighted,
    maze,
}

enum action_type {
    wieghted,
    unweighted,
}
struct Node {
    state: u16,
    actions: Vec<u16>,
    par: Option<Box<Node>>,
}

impl Node {
    fn new(state: u16, actions: Vec<u16>) -> Node {
        Node {
            state,
            actions,
            par: None,
        }
    }
}

struct Graph {
    num_Nodes: u16,
    graph: HashMap<u16, Node>,
}

impl Graph {
    fn MakeWeiGraph(num_Nodes: u16) -> Graph {
        let graph: HashMap<u16, Node> = HashMap::new();

        Graph {
            num_Nodes,
            graph: graph,
        }
    }

    fn MakeUnWeiGraph(num_Nodes: u16) -> Graph {
        let graph: HashMap<u16, Node> = HashMap::new();

        Graph {
            num_Nodes,
            graph: graph,
        }
    }

    fn get_graph(gt: graph_type, num_Nodes: u16) -> Graph {
        match gt {
            graph_type::wieghted => Graph::MakeWeiGraph(num_Nodes),
            graph_type::unweighted => Graph::MakeUnWeiGraph(num_Nodes),
            graph_type::maze => todo!(),
        }
    }
}

// import random

// class Node:
//     def __init__(self, st):
//         self.state = st
//         self.nbrs = []
//         self.node_val = random.randint(100, 501)

// class Graph:
//     def __init__(self):
//         self.graph = {}
//         self.max_val = -1

//     def makeGraph(self):
//         for i in range(25):
//             node = Node(i + 1)
//             if self.max_val < node.node_val:
//                 self.max_val = node.node_val

//             for j in range(3):
//                 node.nbrs.append(random.randint(1, 25))
//             self.graph[i+1] = node
//         print(self.max_val)
//         ...

//     def printGraph(self):
//         keys = self.graph.keys()
//         for k in keys:
//             print("Node_state: ", k, '\nNode_val:', self.graph[k].node_val)
//             print("Nbr_st\tValue")
//             for nbr in self.graph[k].nbrs:
//                 print(self.graph[nbr].state, '\t\t', self.graph[nbr].node_val)
//             print('\n')
