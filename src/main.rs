use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use snake::audio::Audio;
static AUDIO_DIR: &str = "audio";
fn main() ->  Result<(), Box<dyn Error>> {

    let mut audio = Audio::new();
    audio.init(AUDIO_DIR);
    audio.play("win");

    sleep(Duration::from_secs(2));
    println!("Hello!");
    Ok(())
}
