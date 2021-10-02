use std::{collections::HashMap, path::Path};

use crate::{
    sound::{context::AudioContext, parse_ogg},
    utils::file::read_file,
};

#[derive(Debug)]
pub enum Error {
    SoundNotRegistered,
    SoundAlreadyExists,
    ImpossibleToLoadSound,
}

/// `SoundLoadingType` describe how sound will be kept in memory
#[derive(Clone, Debug)]
pub enum SoundLoadingType {
    /// Loads the sound immediately and keep it in memory during all the game process
    AlwaysInMemory,
    /// Loads the sound in memory only when it is needed, and keeps it in memory
    KeepAfterUse,
}

/// `PlayConfig` describe how sound must be played
#[derive(Default)]
pub struct PlayConfig {}

#[derive(Clone)]
pub struct Sound {
    pub(crate) file_path: String,
    pub(crate) loading_type: SoundLoadingType,
}

impl Sound {
    pub fn new(file_path: String, loading_type: SoundLoadingType) -> Self {
        Self { file_path, loading_type }
    }
}

/// `AudioPlayer` is the resource responsible to handle loaded musics, sound effects
pub struct AudioPlayer {
    context: AudioContext,
    sounds_data: HashMap<String, Sound>,
    loaded_sounds: HashMap<String, usize>,
    system_ready: bool,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self {
            context: AudioContext::new(),
            sounds_data: Default::default(),
            loaded_sounds: Default::default(),
            system_ready: false,
        }
    }
}

impl AudioPlayer {
    /// Register a sound in the player, and identify it with `name` key
    pub fn register_sound(&mut self, name: &str, sound: Sound) -> Result<(), Error> {
        if self.sounds_data.contains_key(name) {
            Err(Error::SoundAlreadyExists)
        } else {
            let should_load = if let SoundLoadingType::AlwaysInMemory = sound.loading_type {
                true
            } else {
                false
            };
            self.sounds_data.insert(name.to_string(), sound);
            if should_load {
                self.load_sound_in_memory(name)?;
            }
            Ok(())
        }
    }

    /// Start to play the sound identified with `name`
    pub fn play(&mut self, name: &str, config: PlayConfig) -> Result<(), Error> {
        if !self.loaded_sounds.contains_key(name) {
            if self.sounds_data.contains_key(name) {
                self.load_sound_in_memory(name)?;
            } else {
                return Err(Error::SoundNotRegistered);
            }
        }

        if let Some(sound_id) = self.loaded_sounds.get(name) {
            self.context.play_sound(*sound_id, config);
            Ok(())
        } else {
            Err(Error::ImpossibleToLoadSound)
        }
    }

    pub fn stop(&mut self, name: &str) -> Result<(), Error> {
        if let Some(sound_id) = self.loaded_sounds.get(name) {
            self.context.stop_sound(*sound_id);
            Ok(())
        } else {
            Err(Error::SoundNotRegistered)
        }
    }

    pub fn stop_all(&mut self) -> Result<(), Error> {
        self.context.stop_all_sounds();
        Ok(())
    }

    /// This is a way to tell the audio controller that the system is ready.
    /// The system is considered ready to deliver any sound once at least one frame is finished.
    pub(crate) fn system_ready(&mut self) {
        if !self.system_ready {
            self.context.system_ready();
            self.system_ready = true;
        }
    }

    fn load_sound_in_memory(&mut self, name: &str) -> Result<usize, Error> {
        let sound = self
            .sounds_data
            .get(name)
            .expect("Sound loading failed, load_sound_in_memory tried to get unknown sound data");
        match read_file(Path::new(sound.file_path.as_str())) {
            Ok(bytes) => {
                let id = self.context.next_id();
                if let Ok(audio) = parse_ogg(bytes) {
                    self.context.add_sound(id, audio);
                }
                self.loaded_sounds.insert(name.to_string(), id);
                Ok(id)
            }
            Err(_) => Err(Error::ImpossibleToLoadSound),
        }
    }
}
