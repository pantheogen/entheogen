use crate::primitives::point::{Coordinate, Point};

/// The `Samplable` trait represents the property that a given form is
/// closed. This is required so we can sample a point within the area
/// it encloses.
pub trait Samplable {
    type T: Coordinate;
    /// sample_path samples a point on the path described by the form.
    fn sample_path(&self, step: Self::T) -> Point<Self::T>;
    /// sample_area samples a point within the area covered by the form.
    fn sample_area(&self, step: Self::T) -> Point<Self::T>;
}
