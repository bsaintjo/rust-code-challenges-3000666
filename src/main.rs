use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut costs: HashMap<Node, Cost> = HashMap::new();
    let mut parents: HashMap<Node, Node> = HashMap::new();
    let mut processed: HashSet<Node> = HashSet::new();

    processed.insert(start);

    for &(n, c) in g.edges.get(&start).unwrap().iter() {
        costs.insert(n, c);
        parents.insert(n, start);
    }
    costs.insert(goal, usize::MAX);

    while let Some(node) = find_lowest_cost_node(&costs, &processed) {
        let cost = costs[&node];
        for &(neighbor_node, neighbor_cost) in g.edges.get(&node).unwrap().iter() {
            let new_cost = cost + neighbor_cost;
            if !costs.contains_key(&neighbor_node) || costs[&neighbor_node] > new_cost {
                costs.insert(neighbor_node, new_cost);
                parents.insert(neighbor_node, node);
            }
        }
        processed.insert(node);
    }

    let mut acc = vec![goal];
    let total = costs[&goal];
    parents.get(&goal).and_then(|p| {
        let mut p = *p;
        while let Some(&new_par) = parents.get(&p) {
            acc.push(new_par);
            p = new_par;
            if p == start {
                break;
            }
        }
        if p == start {
            Some((acc, total))
        } else {
            None
        }
    })
}

fn find_lowest_cost_node(costs: &HashMap<Node, Cost>, processed: &HashSet<Node>) -> Option<Node> {
    costs
        .iter()
        .filter(|(node, _)| !processed.contains(node))
        .min_by_key(|(_, &cost)| cost)
        .map(|(&node, _)| node)
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
