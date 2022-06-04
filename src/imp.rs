use crate::colors::BaseScale;
use crate::palette::Palette;
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &impl Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &impl Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["foreground", "editor.foreground"],
        palette.base(BaseScale::Fg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &impl Palette) {
    if palette.are_keywords_bold() {
        builder.add_rules(
            &[
                Semantic("keyword"),
                Semantic("builtinType"),
                Textmate("keyword"),
                Textmate("storage"),
                Textmate("variable.language"),
                Textmate("constant.language"),
                Textmate("entity.name.tag"),
            ],
            (palette.keywords(), FontStyle::Bold),
        );
    } else {
        builder.add_rules(
            &[
                Semantic("keyword"),
                Semantic("builtinType"),
                Textmate("keyword"),
                Textmate("storage"),
                Textmate("variable.language"),
                Textmate("constant.language"),
                Textmate("entity.name.tag"),
            ],
            palette.keywords(),
        );
    }

    builder.add_rules(
        &[Semantic("variable"), Textmate("variable")],
        palette.variables(),
    );
    builder.add_rule(
        Semantic("variable.declaration"),
        palette.variable_declarations(),
    );
    builder.add_rule(Semantic("parameter"), palette.parameters());

    builder.add_rules(
        &[
            Semantic("string"),
            Semantic("characterLiteral"),
            Textmate("string"),
        ],
        palette.strings(),
    );
    builder.add_rules(
        &[Semantic("number"), Textmate("constant.numeric")],
        palette.numbers(),
    );

    builder.add_rules(
        &[Semantic("method"), Textmate("entity.name.function")],
        palette.methods(),
    );
    builder.add_rules(
        &[Semantic("function"), Semantic("method.static")],
        (palette.static_methods(), FontStyle::Italic),
    );
    builder.add_rules(
        &[Semantic("function.trait"), Semantic("method.trait")],
        palette.trait_methods(),
    );
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.method_declarations(),
    );

    builder.add_rules(
        &[Semantic("macro"), Textmate("entity.name.function.macro")],
        palette.macros(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Textmate("entity.name.type"),
        ],
        palette.types(),
    );

    builder.add_rule(Semantic("typeAlias"), palette.type_aliases());

    builder.add_rule(Semantic("interface"), palette.interfaces());

    if palette.are_enums_italic() {
        builder.add_rule(Semantic("enum"), (palette.enums(), FontStyle::Italic));
    } else {
        builder.add_rule(Semantic("enum"), palette.enums());
    }

    if palette.are_enum_members_bold() {
        builder.add_rule(
            Semantic("enumMember"),
            (palette.enum_members(), FontStyle::BoldItalic),
        );
    } else {
        builder.add_rule(
            Semantic("enumMember"),
            (palette.enum_members(), FontStyle::Italic),
        );
    }

    builder.add_rules(
        &[
            Semantic("*.constant"),
            Semantic("variable.static"),
            Textmate("constant"),
        ],
        (palette.constants(), FontStyle::BoldItalic),
    );

    builder.add_rule(Semantic("typeParameter"), palette.type_parameters());

    builder.add_rules(
        &[Semantic("property"), Textmate("support.type.property-name")],
        palette.properties(),
    );

    builder.add_rules(
        &[
            Semantic("lifetime"),
            Textmate("entity.name.type.lifetime"),
            Textmate("punctuation.definition.lifetime"),
        ],
        (palette.lifetime(), FontStyle::Italic),
    );

    builder.add_rules(
        &[Semantic("attribute"), Semantic("*.attribute")],
        (palette.base(BaseScale::DimFg), FontStyle::Italic),
    );

    builder.add_rule(Textmate("keyword.operator"), palette.base(BaseScale::Fg));

    builder.add_rule(Textmate("markup.heading"), palette.strings());
    builder.add_rule(Textmate("markup.quote"), palette.variables());
    builder.add_rule(Textmate("markup.fenced_code"), palette.properties());
    builder.add_rule(Textmate("markup.inline.raw"), palette.lifetime());
}
