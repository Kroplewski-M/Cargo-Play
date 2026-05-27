use std::panic;

use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

pub fn set_panic_hook() {
    let original = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        original(info);
    }));
}

pub fn setup() -> Result<DefaultTerminal> {
    let terminal = ratatui::init();
    Ok(terminal)
}

pub fn teardown(terminal: DefaultTerminal) -> Result<()> {
    drop(terminal);
    ratatui::restore();
    Ok(())
}
