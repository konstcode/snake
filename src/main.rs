use snake::audio::Records;

fn main() {
    let mut audio = Records::new();
    audio.init();
    let _ = audio.play("win");

}
