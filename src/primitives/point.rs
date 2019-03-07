use alga::general::Ring;
use nalgebra::{Point2, Scalar};
use num::cast::{AsPrimitive, FromPrimitive};

pub trait Coordinate: Ring + Scalar + AsPrimitive<i32> + AsPrimitive<f64> + FromPrimitive + PartialEq + PartialOrd { }

impl Coordinate for i32 { }
impl Coordinate for i64 { }
impl Coordinate for f64 { }

pub type Point<T> = Point2<T>;
