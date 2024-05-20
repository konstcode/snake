use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use log::info;
use rodio::{Decoder, OutputStream, Sink};

static AUDIO_DIR: &str = "audio";

pub struct Records {
    all: HashMap<String, PathBuf>,
}

impl Records {
    pub fn new() -> Self {
       let records = Records {
           all: HashMap::new(),
       };
       records
    }

    pub fn init(&mut self) {
        info!("Loading all audio files...");
        fs::read_dir(AUDIO_DIR).unwrap().for_each(|entry| {
            let entry = entry.unwrap();
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    self.add(&entry.path());
                }
            }
        });
    }

    fn add(& mut self, path: &PathBuf) {
        let string = path.file_stem().unwrap().to_str().unwrap();
        self.all.insert(String::from(string), path.to_owned());
        println!("{:?}", self.all);
    }

    pub fn play(&self, name: &str) {
        let (_stream, stream_handle) = &OutputStream::try_default().unwrap();
        let path = self.all.get(&String::from(name)).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);

        sink.play();
        sink.sleep_until_end();
    }
}