use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 248.15926)
    }

    pub(crate) fn salmon(&self) -> Oklch {
        oklch(0.75, 0.03, 20.0)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.65, 0.15, 55.0)
    }

    pub(crate) fn light_yellow(&self) -> Oklch {
        oklch(0.9, 0.15, 105.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.95, 0.2, 110.0)
    }

    pub(crate) fn light_lime_green(&self) -> Oklch {
        oklch(0.85, 0.22, 130.0)
    }

    pub(crate) fn lime_green(&self) -> Oklch {
        oklch(0.85, 0.22, 135.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.65, 0.2, 145.0)
    }

    pub(crate) fn neon_green(&self) -> Oklch {
        oklch(0.9, 0.15, 155.0)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(0.75, 0.13, 175.0)
    }

    pub(crate) fn light_cyan(&self) -> Oklch {
        oklch(0.9, 0.1, 200.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.85, 0.11, 210.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(0.85, 0.085, 225.0)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.6, 0.12, 235.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.65, 0.07, 240.0)
    }

    pub(crate) fn bright_blue(&self) -> Oklch {
        oklch(0.75, 0.13, 260.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.7, 0.12, 335.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    DarkFg,
    Fg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.3..0.925)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.0..0.026)
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::DarkFg => 0.7,
            Self::Fg => 1.0,
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
