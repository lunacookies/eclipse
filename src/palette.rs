use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 248.15926)
    }

    const LOW_LIGHTNESS: f32 = 0.65;
    const MEDIUM_LIGHTNESS: f32 = 0.75;
    const HIGH_LIGHTNESS: f32 = 0.85;

    const LOWER_CHROMA: f32 = 0.03;
    const LOW_CHROMA: f32 = 0.1;
    const MEDIUM_CHROMA: f32 = 0.15;
    const HIGH_CHROMA: f32 = 0.2;

    pub(crate) fn salmon(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOWER_CHROMA, 20.0)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::MEDIUM_CHROMA, 55.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::MEDIUM_CHROMA, 110.0)
    }

    pub(crate) fn lime_green(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::HIGH_CHROMA, 130.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::HIGH_CHROMA, 145.0)
    }

    pub(crate) fn neon_green(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::MEDIUM_CHROMA, 155.0)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOW_CHROMA, 175.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 210.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 225.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::LOW_CHROMA, 235.0)
    }

    pub(crate) fn bright_blue(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOW_CHROMA, 260.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOW_CHROMA, 335.0)
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
