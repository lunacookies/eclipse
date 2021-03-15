use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 248.15926)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6307279, 0.1475799, 53.699818)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.61555946, 0.12401431, 233.3464)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.8956409, 0.10436068, 198.51047)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    LightBg,
    LighterBg,
    DarkFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.25..0.99)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.0..0.03)
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::LightBg => 0.03,
            Self::LighterBg => 0.15,
            Self::DarkFg => 0.5,
            Self::Fg => 0.72,
            Self::BrightFg => 1.0,
        }
    }
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
