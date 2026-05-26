use ratatui::{DefaultTerminal, Frame};

pub struct CargoPlay;

impl CargoPlay {
    pub fn run() -> color_eyre::Result<()> {
        ratatui::run(Self::app)?;
        Ok(())
    }

    fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(Self::render)?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }
    fn render(frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }
}
