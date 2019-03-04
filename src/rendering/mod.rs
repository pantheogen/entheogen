//! Rendering in Entheogen is currently performed using Cairo as a backend. We
//! leverage the "png" feature to provide output in the form of "pngs".
use cairo::{Context, ImageSurface, Format};

use std::fs::File;
use std::io;
use std::path::Path;

use crate::primitives::point::Coordinate;

#[derive(Debug)]
pub struct Renderer<T> {
    surface: ImageSurface,
    context: Context,
    width: T,
    height: T,
}

impl<T: Coordinate> Renderer<T> {
    pub fn new(width: T, height: T) -> Self {
        let w = width.as_();
        let h = height.as_();

        // TODO: have this handled in a custom error type
        let surface = ImageSurface::create(Format::ARgb32, w, h).unwrap();
        let context = Context::new(&surface);

        Renderer {
            surface, context, width, height
        }
    }

    pub fn to_png<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let path = path.as_ref();
        let mut file = File::create(path)?;
        self.surface.write_to_png(&mut file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
