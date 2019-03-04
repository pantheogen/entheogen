//! Entheogen: a rust library for generative art
//!
//! Entheogen provides an array of opinionated primitives for composing
//! generative artwork. It acts as a basis for:
//!
//!  - Constructing high-level abstractions over geometric primitives.
//!  - Converting primitives to points within a 2-D space, namely real numbers
//!    ($$\mathbb{R}^2$$) natural numbers $$\mathbb{N}^2$$.
//!  - Sampling points within boundaries defined by geometric forms using
//!    user-specified sampling methods.
//!  - Rendering a points represention of artwork to common image formats.

extern crate alga;
extern crate cairo;
extern crate nalgebra;
extern crate num;
extern crate palette;

pub mod colour;
pub mod primitives;
pub mod properties;
pub mod rendering;

pub use primitives::point::Point;
