mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut eclipse = ThemeBuilder::new("Eclipse".to_string(), Type::Dark);
    imp::add_rules(&mut eclipse, &palette);
    eclipse.build().save()?;

    Ok(())
}
