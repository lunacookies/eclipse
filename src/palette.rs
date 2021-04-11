use crate::colors::{BaseScale, Dark};
use tincture::Oklch;

pub(crate) trait Palette {
    fn background(&self) -> Oklch;
    fn foreground(&self) -> Oklch;
    fn keywords(&self) -> Oklch;
    fn variables(&self) -> Oklch;
    fn variable_declarations(&self) -> Oklch;
    fn parameters(&self) -> Oklch;
    fn strings(&self) -> Oklch;
    fn numbers(&self) -> Oklch;
    fn methods(&self) -> Oklch;
    fn static_methods(&self) -> Oklch;
    fn trait_methods(&self) -> Oklch;
    fn method_declarations(&self) -> Oklch;
    fn macros(&self) -> Oklch;
    fn types(&self) -> Oklch;
    fn type_aliases(&self) -> Oklch;
    fn interfaces(&self) -> Oklch;
    fn enums(&self) -> Oklch;
    fn enum_members(&self) -> Oklch;
    fn constants(&self) -> Oklch;
    fn type_parameters(&self) -> Oklch;
    fn properties(&self) -> Oklch;
    fn lifetime(&self) -> Oklch;
    fn attributes(&self) -> Oklch;
}

impl Palette for Dark {
    fn background(&self) -> Oklch {
        self.base(BaseScale::Bg)
    }
    fn foreground(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn keywords(&self) -> Oklch {
        self.brown()
    }
    fn variables(&self) -> Oklch {
        self.light_yellow()
    }
    fn variable_declarations(&self) -> Oklch {
        self.yellow()
    }
    fn parameters(&self) -> Oklch {
        self.bright_blue()
    }
    fn strings(&self) -> Oklch {
        self.turquoise()
    }
    fn numbers(&self) -> Oklch {
        self.blue()
    }
    fn methods(&self) -> Oklch {
        self.lime_green()
    }
    fn static_methods(&self) -> Oklch {
        self.lime_green()
    }
    fn trait_methods(&self) -> Oklch {
        self.neon_green()
    }
    fn method_declarations(&self) -> Oklch {
        self.green()
    }
    fn macros(&self) -> Oklch {
        self.neon_green()
    }
    fn types(&self) -> Oklch {
        self.blue()
    }
    fn type_aliases(&self) -> Oklch {
        self.light_blue()
    }
    fn interfaces(&self) -> Oklch {
        self.aqua()
    }
    fn enums(&self) -> Oklch {
        self.pink()
    }
    fn enum_members(&self) -> Oklch {
        self.azure()
    }
    fn constants(&self) -> Oklch {
        self.azure()
    }
    fn type_parameters(&self) -> Oklch {
        self.neon_green()
    }
    fn properties(&self) -> Oklch {
        self.cyan()
    }
    fn lifetime(&self) -> Oklch {
        self.green()
    }
    fn attributes(&self) -> Oklch {
        self.base(BaseScale::DarkFg)
    }
}
