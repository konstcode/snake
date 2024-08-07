use log::{error, info};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// This simple audio system with adjustable amount of sinks(channels),
/// took basic idea from rust_audio crate.
const MAX_CHANNELS: usize = 4;

pub struct Audio {
    tracks: HashMap<String, PathBuf>,
    sinks: Vec<Sink>,
    current: usize,
    stream: Option<(OutputStream, OutputStreamHandle)>,
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

impl Audio {
    pub fn new() -> Self {
        if let Ok(stream) = OutputStream::try_default() {
            let mut sinks: Vec<Sink> = Vec::new();
            for _ in 0..MAX_CHANNELS {
                let sink = Sink::try_new(&stream.1)
                    .unwrap_or_else(|_| panic!("Can't create audio channel."));
                sinks.push(sink);
            }
            Self {
                tracks: HashMap::new(),
                sinks,
                stream: Some(stream),
                current: 0,
            }
        } else {
            Self {
                tracks: HashMap::new(),
                sinks: Vec::new(),
                current: 0,
                stream: None,
            }
        }
    }

    fn disabled(&self) -> bool {
        self.stream.is_none()
    }

    pub fn init<P: AsRef<Path> + Display>(&mut self, audio_dir: P) {
        if self.disabled() {
            return;
        }
        info!("Loading all audio files...");

        match fs::read_dir(&audio_dir) {
            Ok(read_dir) => read_dir.for_each(|entry| {
                let entry = entry.unwrap();
                let path: PathBuf = entry.path();
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        self.add(path);
                    }
                }
            }),
            Err(e) => error!("{}", format!("{}: {}", e.to_string(), audio_dir)),
        };
    }

    fn add(&mut self, path: PathBuf) {
        let string = String::from(path.file_stem().unwrap().to_str().unwrap());
        self.tracks.insert(string, path);
    }

    pub fn play<S: AsRef<str>>(&mut self, name: S) {
        if self.disabled() {
            return;
        }
        let path = match self.tracks.get(name.as_ref()) {
            Some(val) => val,
            None => {
                error!(
                    "{}",
                    format!("No audio file with such name: {}.", name.as_ref())
                );
                return;
            }
        };

        let file = BufReader::new(fs::File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        let current_sink = &self.sinks[self.current];

        self.current += 1;
        if self.current >= MAX_CHANNELS {
            self.current = 0
        }

        current_sink.append(source);
        current_sink.play();
    }

    pub fn wait(&self) {
        if self.disabled() {
            return;
        }
        loop {
            if self.sinks.iter().any(|i| !i.empty()) {
                std::thread::sleep(std::time::Duration::from_millis(50));
                continue;
            }
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    const AUDIO_FILE: &str = "enter";

    #[test]
    fn test_audio_new() {
        let audio = Audio::new();
        assert_eq!(
            audio.sinks.len(),
            MAX_CHANNELS,
            "Sinks same number as channels defined."
        );
        assert!(audio.stream.is_some(), "Audio device connected.");
        assert!(!audio.disabled(), "Disabled return false.");
    }

    #[test]
    fn test_audio_initialization_and_add() {
        let mut audio = Audio::new();
        audio.add(PathBuf::from(format!("audio/{}.wav", AUDIO_FILE)));
        assert_eq!(audio.tracks.len(), 1, "Have correct number of tracks");

        let check_win_exist = |audio: &mut Audio| {
            let str_path = audio.tracks.get(AUDIO_FILE).unwrap().to_str();
            assert_eq!(
                str_path,
                Some(format!("audio/{}.wav", AUDIO_FILE)).as_deref(),
                "Check if we can get path to file by name."
            );
        };
        check_win_exist(&mut audio);
        audio.tracks.clear();

        let files_count = fs::read_dir("audio").unwrap().count();
        audio.init("audio");
        assert_eq!(
            audio.tracks.len(),
            files_count,
            "Files in audio dir same as tracks."
        );
        check_win_exist(&mut audio);
    }

    #[test]
    fn test_play() {
        let mut audio = Audio::new();
        audio.init("audio");
        assert!(audio.sinks[0].empty(), "First sink is empty at start.");
        audio.play(AUDIO_FILE);
        assert!(
            !audio.sinks[0].empty(),
            "After first play, first sink have song."
        );
        assert_eq!(audio.current, 1, "Current sink changes to second.");
        for _ in 1..MAX_CHANNELS {
            audio.play(AUDIO_FILE);
        }
        assert_eq!(
            audio.current, 0,
            "After play from all channels, current become 0."
        );
    }

    #[test]
    fn test_wait() {
        let mut audio = Audio::new();
        audio.init("audio");
        let start = Instant::now();
        audio.play(AUDIO_FILE);
        audio.wait();
        let duration = start.elapsed();
        assert!(duration.as_millis() > 200);
    }
}
