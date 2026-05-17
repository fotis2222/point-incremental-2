use crate::app::*;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let title = Line::from(" Point Incremental 2 ".bold());
    let instructions = Line::from(vec![
        " Get Points ".into(),
        "<Enter>".blue().bold(),
        " Shop ".into(),
        "<S>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold()
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![
        Line::from(vec![
            app.game.player.points.to_string().bold().yellow(),
            " Points".into(),
        ]),
        Line::from(vec![
            format!("{} XP", app.game.player.xp).red()
        ]),
        Line::from(vec![
            "(+".into(),
            app.game.player.points_multi.to_string().bold().light_green(),
            " Points, ".into(),
            app.game.player.xp_mult.to_string().red(),
            " XP per click)".into(),
        ])
    ]);

    Paragraph::new(counter_text)
        .centered()
        .block(block)
        .render(area, frame.buffer_mut());
}

pub fn handle_input(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('s') => {
            app.screen = Screen::Shop
        }
        KeyCode::Enter => app.game.player.gain_points(),
        _ => {}
    }
}
