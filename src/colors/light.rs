use super::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Light;

impl Light {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(base_lightness(scale), base_chroma(scale), 0.0)
    }

    pub(crate) fn maroon(&self) -> Oklch {
        oklch(0.41668323, 0.061962005, 20.004015)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.46220446, 0.30628103, 268.8053)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.36510578, 0.25307775, 264.05856)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.39466092, 0.16567557, 348.28958)
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
