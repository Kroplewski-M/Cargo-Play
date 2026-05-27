use crate::error::Result;
use ratatui::DefaultTerminal;
use std::panic;

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

pub fn teardown() -> Result<()> {
    ratatui::restore();
    Ok(())
}
