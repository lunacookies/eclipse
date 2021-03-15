use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.brown());

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("builtinType"),
            Semantic("typeAlias"),
            Semantic("typeParameter"),
        ],
        palette.blue(),
    );

    builder.add_rule(Semantic("interface"), palette.light_blue());

    builder.add_rule(Semantic("enum"), (palette.pink(), FontStyle::Italic));
    builder.add_rule(Semantic("enumMember"), (palette.cyan(), FontStyle::Italic));
}
