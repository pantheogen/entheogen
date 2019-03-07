use std::f64::consts::PI;

use crate::primitives::point::{Coordinate, Point};
use crate::properties::Plottable;
use crate::error::*;

const ABS_DIFFERENCE: f64 = 1e-10;

#[derive(Debug, Clone)]
pub struct Circle<T: Coordinate> {
    centre: Point<T>,
    radius: T,
}

impl<T: Coordinate> Circle<T> {
    pub fn new(centre: Point<T>, radius: T) -> Result<Circle<T>> {
        if radius <= T::zero() {
            return Err(Error::InvalidParameter {
                reason: "a circle cannot have a negative radius",
            }.into())
        }

        Ok(Circle {
            centre, radius,
        })
    }

    pub fn centre(&self) -> Point<T> {
        self.centre
    }

    pub fn radius(&self) -> T {
        self.radius
    }
}

pub struct CirclePointIterator<'a, T: Coordinate> {
    circle: &'a Circle<T>,
    angle: f64,
}

impl<'a, T: Coordinate> Iterator for CirclePointIterator<'a, T> {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.angle - 2.0 * PI).abs() >= ABS_DIFFERENCE {
            let cx = self.circle.centre.x;
            let cy = self.circle.centre.y;
            let r = self.circle.radius;

            let x = cx + r * T::from_f64(self.angle.sin()).unwrap();
            let y = cy + r * T::from_f64(self.angle.cos()).unwrap();

            self.angle += PI / 180.0;

            Some(Point::new(x, y))
        } else {
            None
        }
    }
}

impl<'a, T: Coordinate> Plottable for &'a Circle<T> {
    type Item = T;
    type Iter = CirclePointIterator<'a, T>;

    fn to_points(self) -> Self::Iter {
        CirclePointIterator {
            circle: self,
            angle: 0.0,
        }
    }
}
