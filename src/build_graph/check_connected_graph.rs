use super::NetGraph;
use simplegraph::GraphVisitor;

pub fn is_connected(g: &NetGraph) -> bool {
    let nodes = g.node_count();
    let mut node_queue: Vec<usize> = Vec::with_capacity(nodes);
    let mut visited: Vec<bool> = vec![false; nodes];
    node_queue.push(0);

    while let Some(i) = node_queue.pop() {
        visited[i] = true;
        for j in successors(g, i) {
            if !visited[j] {
                node_queue.push(j);
            }
        }
    }

    visited.iter().all(|b| *b)
}

fn successors(g: &NetGraph, n: usize) -> impl Iterator<Item = usize> + '_ {
    g.successor_iterator(n).map(|(_, dst, _)| dst)
}

#[cfg(test)]
mod test {

    use super::*;
    use simplegraph::Graph;

    #[test]
    fn test_connected_graph() {
        let mut graph = make_base_graph();
        graph.add_new_default_arc(1, 3);
        graph.add_new_default_arc(2, 4);

        assert!(is_connected(&graph));
    }

    #[test]
    fn test_unconnected_graph() {
        let graph = make_base_graph();
        assert!(!is_connected(&graph));
    }

    fn make_base_graph() -> NetGraph {
        let mut graph = NetGraph::new_direct(5);

        graph.add_new_default_arc(0, 1);
        graph.add_new_default_arc(1, 2);
        graph.add_new_default_arc(3, 4);

        graph
    }
}
