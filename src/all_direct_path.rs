use super::build_graph;
use simplegraph::{path_cost, Graph, GetGraphType, GraphVisitor};

pub fn all_direct_path(mut net: build_graph::Network) -> build_graph::Network {
    net.graph = build_all_direct_path_graph(net.graph, &net.lines);
    net
}

fn build_all_direct_path_graph(
    input: build_graph::NetGraph,
    lines: &build_graph::Lines,
) -> build_graph::NetGraph {
    let mut output = new_graph(&input);
    for line in lines {
        add_line_arcs(&input, &mut output, line);
    }
    output
}

fn add_line_arcs(
    input: &build_graph::NetGraph,
    output: &mut build_graph::NetGraph,
    line: &[usize],
) {
    for (i, j, w) in path_cost::AllSubPathCost::new(input, line) {
        output.add_new_arc(i, j, w)
    }
}

fn new_graph(g: &build_graph::NetGraph) -> build_graph::NetGraph {
    let nodes = g.node_count();
    build_graph::NetGraph::new(nodes, g.graph_type())
}

#[cfg(test)]
mod test {

    use super::*;
    use simplegraph::GraphVisitor;
    use std::collections::HashMap;

    #[test]
    fn test_all_direct_build_direct() {
        run_test(false);
    }

    #[test]
    fn test_all_direct_build_undirect() {
        run_test(true);
    }

    fn run_test(undirect: bool) {
        let network = init_network(undirect);
        let network = all_direct_path(network);

        let mut expected = get_all_arcs(undirect);
        let graph = &network.graph;
        graph.arc_visitor(|i, j, w| {
            let value = expected.get_mut(&(i, j));
            assert!(value.is_some(), "{:?}", (i, j));
            let (aw, v) = value.unwrap();
            assert_eq!(w, *aw);
            *v = true;
        });

        for (k, (_, v)) in expected.iter() {
            assert!(v, "{:?}", k);
        }

    }


    fn init_network(undirect: bool) -> build_graph::Network {
        let mut graph = if undirect {
            build_graph::NetGraph::new_undirect(9)
        } else {
            build_graph::NetGraph::new_direct(9)
        };
        for (i, j, w) in get_base_case_arcs() {
            graph.add_new_arc(i, j, w);
        }
        let lines = vec![vec![0, 1, 2, 3, 4], vec![6, 5, 2, 7, 8]];

        build_graph::Network {
            lines,
            graph,
            points: vec![],
        }
    }

    fn get_base_case_arcs() -> Vec<(usize, usize, f64)> {
        vec![
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
            (3, 4, 1.0),
            (6, 5, 1.0),
            (5, 2, 1.0),
            (2, 7, 1.0),
            (7, 8, 1.0),
        ]
    }

    fn get_all_arcs(undirect: bool) -> HashMap<(usize, usize), (f64, bool)> {
        let all_arcs = vec![
            (0, 1, 1.0),
            (0, 2, 2.0),
            (0, 3, 3.0),
            (0, 4, 4.0),
            (1, 2, 1.0),
            (1, 3, 2.0),
            (1, 4, 3.0),
            (2, 3, 1.0),
            (2, 4, 2.0),
            (3, 4, 1.0),
            (6, 5, 1.0),
            (6, 2, 2.0),
            (6, 7, 3.0),
            (6, 8, 4.0),
            (5, 2, 1.0),
            (5, 7, 2.0),
            (5, 8, 3.0),
            (2, 7, 1.0),
            (2, 8, 2.0),
            (7, 8, 1.0),
        ];

        let mut hm = HashMap::with_capacity(all_arcs.len());
        for (i, j, w) in all_arcs.into_iter() {
            hm.insert((i, j), (w, false));
            if undirect {
                hm.insert((j, i), (w, false));
            }
        }

        hm
    }
}
