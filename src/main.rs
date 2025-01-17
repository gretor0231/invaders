use std::error::Error;
use std::{io, thread};
use std::io::stdout;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use invaderstest::{frame, render};
use invaderstest::frame::{Drawable, new_frame};
use invaderstest::player::Player;
use invaderstest::render::render;

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


    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move||{
       let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop{
            let curr_frame = match render_rx.recv(){
                Ok(x)=>x,
                Err(x)=>break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });


    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop:loop{

        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Left=> player.move_left(),
                    KeyCode::Right=> player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter=>{
                        if player.shoot(){
                            audio.play("pew")
                        }
                    }
                    KeyCode::Esc| KeyCode::Char('q')=>{
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        //Updates
        player.update(delta);

        // Draw & render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }



    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    stdout().execute(Show)?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
