#[cfg(test)]
mod graph_topological_sort_tests {
    use crate::sorting::graph_topological_sort::Graph;

    fn is_valid_topological_order(graph: &Graph, order: &Vec<String>) -> bool {
        let mut position = std::collections::HashMap::new();
        for (index, label) in order.iter().enumerate() {
            position.insert(label, index);
        }

        for edges in graph.from_edges.values() {
            for edge in edges {
                if position[&edge.from_vertex.label] > position[&edge.to_vertex.label] {
                    return false;
                }
            }
        }

        true
    }

    #[test]
    fn test_topological_sort_graph1() {
        let mut graph = Graph::new();
        let vertex_a = graph.add_vertex("A");
        let vertex_b = graph.add_vertex("B");
        let vertex_c = graph.add_vertex("C");
        let vertex_d = graph.add_vertex("D");
        let vertex_e = graph.add_vertex("E");
        let vertex_f = graph.add_vertex("F");
        let vertex_g = graph.add_vertex("G");

        graph.add_directed_edge(&vertex_a, &vertex_b, 1.0);
        graph.add_directed_edge(&vertex_a, &vertex_c, 1.0);
        graph.add_directed_edge(&vertex_b, &vertex_f, 1.0);
        graph.add_directed_edge(&vertex_c, &vertex_d, 1.0);
        graph.add_directed_edge(&vertex_d, &vertex_f, 1.0);
        graph.add_directed_edge(&vertex_e, &vertex_f, 1.0);
        graph.add_directed_edge(&vertex_e, &vertex_g, 1.0);
        graph.add_directed_edge(&vertex_f, &vertex_g, 1.0);

        let sorted = graph.topological_sort().unwrap();
        assert!(is_valid_topological_order(&graph, &sorted));
    }

    #[test]
    fn test_topological_sort_graph2() {
        let mut graph = Graph::new();
        let vertex_a = graph.add_vertex("A");
        let vertex_b = graph.add_vertex("B");
        let vertex_c = graph.add_vertex("C");
        let vertex_d = graph.add_vertex("D");
        let vertex_e = graph.add_vertex("E");
        let vertex_f = graph.add_vertex("F");
        let vertex_g = graph.add_vertex("G");

        graph.add_directed_edge(&vertex_a, &vertex_e, 1.0);
        graph.add_directed_edge(&vertex_b, &vertex_c, 1.0);
        graph.add_directed_edge(&vertex_c, &vertex_f, 1.0);
        graph.add_directed_edge(&vertex_c, &vertex_g, 1.0);
        graph.add_directed_edge(&vertex_d, &vertex_a, 1.0);
        graph.add_directed_edge(&vertex_d, &vertex_b, 1.0);
        graph.add_directed_edge(&vertex_e, &vertex_g, 1.0);
        graph.add_directed_edge(&vertex_f, &vertex_g, 1.0);

        let sorted = graph.topological_sort().unwrap();
        assert!(is_valid_topological_order(&graph, &sorted));
    }

    #[test]
    fn test_graph_with_cycle() {
        let mut graph = Graph::new();
        let vertex_a = graph.add_vertex("A");
        let vertex_b = graph.add_vertex("B");
        let vertex_c = graph.add_vertex("C");

        graph.add_directed_edge(&vertex_a, &vertex_b, 1.0);
        graph.add_directed_edge(&vertex_b, &vertex_c, 1.0);
        graph.add_directed_edge(&vertex_c, &vertex_a, 1.0);

        let result = graph.topological_sort();
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new();
        let sorted = graph.topological_sort().unwrap();
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_single_vertex() {
        let mut graph = Graph::new();
        graph.add_vertex("A");
        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted, vec!["A"]);
    }
}
