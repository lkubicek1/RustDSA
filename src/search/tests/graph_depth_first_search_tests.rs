#[cfg(test)]
mod graph_depth_first_search_tests {
    use crate::search::graph_depth_first_search::Graph;

    fn contains_cycle(cycle: Vec<usize>, expected_cycles: Vec<Vec<usize>>) -> bool {
        for expected in expected_cycles {
            if cycle.len() != expected.len() {
                continue;
            }
            // Double the expected cycle to handle rotations
            let double_expected = [expected.clone(), expected.clone()].concat();
            if double_expected.windows(cycle.len()).any(|window| window == cycle) {
                return true;
            }
        }
        false
    }

    #[test]
    fn test_no_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        assert_eq!(graph.detect_cycle(), None);
    }

    #[test]
    fn test_simple_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(cycle, vec![vec![0, 1, 2, 0], vec![1, 2, 0, 1], vec![2, 0, 1, 2]]));
    }

    #[test]
    fn test_complex_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 2);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(
            cycle,
            vec![
                vec![2, 0, 1, 2],
                vec![0, 1, 2, 0],
                vec![1, 2, 0, 1],
                vec![4, 2, 3, 4],
                vec![3, 4, 2, 3],
                vec![2, 3, 4, 2]
            ]
        ));
    }

    #[test]
    fn test_disconnected_graph_with_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(
            cycle,
            vec![vec![0, 1, 2, 0], vec![1, 2, 0, 1], vec![2, 0, 1, 2]]
        ));
    }

    #[test]
    fn test_disconnected_graph_no_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        assert_eq!(graph.detect_cycle(), None);
    }

    #[test]
    fn test_self_loop() {
        let mut graph = Graph::new();
        graph.add_edge(0, 0);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(cycle, vec![vec![0, 0]]));
    }

    #[test]
    fn test_multiple_cycles() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 3);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(
            cycle,
            vec![
                vec![0, 1, 2, 0],
                vec![1, 2, 0, 1],
                vec![2, 0, 1, 2],
                vec![3, 4, 5, 3],
                vec![4, 5, 3, 4],
                vec![5, 3, 4, 5]
            ]
        ));
    }

    #[test]
    fn test_complex_disconnected_graph() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 2);
        graph.add_edge(5, 6);
        graph.add_edge(6, 7);
        graph.add_edge(7, 8);
        graph.add_edge(8, 5);
        graph.add_edge(9, 10);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(
            cycle,
            vec![
                vec![2, 3, 4, 2],
                vec![3, 4, 2, 3],
                vec![4, 2, 3, 4],
                vec![5, 6, 7, 8, 5],
                vec![6, 7, 8, 5, 6],
                vec![7, 8, 5, 6, 7],
                vec![8, 5, 6, 7, 8]
            ]
        ));
    }

    #[test]
    fn test_no_edges() {
        let graph = Graph::new();
        assert_eq!(graph.detect_cycle(), None);
    }

    #[test]
    fn test_single_vertex_no_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        assert_eq!(graph.detect_cycle(), None);
    }

    #[test]
    fn test_single_vertex_with_cycle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let cycle = graph.detect_cycle().unwrap();
        assert!(contains_cycle(cycle, vec![vec![0, 1, 2, 0], vec![1, 2, 0, 1], vec![2, 0, 1, 2]]));
    }

    #[test]
    fn test_large_acyclic_graph() {
        let mut graph = Graph::new();
        for i in 0..1000 {
            graph.add_edge(i, i + 1);
        }

        assert_eq!(graph.detect_cycle(), None);
    }

    #[test]
    fn test_large_cyclic_graph() {
        let mut graph = Graph::new();
        for i in 0..1000 {
            graph.add_edge(i, i + 1);
        }
        graph.add_edge(1000, 0);

        let cycle = graph.detect_cycle();

        assert!(cycle.is_some())

    }
}
