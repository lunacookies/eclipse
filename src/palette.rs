use crate::colors::{BaseScale, Dark};
use tincture::Oklch;

pub(crate) struct Palette {
    pub(crate) background: Oklch,
    pub(crate) foreground: Oklch,
    pub(crate) keywords: Oklch,
    pub(crate) variables: Oklch,
    pub(crate) variable_declarations: Oklch,
    pub(crate) parameters: Oklch,
    pub(crate) strings: Oklch,
    pub(crate) numbers: Oklch,
    pub(crate) methods: Oklch,
    pub(crate) static_methods: Oklch,
    pub(crate) trait_methods: Oklch,
    pub(crate) method_declarations: Oklch,
    pub(crate) macros: Oklch,
    pub(crate) types: Oklch,
    pub(crate) type_aliases: Oklch,
    pub(crate) interfaces: Oklch,
    pub(crate) enums: Oklch,
    pub(crate) enum_members: Oklch,
    pub(crate) constants: Oklch,
    pub(crate) type_parameters: Oklch,
    pub(crate) properties: Oklch,
    pub(crate) lifetime: Oklch,
    pub(crate) attributes: Oklch,
}

impl Palette {
    pub(crate) fn dark() -> Self {
        let colors = Dark;

        Self {
            background: colors.base(BaseScale::Bg),
            foreground: colors.base(BaseScale::Fg),
            keywords: colors.brown(),
            variables: colors.light_yellow(),
            variable_declarations: colors.yellow(),
            parameters: colors.bright_blue(),
            strings: colors.turquoise(),
            numbers: colors.blue(),
            methods: colors.lime_green(),
            static_methods: colors.lime_green(),
            trait_methods: colors.neon_green(),
            method_declarations: colors.green(),
            macros: colors.neon_green(),
            types: colors.blue(),
            type_aliases: colors.light_blue(),
            interfaces: colors.aqua(),
            enums: colors.pink(),
            enum_members: colors.azure(),
            constants: colors.azure(),
            type_parameters: colors.neon_green(),
            properties: colors.cyan(),
            lifetime: colors.green(),
            attributes: colors.base(BaseScale::DarkFg),
        }
    }
}
