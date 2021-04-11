use super::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Dark;

impl Dark {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(base_lightness(scale), base_chroma(scale), 250.0)
    }

    const LOW_LIGHTNESS: f32 = 0.65;
    const MEDIUM_LIGHTNESS: f32 = 0.75;
    const HIGH_LIGHTNESS: f32 = 0.85;
    const HIGHER_LIGHTNESS: f32 = 0.92;

    const LOW_CHROMA: f32 = 0.1;
    const MEDIUM_CHROMA: f32 = 0.15;
    const HIGH_CHROMA: f32 = 0.2;

    pub(crate) fn brown(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::MEDIUM_CHROMA, 55.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(Self::HIGHER_LIGHTNESS, Self::HIGH_CHROMA, 110.0)
    }

    pub(crate) fn light_yellow(&self) -> Oklch {
        oklch(Self::HIGHER_LIGHTNESS, Self::LOW_CHROMA, 110.0)
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

    pub(crate) fn aqua(&self) -> Oklch {
        oklch(Self::HIGHER_LIGHTNESS, Self::LOW_CHROMA, 200.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 210.0)
    }

    pub(crate) fn azure(&self) -> Oklch {
        oklch(Self::HIGH_LIGHTNESS, Self::LOW_CHROMA, 225.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(Self::LOW_LIGHTNESS, Self::LOW_CHROMA, 235.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOW_CHROMA, 235.0)
    }

    pub(crate) fn bright_blue(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOW_CHROMA, 260.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::LOW_CHROMA, 335.0)
    }
}

fn base_lightness(scale: BaseScale) -> f32 {
    lerp(base_value(scale), 0.3..0.925)
}

fn base_chroma(scale: BaseScale) -> f32 {
    lerp(base_value(scale), 0.0..0.026)
}

fn base_value(scale: BaseScale) -> f32 {
    match scale {
        BaseScale::Bg => 0.0,
        BaseScale::DimFg => 0.7,
        BaseScale::Fg => 1.0,
    }
}
