//! Graph algorithms and data structures

pub use petgraph::prelude::*;

/// Re-export commonly used graph types
pub type DirectedGraph<N, E> = petgraph::Graph<N, E, petgraph::Directed>;
pub type UnDirectedGraph<N, E> = petgraph::Graph<N, E, petgraph::Undirected>;

/// Algorithms module
pub mod algorithms {
    use super::*;
    use petgraph::algo;

    /// Find strongly connected components
    pub fn strongly_connected_components<N, E>(
        graph: &DirectedGraph<N, E>,
    ) -> Vec<Vec<NodeIndex>> {
        algo::kosaraju_scc(graph)
    }

    /// Find the shortest path between two nodes
    pub fn shortest_path<N, E>(
        graph: &DirectedGraph<N, E>,
        start: NodeIndex,
        goal: NodeIndex,
    ) -> Option<Vec<NodeIndex>>
    where
        E: Clone,
    {
        algo::astar(
            graph,
            start,
            |finish| finish == goal,
            |_| 1,
            |_| 0,
        )
        .map(|(_, path)| path)
    }

    /// Check if the graph has a cycle
    pub fn has_cycle<N, E>(graph: &DirectedGraph<N, E>) -> bool {
        algo::is_cyclic_directed(graph)
    }

    /// Topological sort (returns None if graph has cycles)
    pub fn topological_sort<N, E>(
        graph: &DirectedGraph<N, E>,
    ) -> Result<Vec<NodeIndex>, ()> {
        algo::toposort(graph, None).map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algorithms::*;

    #[test]
    fn test_graph_creation() {
        let mut graph: DirectedGraph<i32, ()> = DirectedGraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, ());

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph: DirectedGraph<i32, ()> = DirectedGraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, ());

        assert!(!has_cycle(&graph));

        graph.add_edge(n2, n1, ());
        assert!(has_cycle(&graph));
    }

    #[test]
    fn test_topological_sort() {
        let mut graph: DirectedGraph<&str, ()> = DirectedGraph::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());

        let sorted = topological_sort(&graph).unwrap();
        assert_eq!(sorted.len(), 3);
    }
}
