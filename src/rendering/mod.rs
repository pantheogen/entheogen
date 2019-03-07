//! Rendering in Entheogen is currently performed using Cairo as a backend. We
//! leverage the "png" feature to provide output in the form of "pngs".
use cairo::{Context, ImageSurface, Format, LineCap};
use num::cast::AsPrimitive;

use std::fs::File;
use std::path::Path;

use crate::colour::RGBAColour;
use crate::primitives::point::Coordinate;
use crate::properties::{AsPoint, Plottable};
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

    pub fn fill(&self, colour: RGBAColour) -> Result<()> {
        let colour = colour.rgba();
        self.context.rectangle(0.0, 0.0, self.width.as_(), self.height.as_());
        self.context.set_source_rgba(colour.red, colour.green, colour.blue, colour.alpha);
        self.context.fill();
        Ok(())
    }

    pub fn plot<P: Plottable>(&self, p: P) -> Result<()> {
        for p in p.to_points() {
            let colour = p.colour().rgba();
            self.context.set_source_rgba(colour.red, colour.green, colour.blue, colour.alpha);
            self.context.set_line_width(p.weight());
            self.context.set_line_cap(LineCap::Round);
            self.context.move_to(p.x.as_(), p.y.as_());
            self.context.line_to(p.x.as_(), p.y.as_());
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
