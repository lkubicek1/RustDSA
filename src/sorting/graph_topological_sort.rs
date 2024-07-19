// sorting/graph_topological_sort.rs

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub label: String,
}

impl Vertex {
    pub fn new(label: &str) -> Self {
        Vertex {
            label: label.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from_vertex: Vertex,
    pub to_vertex: Vertex,
    pub weight: f64,
}

impl Edge {
    pub fn new(from_vertex: Vertex, to_vertex: Vertex, weight: f64) -> Self {
        Edge {
            from_vertex,
            to_vertex,
            weight,
        }
    }
}

pub struct Graph {
    pub(crate) from_edges: HashMap<String, Vec<Edge>>,
    to_edges: HashMap<String, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            from_edges: HashMap::new(),
            to_edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, label: &str) -> Vertex {
        let vertex = Vertex::new(label);
        self.from_edges.entry(label.to_string()).or_insert(Vec::new());
        self.to_edges.entry(label.to_string()).or_insert(Vec::new());
        vertex
    }

    pub fn add_directed_edge(&mut self, from: &Vertex, to: &Vertex, weight: f64) {
        let edge = Edge::new(from.clone(), to.clone(), weight);
        self.from_edges.get_mut(&from.label).unwrap().push(edge.clone());
        self.to_edges.get_mut(&to.label).unwrap().push(edge);
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, &'static str> {
        let mut result = Vec::new();
        let mut no_incoming = VecDeque::new();
        let mut incoming_count = HashMap::new();

        for (vertex, edges) in &self.to_edges {
            let count = edges.len();
            incoming_count.insert(vertex.clone(), count);

            if count == 0 {
                no_incoming.push_back(vertex.clone());
            }
        }

        let mut processed_count = 0;

        while let Some(current_vertex) = no_incoming.pop_front() {
            result.push(current_vertex.clone());
            processed_count += 1;

            if let Some(outgoing) = self.from_edges.get(&current_vertex) {
                for edge in outgoing {
                    let to_vertex = &edge.to_vertex.label;
                    let new_count = incoming_count.get(to_vertex).unwrap() - 1;
                    incoming_count.insert(to_vertex.clone(), new_count);

                    if new_count == 0 {
                        no_incoming.push_back(to_vertex.clone());
                    }
                }
            }
        }

        if processed_count != self.from_edges.len() {
            return Err("Graph contains a cycle");
        }

        Ok(result)
    }
}
