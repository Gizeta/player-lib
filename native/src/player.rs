use rodio::{ Decoder, Sink };
use std::io::BufReader;

pub enum AudioSource {
    File(String),
}

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new() -> Player {
        let device = rodio::default_output_device().expect("Fail to get default output device");
        Player {
            sink: Sink::new(&device),
        }
    }

    pub fn append(&self, source: AudioSource) {
        match source {
            AudioSource::File(file) => {
                let file = std::fs::File::open(file).unwrap();
                self.sink.append(Decoder::new(BufReader::new(file)).unwrap());
            },
        }
    }

    pub fn block(&self) {
        self.sink.sleep_until_end();
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}
