use crate::palette::Palette;
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.background);
    builder.add_workspace_rules(&["foreground", "editor.foreground"], palette.foreground);
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[Semantic("keyword"), Semantic("builtinType")],
        palette.keywords,
    );

    builder.add_rule(Semantic("variable"), palette.variables);
    builder.add_rule(
        Semantic("variable.declaration"),
        palette.variable_declarations,
    );
    builder.add_rule(Semantic("parameter"), palette.parameters);

    builder.add_rules(
        &[Semantic("string"), Semantic("characterLiteral")],
        palette.strings,
    );
    builder.add_rule(Semantic("number"), palette.numbers);

    builder.add_rule(Semantic("method"), palette.methods);
    builder.add_rules(
        &[Semantic("function"), Semantic("method.static")],
        (palette.static_methods, FontStyle::Italic),
    );
    builder.add_rules(
        &[Semantic("function.trait"), Semantic("method.trait")],
        palette.trait_methods,
    );
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.method_declarations,
    );

    builder.add_rule(Semantic("macro"), palette.macros);

    builder.add_rules(
        &[Semantic("type"), Semantic("class"), Semantic("struct")],
        palette.types,
    );

    builder.add_rule(Semantic("typeAlias"), palette.type_aliases);

    builder.add_rule(Semantic("interface"), palette.interfaces);

    builder.add_rule(Semantic("enum"), (palette.enums, FontStyle::Italic));
    builder.add_rule(
        Semantic("enumMember"),
        (palette.enum_members, FontStyle::Italic),
    );

    builder.add_rules(
        &[Semantic("*.constant"), Semantic("variable.static")],
        (palette.constants, FontStyle::BoldItalic),
    );

    builder.add_rule(Semantic("typeParameter"), palette.type_parameters);

    builder.add_rule(Semantic("property"), palette.properties);

    builder.add_rule(Semantic("lifetime"), (palette.lifetime, FontStyle::Italic));

    builder.add_rules(
        &[Semantic("attribute"), Semantic("*.attribute")],
        (palette.attributes, FontStyle::Italic),
    );
}
