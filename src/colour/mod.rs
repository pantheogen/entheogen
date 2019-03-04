/// We use the palette library to provide colour conversions the default colour
/// that maps to screen colours that appear as "RGBA" elsewhere are represented
/// as linear SRGBA encoding.

use palette::encoding::Srgb;
use palette::LinSrgba;
use palette::Hsva;

pub type RGBA = LinSrgba<f64>;
pub type HSVA = Hsva<Srgb, f64>;

pub mod prelude {
    pub use super::{RGBA, HSVA, RGBAColour, HSVAColour};
    pub use palette::{Blend, Component, Mix, Shade};
}

pub struct RGBAColour {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl RGBAColour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGBAColour { r, g, b, a: 1.0 }
    }

    pub fn red(self, r: f64) -> Self {
        RGBAColour { r, ..self }
    }

    pub fn green(self, g: f64) -> Self {
        RGBAColour { g, ..self }
    }

    pub fn blue(self, b: f64) -> Self {
        RGBAColour { b, ..self }
    }

    pub fn alpha(self, alpha: f64) -> Self {
        RGBAColour { a: alpha, ..self }
    }

    pub fn rgba(self) -> RGBA {
        LinSrgba::new(self.r, self.g, self.b, self.a)
    }

    pub fn hsva(self) -> HSVA {
        LinSrgba::new(self.r, self.g, self.b, self.a).into()
    }
}

pub struct HSVAColour {
    h: f64,
    s: f64,
    v: f64,
    a: f64,
}

impl HSVAColour {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        HSVAColour { h, s, v, a: 1.0 }
    }

    pub fn hue(self, h: f64) -> Self {
        HSVAColour { h, ..self }
    }

    pub fn saturation(self, s: f64) -> Self {
        HSVAColour { s, ..self }
    }

    pub fn value(self, v: f64) -> Self {
        HSVAColour { v, ..self }
    }

    pub fn alpha(self, alpha: f64) -> Self {
        HSVAColour { a: alpha, ..self }
    }

    pub fn rgba(self) -> RGBA {
        Hsva::new(self.h, self.s, self.v, self.a).into()
    }

    pub fn hsva(self) -> HSVA {
        Hsva::new(self.h, self.s, self.v, self.a)
    }
}
