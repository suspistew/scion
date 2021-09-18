use std::{io::Cursor, sync::mpsc};

use audrey::Reader;

use crate::{
    core::resources::sound::PlayConfig,
    sound::controller::{AudioController, AudioEvent},
};

/// Must be used only once, allow to create the audio thread
/// and keep a channel sender to communicate
pub(crate) struct AudioContext {
    sender: mpsc::Sender<AudioEvent>,
    counter: usize,
}

impl AudioContext {
    pub(crate) fn new() -> AudioContext {
        let (sender, receiver) = mpsc::channel();
        if cfg!(target_os = "linux") {
            std::thread::spawn(move || unsafe {
                super::bindings::snd::audio_thread(AudioController::new(receiver))
            });
        }
        AudioContext { sender, counter: 0 }
    }

    pub(crate) fn next_id(&mut self) -> usize {
        let res = self.counter;
        self.counter += 1;
        res
    }

    pub(crate) fn add_sound(&self, sound_id: usize, data: Reader<Cursor<Vec<u8>>>) {
        let _r = self.sender.send(AudioEvent::AddSound { sound_id, data });
    }

    pub(crate) fn play_sound(&mut self, sound_id: usize, config: PlayConfig) {
        let _r = self.sender.send(AudioEvent::PlaySound { sound_id, config });
    }

    pub(crate) fn stop_sound(&mut self, sound_id: usize) {
        let _r = self.sender.send(AudioEvent::StopSound { sound_id });
    }

    pub(crate) fn system_ready(&self) { let _r = self.sender.send(AudioEvent::SystemReady); }
}
