use std::vec::IntoIter;
use crate::primitives::point::{Coordinate, Point};

use super::AsPoint;

pub trait Plottable {
    type Item: Coordinate;
    type Iter: Iterator<Item=Point<Self::Item>>;

    fn to_points(self) -> Self::Iter;
}

impl<T: Coordinate> Plottable for Vec<Point<T>> {
    type Item = T;
    type Iter = IntoIter<Point<T>>;

    fn to_points(self) -> Self::Iter {
        self.into_iter()
    }
}

impl<P: AsPoint> Plottable for P {
    type Item = <P as AsPoint>::Coordinate;
    type Iter = IntoIter<Point<Self::Item>>;

    fn to_points(self) -> Self::Iter {
        vec![self.to_point()].into_iter()
    }
}
