use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sample, Sink, source::Source};
use rodio::source::SamplesConverter;
use crate::core::audio_controller;
use crate::core::audio_controller::AudioController;
use crate::core::resources::inputs::types::KeyCode::N;
use crate::utils::file::{open_file, read_file};

/// `AudioPlayer` is the resource responsible to handle musics, sound effects, and action on them
pub struct Audio {
    sounds_data: HashMap<String, Sound>,
    system_ready: bool,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    event_sender: mpsc::Sender<AudioEvent>,
    sounds_cursor: usize,
}

impl Audio {
    pub(crate) fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let (event_sender, receiver) = mpsc::channel();

        std::thread::spawn(move || unsafe {
            audio_controller::audio_thread(AudioController::new(receiver))
        });

        Audio {
            sounds_data: HashMap::default(),
            system_ready: false,
            stream_handle,
            sink,
            event_sender,
            sounds_cursor: 0,
        }
    }

    /// Start to play the sound identified with `name`
    pub fn play(&mut self, path: String, config: PlayConfig) -> Result<usize, Error> {
        let sound_id = self.sounds_cursor;
        if let Ok(()) = self.event_sender.send(AudioEvent::PlaySound { path, config, sound_id }) {
            self.sounds_cursor += 1;
            return Ok(sound_id);
        }
        return Err(Error::ImpossibleToLoadSound);
    }
}


/// Error that can be thrown by the AudioPlayer
#[derive(Debug)]
pub enum Error {
    SoundNotRegistered,
    SoundAlreadyExists,
    ImpossibleToLoadSound,
}

#[derive(Clone)]
pub struct Sound {
    pub(crate) file_path: String,
}

impl Sound {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

/// `PlayConfig` describe how sound must be played
pub struct PlayConfig {
    /// Volume of the sound (should be between 0 and 1)
    pub volume: f32,
    /// Should this sound be looped
    pub looped: bool,
    /// Category of the sound. Usefull when you want to be able to change volume of a given category of sounds
    pub category: Option<String>,
}

impl Default for PlayConfig{
    fn default() -> Self {
        Self{
            volume: 0.2,
            looped: false,
            category: None
        }
    }
}

/// `AudioEvent` represents events send from the audio controller to the Audio Thread
pub(crate) enum AudioEvent {
    PlaySound { path: String, config: PlayConfig, sound_id: usize },
    StopSound { sound_id: usize },
    StopAllSounds,
}