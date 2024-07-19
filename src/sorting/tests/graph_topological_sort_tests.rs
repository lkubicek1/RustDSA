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

    #[test]
    fn test_complex_graph() {
        let mut graph = Graph::new();
        let vertex_m = graph.add_vertex("m");
        let vertex_n = graph.add_vertex("n");
        let vertex_o = graph.add_vertex("o");
        let vertex_p = graph.add_vertex("p");
        let vertex_q = graph.add_vertex("q");
        let vertex_r = graph.add_vertex("r");
        let vertex_s = graph.add_vertex("s");
        let vertex_t = graph.add_vertex("t");
        let vertex_u = graph.add_vertex("u");
        let vertex_v = graph.add_vertex("v");
        let vertex_w = graph.add_vertex("w");
        let vertex_x = graph.add_vertex("x");
        let vertex_y = graph.add_vertex("y");
        let vertex_z = graph.add_vertex("z");

        graph.add_directed_edge(&vertex_m, &vertex_q, 1.0);
        graph.add_directed_edge(&vertex_m, &vertex_r, 1.0);
        graph.add_directed_edge(&vertex_m, &vertex_x, 1.0);
        graph.add_directed_edge(&vertex_n, &vertex_q, 1.0);
        graph.add_directed_edge(&vertex_n, &vertex_u, 1.0);
        graph.add_directed_edge(&vertex_o, &vertex_r, 1.0);
        graph.add_directed_edge(&vertex_o, &vertex_v, 1.0);
        graph.add_directed_edge(&vertex_o, &vertex_s, 1.0);
        graph.add_directed_edge(&vertex_p, &vertex_o, 1.0);
        graph.add_directed_edge(&vertex_p, &vertex_s, 1.0);
        graph.add_directed_edge(&vertex_p, &vertex_z, 1.0);
        graph.add_directed_edge(&vertex_q, &vertex_t, 1.0);
        graph.add_directed_edge(&vertex_r, &vertex_u, 1.0);
        graph.add_directed_edge(&vertex_r, &vertex_y, 1.0);
        graph.add_directed_edge(&vertex_s, &vertex_r, 1.0);
        graph.add_directed_edge(&vertex_u, &vertex_t, 1.0);
        graph.add_directed_edge(&vertex_v, &vertex_x, 1.0);
        graph.add_directed_edge(&vertex_v, &vertex_w, 1.0);
        graph.add_directed_edge(&vertex_w, &vertex_z, 1.0);
        graph.add_directed_edge(&vertex_y, &vertex_v, 1.0);

        let sorted = graph.topological_sort().unwrap();
        println!("{:?}", sorted);
        assert!(is_valid_topological_order(&graph, &sorted));
    }
}
