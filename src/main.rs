use std::io::{stdout, Result, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crossterm::cursor::MoveToNextLine;
use glam::IVec2;

use crossterm::queue;
use crossterm::{
    execute, 
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, 
    terminal::{Clear, ClearType, EnterAlternateScreen},
    cursor
};

fn init_terminal() -> Result<()> {
    execute!(
        stdout(),
        EnterAlternateScreen,
        cursor::Hide,
    )?;

    Ok(())
}

const screen_size: IVec2 = IVec2::new(200, 45);

fn main() -> Result<()> {
    let mut n = 0;

    let dt = 1.0;// / 60.0;

    init_terminal()?;

    let mut stdout = stdout();

    loop {
        let frame_start = SystemTime::now();
        
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print(n.to_string() + "░▒▓█"),
            cursor::MoveTo(1, 1),
        )?;

        for _ in 0..screen_size.x {
            queue!(stdout, Print("-"))?;
        }

        queue!(stdout, 
            cursor::MoveToNextLine(1),
        )?;

        for i in 0..screen_size.y {
            queue!(stdout,
                Print("|")
            )?;

            for _ in 0..screen_size.x {
                queue!(
                    stdout,
                    Print(" ")
                )?;
            }
            queue!(stdout,
                Print("|".to_owned() + &(i+1).to_string()),
                cursor::MoveToNextLine(1),
            )?;
        }

        queue!(stdout, 
            cursor::MoveRight(1),
        )?;

        for _ in 0..screen_size.x {
            queue!(stdout, Print("-"))?;
        }


        n += 1;

        let elapsed = frame_start.elapsed().unwrap();

        queue!(stdout, MoveToNextLine(1), Print(elapsed.as_millis()))?;

        stdout.flush()?;

        sleep(Duration::from_secs_f64(dt) - elapsed);
    }
    
    Ok(())
}
