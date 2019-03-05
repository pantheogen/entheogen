//! Rendering in Entheogen is currently performed using Cairo as a backend. We
//! leverage the "png" feature to provide output in the form of "pngs".
use cairo::{Context, ImageSurface, Format};

use std::fs::File;
use std::path::Path;

use crate::primitives::point::Coordinate;
use crate::properties::Plottable;
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

    pub fn plot<P: Plottable>(&self, p: P) -> Result<()> {
        for p in p.points().into_iter() {
            self.context.set_line_width(1.0);
            self.context.move_to(p.x, p.y);
            self.context.stroke();
        }
        Ok(())
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
