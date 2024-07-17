use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result<(),Box<dyn Error>> {

    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    println!("Hello, world!");

    //cleanup
    audio.wait();
    Ok(())

    //terminal
    // let mut sudout = io::sudout();
    // terminal::enable_raw_mode();
    // sudout.execute(EnterAlternateScreen)?;




}
