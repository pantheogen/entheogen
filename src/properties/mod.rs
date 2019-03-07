mod plottable;
mod samplable;

pub use self::samplable::Samplable;
pub use self::plottable::Plottable;

use crate::primitives::point::{Coordinate, Point};
use crate::colour::defaults::*;
use crate::colour::RGBAColour;

pub trait AsPoint {
    type Coordinate: Coordinate;

    fn x(&self) -> Self::Coordinate;
    fn y(&self) -> Self::Coordinate;

    fn weight(&self) -> f64 {
        1.0
    }

    fn colour(&self) -> RGBAColour {
        *BLACK
    }

    fn to_point(self) -> Point<Self::Coordinate>;
}

impl<T: Coordinate> AsPoint for Point<T> {
    type Coordinate = T;

    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn to_point(self) -> Self {
        self
    }
}
