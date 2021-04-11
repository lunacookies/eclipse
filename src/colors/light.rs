use super::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Light;

impl Light {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(base_lightness(scale), base_chroma(scale), 0.0)
    }

    pub(crate) fn maroon(&self) -> Oklch {
        oklch(0.4, 0.06, 20.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.45, 0.29, 270.0)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.35, 0.24, 265.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.4, 0.16, 350.0)
    }
}

fn base_lightness(scale: BaseScale) -> f32 {
    lerp(base_value(scale), 1.0..0.0)
}

fn base_chroma(scale: BaseScale) -> f32 {
    lerp(base_value(scale), 0.0..0.0)
}

fn base_value(scale: BaseScale) -> f32 {
    match scale {
        BaseScale::Bg => 0.0,
        BaseScale::DimFg => 0.5,
        BaseScale::Fg => 1.0,
    }
}
