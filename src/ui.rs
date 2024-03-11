use std::io::{self, stdout};

use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{block::Position, *},
};

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(Local::now().format("%H:%M:%S").to_string())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" Pomodoro Timer (press 'q' to quit) ")
                    .title_position(Position::Bottom)
                    .title_alignment(Alignment::Center)
                    .cyan()
                    .border_style(Style::new().blue())
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::LightRed)),
            ),
        frame.size(),
    );
}

pub fn render_ui() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
