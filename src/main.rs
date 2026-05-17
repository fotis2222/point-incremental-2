use std::io;
use point_incremental::app::*;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
