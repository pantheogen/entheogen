use crate::primitives::point::{Coordinate, Point};

pub trait Plottable {
    // TODO: Make this into an iterator
    fn points(&self) -> Vec<Point<f64>>;
}

impl<T: Coordinate> Plottable for Point<T> {
    fn points(&self) -> Vec<Point<f64>> {
        vec![Point::new(self.x.as_(), self.y.as_())]
    }
}

impl<'a, T: Coordinate> Plottable for &'a [Point<T>] {
    fn points(&self) -> Vec<Point<f64>> {
        self.iter().map(|p| Point::new(p.x.as_(), p.y.as_())).collect()
    }
}

impl<'a, T: Coordinate> Plottable for Vec<Point<T>> {
    fn points(&self) -> Vec<Point<f64>> {
        self.as_slice().points()
    }
}
