use crate::tui::CargoPlay;

mod tui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    CargoPlay::run()?;
    Ok(())
}
