use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug)]
enum Color {
    White,
    Gray,
    Black,
}

pub(crate) struct Graph {
    adj_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub(crate) fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub(crate) fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list.entry(u).or_insert_with(Vec::new).push(v);
        // Ensure all nodes are in the adj_list, even if they have no outgoing edges
        self.adj_list.entry(v).or_insert_with(Vec::new);
    }

    pub(crate) fn detect_cycle(&self) -> Option<Vec<usize>> {
        let mut color = HashMap::new();
        let mut parent = HashMap::new();

        for &vertex in self.adj_list.keys() {
            color.insert(vertex, Color::White);
            parent.insert(vertex, None);
        }

        for &vertex in self.adj_list.keys() {
            if let Color::White = color[&vertex] {
                if let Some(cycle) = self.dfs_visit_cycle(vertex, &mut color, &mut parent) {
                    return Some(cycle);
                }
            }
        }

        None
    }

    fn dfs_visit_cycle(
        &self,
        u: usize,
        color: &mut HashMap<usize, Color>,
        parent: &mut HashMap<usize, Option<usize>>,
    ) -> Option<Vec<usize>> {
        color.insert(u, Color::Gray);

        if let Some(neighbors) = self.adj_list.get(&u) {
            for &v in neighbors {
                if let Color::White = color[&v] {
                    parent.insert(v, Some(u));
                    if let Some(cycle) = self.dfs_visit_cycle(v, color, parent) {
                        return Some(cycle);
                    }
                } else if let Color::Gray = color[&v] {
                    // Back edge found, construct cycle
                    let mut cycle = VecDeque::new();
                    cycle.push_front(u);
                    let mut current = u;
                    while let Some(p) = parent[&current] {
                        cycle.push_front(p);
                        current = p;
                        if p == v {
                            break;
                        }
                    }
                    cycle.push_front(v);
                    return Some(cycle.into_iter().collect());
                }
            }
        }

        color.insert(u, Color::Black);
        None
    }
}