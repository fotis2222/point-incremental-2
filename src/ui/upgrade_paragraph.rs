use ratatui::{style::Stylize, text::*, Frame, layout::Rect, widgets::{Paragraph, Block, Widget}};

use crate::game::upgrade::Upgrade;

pub fn render_upgrade(upgrade: &Upgrade, area: Rect, frame: &mut Frame) {
    let block = Block::bordered().title(upgrade.name.clone());

    let upgrade_text = Text::from(vec![
       Line::from(vec![
           upgrade.desc.clone().into(),
       ]),
       Line::from(vec![
           "Cost: ".into(),
           upgrade.cost.to_string().bold().yellow(),
           " ".into(),
           upgrade.currency.as_str().bold().blue(),
       ]),
       Line::from(vec![
           "Level ".into(),
           upgrade.level.to_string().light_blue(),
           "/".light_blue(),
           upgrade.max_level.to_string().light_blue(),
       ])
    ]);

    Paragraph::new(upgrade_text)
        .block(block)
        .render(area, frame.buffer_mut());
}
