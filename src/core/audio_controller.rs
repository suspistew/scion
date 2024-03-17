use crate::core::resources::audio::AudioEvent;
use rodio::{OutputStream, Sink, Source};
use std::collections::HashMap;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use log::{debug};

pub(crate) struct AudioController {
    receiver: Receiver<AudioEvent>,
}

impl AudioController {
    pub(crate) fn new(receiver: Receiver<AudioEvent>) -> Self {
        Self { receiver }
    }
}

pub(crate) fn audio_thread(controller: AudioController) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sinks: HashMap<usize, Sink> = HashMap::new();

    loop {
        if let Ok(message) = controller.receiver.try_recv() {
            match message {
                AudioEvent::PlaySound { path, config, sound_id } => {
                    debug!("Started to play sound {}", path);
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    let file = std::fs::File::open(path.as_str()).unwrap();
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    if config.looped {
                        sink.append(source.repeat_infinite());
                    } else {
                        sink.append(source);
                    }
                    // TODO: handle categories
                    sink.set_volume(config.volume);
                    sinks.insert(sound_id, sink);
                }
                AudioEvent::StopSound { sound_id } => {
                    if let Some(sink) = sinks.remove(&sound_id) {
                        sink.stop();
                        drop(sink);
                    }
                }
            }
        }
        sinks.retain(|&_k, sink| {
            !sink.empty()
        });
    }
}
