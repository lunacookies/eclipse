mod colors;
mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let mut builder = ThemeBuilder::new("Eclipse Dark".to_string(), Type::Dark);
    imp::add_rules(&mut builder, &colors::Dark);
    builder.build().save()?;

    Ok(())
}
