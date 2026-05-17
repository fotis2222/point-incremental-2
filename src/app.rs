use std::io;
use crate::{game::{game::Game}, ui};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    MainScreen,
    Shop,
}

#[derive(Debug)]
pub struct App {
    pub game: Game,
    pub screen: Screen,
    exit: bool
}

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            screen: Screen::MainScreen,
            exit: false
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let _ = terminal.draw(|frame| self.draw(frame));
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        match self.screen {
            Screen::MainScreen => ui::main::render(self, frame),
            Screen::Shop => ui::shop::render(self, frame),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.screen {
            Screen::MainScreen => ui::main::handle_input(self, key_event),
            Screen::Shop => ui::shop::handle_input(self, key_event),
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
