pub mod map;

use figlet_rs::FIGfont;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tui_textarea::TextArea;

const TITLE: &str = "collab :O)";

fn main() -> Result<(), std::io::Error> {
    color_eyre::install().unwrap();

    let mut terminal = ratatui::init();
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());

    let result = run(&mut terminal, &mut textarea);

    ratatui::restore();

    println!("Exited tui-maps. Lines typed: {:?}", textarea.lines());
    result
}

fn run(terminal: &mut DefaultTerminal, textarea: &mut TextArea) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| render(f, textarea))?;

        if let Event::Key(key) = ratatui::crossterm::event::read()? {
            match key.code {
                KeyCode::Esc => break,
                _ => {
                    textarea.input(key);
                }
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, textarea: &TextArea) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
        .split(frame.size());

    let font = FIGfont::from_file("fonts/The Edge.flf").unwrap();
    let ascii_art = font.convert("TUI-COLLAB").unwrap();

    let block = Block::default().title(TITLE).borders(Borders::ALL);
    let paragraph = Paragraph::new(ascii_art.to_string())
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, chunks[0]);
    frame.render_widget(textarea.widget(), chunks[1]);
}
