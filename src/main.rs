use std::time::Duration;
use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for i in 0..10 {
        // Add a dummy source of the sake of the example.
        let source = SineWave::new(100.0*i as f32)
            .take_duration(Duration::from_secs_f32(1.0)).amplify(0.40);
        sink.append(source);

        sink.sleep_until_end();
    }
}
