use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["foreground", "editor.foreground"],
        palette.base(BaseScale::Fg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[Semantic("keyword"), Semantic("builtinType")],
        palette.brown(),
    );

    builder.add_rule(Semantic("variable"), palette.light_yellow());
    builder.add_rule(Semantic("variable.declaration"), palette.yellow());
    builder.add_rule(Semantic("parameter"), palette.bright_blue());

    builder.add_rule(Semantic("string"), palette.turquoise());
    builder.add_rule(Semantic("number"), palette.blue());

    builder.add_rule(Semantic("method"), palette.lime_green());
    builder.add_rules(
        &[Semantic("function"), Semantic("method.static")],
        (palette.lime_green(), FontStyle::Italic),
    );
    builder.add_rules(
        &[Semantic("function.trait"), Semantic("method.trait")],
        palette.neon_green(),
    );
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.green(),
    );

    builder.add_rule(Semantic("macro"), palette.neon_green());

    builder.add_rules(
        &[Semantic("type"), Semantic("class"), Semantic("struct")],
        palette.blue(),
    );

    builder.add_rule(Semantic("typeAlias"), palette.light_blue());

    builder.add_rule(Semantic("interface"), palette.aqua());

    builder.add_rule(Semantic("enum"), (palette.pink(), FontStyle::Italic));
    builder.add_rule(Semantic("enumMember"), (palette.azure(), FontStyle::Italic));

    builder.add_rules(
        &[Semantic("*.constant"), Semantic("variable.static")],
        (palette.azure(), FontStyle::BoldItalic),
    );

    builder.add_rule(Semantic("typeParameter"), palette.neon_green());

    builder.add_rule(Semantic("property"), palette.cyan());

    builder.add_rule(Semantic("lifetime"), palette.green());

    builder.add_rules(
        &[Semantic("attribute"), Semantic("*.attribute")],
        (palette.base(BaseScale::DarkFg), FontStyle::Italic),
    );
}
