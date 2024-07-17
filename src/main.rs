use std::error::Error;
use std::io;
use std::io::stdout;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;


fn main() -> Result<(),Box<dyn Error>> {

    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // terminal
    let mut sudout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(Hide);


    // Game Loop


    'gameloop:loop{
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Esc| KeyCode::Char('q')=>{
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }






    //Cleanup
    audio.wait();
    stdout().execute(Show)?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())





}
