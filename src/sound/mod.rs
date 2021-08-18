use std::{io::Cursor};

use audrey::{
    Reader,
};



pub(crate) mod bindings;
pub(crate) mod context;
pub(crate) mod controller;

pub const CHANNELS: u32 = 2;
pub const RATE: u32 = 44100;
pub const MONO: u32 = 1;
pub const STEREO: u32 = 2;

/// Parse ogg
pub(crate) fn parse_ogg(bytes: Vec<u8>) -> Result<Reader<Cursor<Vec<u8>>>, ()> {
    let cursor = Cursor::new(bytes);
    match audrey::Reader::new(cursor) {
        Ok(audio) => {
            if audio.description().sample_rate() != RATE {
                return Err(());
            }
            if audio.description().channel_count() != MONO
                && audio.description().channel_count() != STEREO
            {
                return Err(());
            }
            Ok(audio)
        }
        Err(_) => Err(()),
    }
}
