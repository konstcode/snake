use std::error::Error;
use snake::audio::Records;

fn main() ->  Result<(), Box<dyn Error>> {

    let records = Records::init()?;
    let _ = records.play("win");
    Ok(())
}
