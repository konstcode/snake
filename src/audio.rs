use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use log::{error, info};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

/// This simple audio system with adjustable amount of sinks,
/// took basic idea from rust_audio crate.
const MAX_CHANNELS: usize = 4;

pub struct Audio {
    tracks: HashMap<String, PathBuf>,
    sinks: Vec<Sink>,
    current: usize,
    stream: Option<(OutputStream, OutputStreamHandle)>,
}

impl Audio {
    pub fn new() -> Self {
       if let Ok(stream) = OutputStream::try_default() {
           let mut sinks: Vec<Sink> = Vec::new();
           for _ in 0..MAX_CHANNELS {
               let sink = Sink::try_new(&stream.1).unwrap_or_else(|_| {
                   panic!("Can't create audio channel.")
               });
               sinks.push(sink);
           }
           Self {
               tracks: HashMap::new(),
               sinks: sinks,
               stream: Some(stream),
               current: 0,
           }
       }
       else {
           Self {
               tracks: HashMap::new(),
               sinks: Vec::new(),
               current: 0,
               stream: None,
           }
       }
    }

    pub fn init<P: AsRef<Path> + Display>(&mut self, audio_dir: P) {
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
        let path = match self.tracks.get(name.as_ref()) {
            Some(val) => val,
            None => { error!("{}", format!("No audio file with such name: {}.", name.as_ref())); return },
        };

        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        println!("before current sink: {}", self.current);
        let current_sink = &self.sinks[self.current];

        if (self.current < MAX_CHANNELS - 1) { self.current += 1; } else { self.current = 0 }
        println!("current sink: {}", self.current);

        current_sink.append(source);
        current_sink.play();
    }
}