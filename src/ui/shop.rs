use crate::app::*;
use crate::game::game::Upgrades;
use crate::ui::upgrade_paragraph::render_upgrade;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame, layout::*, style::Stylize, symbols::border, text::{Line}, widgets::{Block, Paragraph, Widget}
};

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let title = Line::from(" Shop ");
    let instructions = Line::from(vec![
        " Main ".into(),
        "<M>".blue().bold(),
        " Quit ".into(),
        "<Q>".blue().bold(),
        " Buy Left ".into(),
        "<1>".blue().bold(),
        " Buy Right ".into(),
        "<2> ".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let block_inner_area = block.inner(area);

    // create chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(block_inner_area);

    Paragraph::new("")
        .centered()
        .block(block)
        .render(area, frame.buffer_mut());

    render_upgrade(&app.game.upgrades[Upgrades::PointsI as usize], chunks[0], frame);
    render_upgrade(&app.game.upgrades[Upgrades::XPI as usize], chunks[1], frame);
}

pub fn handle_input(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('m') => {
            app.screen = Screen::MainScreen
        }
        KeyCode::Char('1') => app.game.upgrades[Upgrades::PointsI as usize].buy(&mut app.game.player),
        KeyCode::Char('2') => app.game.upgrades[Upgrades::XPI as usize].buy(&mut app.game.player),
        _ => {}
    }
}
