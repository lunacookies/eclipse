mod colors;
mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let mut eclipse = ThemeBuilder::new("Eclipse".to_string(), Type::Dark);
    imp::add_rules(&mut eclipse, &colors::Dark);
    eclipse.build().save()?;

    Ok(())
}
