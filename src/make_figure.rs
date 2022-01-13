use crate::build_graph::Network;

pub fn make_figure(net: &Network) -> gnuplot::Figure {
    let mut fig = gnuplot::Figure::new();
    let ax = fig.axes2d();
    for line in &net.lines {
        plot_line(ax, line, &net.points);
    }
    fig
}

fn plot_line(ax: &mut gnuplot::Axes2D, line: &[usize], pts: &[(f64, f64)]) {
    let mut prev: Option<usize> = None;
    for id in line {
        let (x2, y2) = pts[*id];
        ax.points(
            [x2],
            [y2],
            &[gnuplot::PointSymbol('+'), gnuplot::Color("red")],
        );
        if let Some(prev) = prev {
            let (x1, y1) = pts[prev];
            ax.lines([x1, x2], [y1, y2], &[gnuplot::Color("black")]);
        }

        prev = Some(*id);
    }
}
