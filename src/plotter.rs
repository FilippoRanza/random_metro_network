use gnuplot::{Axes2D, Color, Figure, PointSize, PointSymbol};
use ndarray::Array2;

pub struct PlotFigure {
    fig: Figure,
}
impl PlotFigure {
    pub fn new() -> Self {
        Self { fig: Figure::new() }
    }

    pub fn make_plotter(&mut self) -> Plotter<'_> {
        Plotter::new(&mut self.fig)
    }

    pub fn show(&mut self) {
        self.fig.show().unwrap();
    }
}

pub struct Plotter<'a> {
    ax: &'a mut Axes2D,
}

impl<'a> Plotter<'a> {
    pub fn new(fig: &'a mut Figure) -> Self {
        Self { ax: fig.axes2d() }
    }

    pub fn draw_points<T: super::Localizable>(&mut self, points: &[T], symbol: Option<char>) {
        let x = points.iter().map(|p| p.get_x());
        let y = points.iter().map(|p| p.get_y());

        self.ax
            .points(x, y, &[PointSize(2.), PointSymbol(symbol.unwrap_or('S'))]);
    }

    pub fn draw_line<T: super::Localizable>(&mut self, p1: &T, p2: &T) {
        let (x1, y1) = p1.coordinates();
        let (x2, y2) = p2.coordinates();
        self.ax.lines([x1, x2], [y1, y2], &[Color("black")]);
    }

    pub fn draw_lines<T: super::Localizable>(&mut self, points: &[T], adj_mat: &Array2<f64>) {
        for ((i, j), adj) in adj_mat.indexed_iter() {
            if *adj < f64::MAX {
                self.draw_line(&points[i], &points[j]);
            }
        }
    }
}
