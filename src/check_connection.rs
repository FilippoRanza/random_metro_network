use ndarray::{Array2, ArrayView1};

pub fn is_connected(g: &Array2<f64>) -> bool {
    let visited = breadth_first_search(g);
    visited.iter().all(|v| *v)
}


fn breadth_first_search(g: &Array2<f64>) -> Vec<bool> {
    let mut visited = vec![false; g.nrows()];
    let mut stack = vec![0];
    while let Some(curr) = stack.pop() {
        visited[curr] = true;
        for next in adj_nodes(&g.row(curr)) {
            if !visited[next] {
                stack.push(next);
            } 
        }
    }
    visited
}

fn adj_nodes<'a>(row: &'a ArrayView1<f64>) -> impl Iterator<Item = usize> + 'a {
    row.iter().enumerate().filter_map(move |(i, v)| if *v < f64::MAX {Some(i)} else {None})
}