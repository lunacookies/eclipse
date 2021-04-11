mod dark;
mod light;

pub(crate) use dark::Dark;
pub(crate) use light::Light;

use std::ops::Range;
use tincture::{Hue, Oklch};

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    DimFg,
    Fg,
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
