use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 248.15926)
    }

    pub(crate) fn salmon(&self) -> Oklch {
        oklch(0.74244213, 0.03172374, 17.786991)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6307279, 0.1475799, 53.699818)
    }

    pub(crate) fn light_yellow(&self) -> Oklch {
        oklch(0.9256173, 0.13718618, 105.65656)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.9303855, 0.20273693, 109.78146)
    }

    pub(crate) fn light_lime_green(&self) -> Oklch {
        oklch(0.8648584, 0.21983564, 128.32455)
    }

    pub(crate) fn lime_green(&self) -> Oklch {
        oklch(0.856865, 0.21560241, 132.73979)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.6757668, 0.19794942, 145.87831)
    }

    pub(crate) fn neon_green(&self) -> Oklch {
        oklch(0.88403714, 0.15421008, 152.81999)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(0.7386806, 0.13793829, 173.58904)
    }

    pub(crate) fn light_cyan(&self) -> Oklch {
        oklch(0.8956409, 0.10436068, 198.51047)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.8484357, 0.113502264, 211.77254)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(0.8488954, 0.0866772, 224.72339)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.61555946, 0.12401431, 233.3464)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.65661603, 0.074155696, 242.23978)
    }

    pub(crate) fn bright_blue(&self) -> Oklch {
        oklch(0.7411311, 0.1336519, 260.34583)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.6994459, 0.11785177, 336.03506)
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
