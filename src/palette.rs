use crate::colors::{BaseScale, Dark, Light};
use tincture::Oklch;

pub(crate) trait Palette {
    fn base(&self, scale: BaseScale) -> Oklch;
    fn keywords(&self) -> Oklch;
    fn are_keywords_bold(&self) -> bool;
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
    fn are_enums_italic(&self) -> bool;
    fn enum_members(&self) -> Oklch;
    fn are_enum_members_bold(&self) -> bool;
    fn constants(&self) -> Oklch;
    fn type_parameters(&self) -> Oklch;
    fn properties(&self) -> Oklch;
    fn lifetime(&self) -> Oklch;
}

impl Palette for Dark {
    fn base(&self, scale: BaseScale) -> Oklch {
        self.base(scale)
    }
    fn keywords(&self) -> Oklch {
        self.brown()
    }
    fn are_keywords_bold(&self) -> bool {
        false
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
    fn are_enums_italic(&self) -> bool {
        true
    }
    fn enum_members(&self) -> Oklch {
        self.azure()
    }
    fn are_enum_members_bold(&self) -> bool {
        false
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
}

impl Palette for Light {
    fn base(&self, scale: BaseScale) -> Oklch {
        self.base(scale)
    }
    fn keywords(&self) -> Oklch {
        self.purple()
    }
    fn are_keywords_bold(&self) -> bool {
        true
    }
    fn variables(&self) -> Oklch {
        self.maroon()
    }
    fn variable_declarations(&self) -> Oklch {
        self.maroon()
    }
    fn parameters(&self) -> Oklch {
        self.maroon()
    }
    fn strings(&self) -> Oklch {
        self.blue()
    }
    fn numbers(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn methods(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn static_methods(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn trait_methods(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn method_declarations(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn macros(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn types(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn type_aliases(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn interfaces(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn enums(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn are_enums_italic(&self) -> bool {
        false
    }
    fn enum_members(&self) -> Oklch {
        self.dark_blue()
    }
    fn are_enum_members_bold(&self) -> bool {
        true
    }
    fn constants(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn type_parameters(&self) -> Oklch {
        self.base(BaseScale::Fg)
    }
    fn properties(&self) -> Oklch {
        self.dark_blue()
    }
    fn lifetime(&self) -> Oklch {
        self.blue()
    }
}
