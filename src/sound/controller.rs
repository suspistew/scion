use std::{
    collections::HashMap,
    io::Cursor,
    sync::mpsc,
    thread,
};

use audrey::{
    Reader,
};

use crate::{
    core::resources::sound::{PlayConfig, Sound},
    sound::{parse_ogg, CHANNELS},
    utils::file::read_file,
};

pub enum AudioEvent {
    AddSound { sound_id: usize, data: Reader<Cursor<Vec<u8>>> },
    PlaySound { sound_id: usize, config: PlayConfig },
    StopSound { sound_id: usize },
    SystemReady,
}

static mut VEC: Vec<(usize, [f32; 2])> = vec![];

struct SoundState {
    config: PlayConfig,
    sample_cursor: usize,
}

pub(crate) struct AudioController {
    receiver: mpsc::Receiver<AudioEvent>,
    sounds: HashMap<usize, Reader<Cursor<Vec<u8>>>>,
    loaded_sounds: HashMap<usize, Vec<[f32; 2]>>,
    currently_played_sounds: HashMap<usize, SoundState>,
    system_is_ready: bool,
}

impl AudioController {
    pub(crate) fn new(receiver: mpsc::Receiver<AudioEvent>) -> AudioController {
        AudioController {
            receiver,
            sounds: HashMap::new(),
            loaded_sounds: Default::default(),
            currently_played_sounds: Default::default(),
            system_is_ready: false,
        }
    }

    /// Handles events from the AudioMessage queue
    pub(crate) fn handle_events(&mut self) {
        if let Ok(message) = self.receiver.try_recv() {
            match message {
                AudioEvent::SystemReady => self.system_is_ready = true,
                AudioEvent::AddSound { sound_id, data } => {
                    self.sounds.insert(sound_id, data);
                }
                AudioEvent::PlaySound { sound_id, config } => {
                    self.currently_played_sounds
                        .insert(sound_id, SoundState { config, sample_cursor: 0 });
                }
                AudioEvent::StopSound { sound_id } => {
                    self.currently_played_sounds.remove(&sound_id);
                }
            }
        }
    }

    /// responsible to drain sounds that needs to be loaded and creates a thread
    /// that will add to a static vec the data.
    /// Then, drain the vec to add data to the loaded sounds
    pub(crate) unsafe fn load_unload_sounds(&mut self) {
        self.sounds.drain().for_each(|(id, mut data)| {
            thread::spawn(move || {
                let channel = data.description().channel_count();
                let mut samples = data.samples::<f32>().map(|e| e.unwrap());
                match channel {
                    super::MONO => { while let Some(sample) = samples.next() {
                        VEC.push((id, [sample, sample]));
                    }}
                    super::STEREO => unsafe {
                        while let (Some(sample_left), Some(sample_right)) =
                            (samples.next(), samples.next())
                        {
                            VEC.push((id, [sample_left, sample_right]));
                        }
                    },
                    _ => {}
                }
            });
        });
        VEC.drain(0..).for_each(|(id, sample)| {
            self.loaded_sounds.entry(id).or_insert(Vec::new()).push(sample);
        });
    }

    /// Add data to the OS audio buffer according to the frames number
    pub(crate) fn fill_audio_buffer(&mut self, buffer: &mut [f32], frames: usize) {
        if !self.system_is_ready {
            return;
        }

        let (sounds, currently_played_sounds) = self.split_loaded_and_playing();
        for frame in 0..frames {
            let (mut left_channel, mut right_channel) = (0., 0.);
            currently_played_sounds.retain(|sound_id, state| {
                match sounds.get(sound_id) {
                    Some(sound_data) => {
                        left_channel += sound_data[state.sample_cursor][0] * 1.;
                        right_channel += sound_data[state.sample_cursor][1] * 1.;
                        state.sample_cursor += 1;
                        if state.sample_cursor >= sound_data.len() {
                            false
                        } else {
                            true
                        }
                    }
                    None => {
                        log::error!(
                            "A sound can't be played because it's not present in the datas"
                        );
                        true
                    }
                }
            });
            buffer[CHANNELS as usize * frame as usize] = left_channel;
            buffer[CHANNELS as usize * frame as usize + 1] = right_channel;
        }
    }

    fn split_loaded_and_playing(
        &mut self,
    ) -> (&mut HashMap<usize, Vec<[f32; 2]>>, &mut HashMap<usize, SoundState>) {
        (&mut self.loaded_sounds, &mut self.currently_played_sounds)
    }
}
