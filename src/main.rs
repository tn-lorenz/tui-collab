pub mod map;

use figlet_rs::FIGfont;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::Alignment;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

// This should later be dynamically changed from "tui-maps :O)" to whatever project the user is currently working on
const TITLE: &str = "PROJECT-NAME";

fn main() -> Result<(), std::io::Error> {
    color_eyre::install().unwrap();
    
    let terminal = ratatui::init();
    let result = run(terminal);
    
    ratatui::restore();

    println!("Exited tui-maps. Smell 'ya later :O) !");
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break ;
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}

fn render(frame: &mut Frame) {
    //Paragraph::new("Ayo").render(frame.area(), frame.buffer_mut());
    //let font = FIGfont::standard().unwrap();
    let font = FIGfont::from_file("fonts/The Edge.flf").unwrap();
    let ascii_art = font.convert("TUI-MAPS").unwrap();

    let block = Block::default()
        .title(TITLE)
        .borders(Borders::ALL);

    Paragraph::new(ascii_art.to_string())
        .block(block)
        .alignment(Alignment::Center).render(frame.area(), frame.buffer_mut());
}
