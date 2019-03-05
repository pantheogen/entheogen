//! Rendering in Entheogen is currently performed using Cairo as a backend. We
//! leverage the "png" feature to provide output in the form of "pngs".
use cairo::{Context, ImageSurface, Format};

use std::fs::File;
use std::path::Path;

use crate::primitives::point::Coordinate;
use crate::error::*;

#[derive(Debug)]
pub struct Renderer<T> {
    surface: ImageSurface,
    context: Context,
    width: T,
    height: T,
}

impl<T: Coordinate> Renderer<T> {
    pub fn new(width: T, height: T) -> Result<Self> {
        let w = width.as_();
        let h = height.as_();

        let surface = ImageSurface::create(Format::ARgb32, w, h)
            .map_err(Error::from)
            .context("creating image surface")?;

        let context = Context::new(&surface);

        Ok(Renderer {
            surface, context, width, height
        })
    }

    pub fn to_png<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let mut file = File::create(path)
            .context("creating output png")?;
        self.surface.write_to_png(&mut file)
            .context("writing to png")?;
        Ok(())
    }
}
