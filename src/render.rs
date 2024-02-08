use super::body::Body;
use super::simulation::SimComponent;

use std::io::{stdout, Result, Stdout, Write};

use crossterm::terminal::size;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::{queue, terminal};

pub struct TerminalRender {
    stdout: Stdout,
}

impl TerminalRender {
    pub fn new() -> TerminalRender {
        TerminalRender { stdout: stdout() }
    }

    fn init_terminal(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            EnterAlternateScreen,
            cursor::Hide,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )
    }

    fn close_terminal(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            LeaveAlternateScreen,
            cursor::Show,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )
    }

    fn render_bodies(&mut self, bodies: &mut Vec<Body>) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All),)?;

        let (size_x, size_y) = terminal::size().unwrap();

        for body in bodies.iter() {
            let x = (body.position.x + size_x as f32 / 2.0) as u16;
            let y = (body.position.y + size_y as f32 / 2.0) as u16;

            if x <= 0 || y <= 0 || x > size_x || y > size_y {
                continue;
            }

            queue!(
                self.stdout,
                cursor::MoveTo(x, y),
                SetBackgroundColor(Color::Red),
                Print(" "),
                ResetColor,
            )?;
        }

        queue!(self.stdout, cursor::MoveTo(0, 0))?;

        self.stdout.flush()?;

        Ok(())
    }
}

impl SimComponent for TerminalRender {
    fn start(&mut self) {
        self.init_terminal()
            .expect("Error in terminal initalization!");
    }

    fn update(&mut self, bodies: &mut Vec<Body>) {
        self.render_bodies(bodies).expect("Error rendering bodies!");
    }

    fn finish(&mut self) {
        self.close_terminal().expect("Error closing terminal!");
    }
}
