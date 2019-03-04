use alga::general::Ring;
use nalgebra::{Point2, Scalar};
use num::cast::AsPrimitive;

pub trait Coordinate: Ring + Scalar + AsPrimitive<i32> { }

impl Coordinate for i32 { }
impl Coordinate for f64 { }

pub type Point<T> = Point2<T>;
