// This comes from https://github.com/not-fl3/quad-snd/

use quad_alsa_sys as sys;

use crate::sound::{controller::AudioController, CHANNELS, RATE};

mod consts {
    pub const DEVICE: &'static str = "default\0";
    pub const PCM_BUFFER_SIZE: u64 = 4096;
}

unsafe fn setup_pcm_device() -> *mut sys::snd_pcm_t {
    let mut pcm_handle = std::ptr::null_mut();

    // Open the PCM device in playback mode
    if sys::snd_pcm_open(
        &mut pcm_handle,
        consts::DEVICE.as_ptr() as _,
        sys::SND_PCM_STREAM_PLAYBACK,
        0,
    ) < 0
    {
        panic!("Can't open PCM device.");
    }

    let mut hw_params: *mut sys::snd_pcm_hw_params_t = std::ptr::null_mut();
    sys::snd_pcm_hw_params_malloc(&mut hw_params);
    sys::snd_pcm_hw_params_any(pcm_handle, hw_params);

    if sys::snd_pcm_hw_params_set_access(pcm_handle, hw_params, sys::SND_PCM_ACCESS_RW_INTERLEAVED)
        < 0
    {
        panic!("Can't set interleaved mode");
    }

    if sys::snd_pcm_hw_params_set_format(pcm_handle, hw_params, sys::SND_PCM_FORMAT_FLOAT_LE) < 0 {
        panic!("Can't set SND_PCM_FORMAT_FLOAT_LE format");
    }
    if sys::snd_pcm_hw_params_set_buffer_size(pcm_handle, hw_params, consts::PCM_BUFFER_SIZE) < 0 {
        panic!("Cant's set buffer size");
    }
    if sys::snd_pcm_hw_params_set_channels(pcm_handle, hw_params, CHANNELS) < 0 {
        panic!("Can't set channels number.");
    }

    let mut rate = RATE;
    if sys::snd_pcm_hw_params_set_rate_near(pcm_handle, hw_params, &mut rate, std::ptr::null_mut())
        < 0
    {
        panic!("Can't set rate.");
    }

    // Write parameters
    if sys::snd_pcm_hw_params(pcm_handle, hw_params) < 0 {
        panic!("Can't set hardware parameters.");
    }
    sys::snd_pcm_hw_params_free(hw_params);

    // tell ALSA to wake us up whenever AudioContext::PCM_BUFFER_SIZE or more frames
    //   of playback data can be delivered. Also, tell
    //   ALSA that we'll start the device ourselves.
    let mut sw_params: *mut sys::snd_pcm_sw_params_t = std::ptr::null_mut();

    if sys::snd_pcm_sw_params_malloc(&mut sw_params) < 0 {
        panic!("cannot allocate software parameters structure");
    }
    if sys::snd_pcm_sw_params_current(pcm_handle, sw_params) < 0 {
        panic!("cannot initialize software parameters structure");
    }

    if sys::snd_pcm_sw_params_set_start_threshold(pcm_handle, sw_params, 0) < 0 {
        panic!("cannot set start mode");
    }
    if sys::snd_pcm_sw_params(pcm_handle, sw_params) < 0 {
        panic!("cannot set software parameters");
    }
    sys::snd_pcm_sw_params_free(sw_params);

    if sys::snd_pcm_prepare(pcm_handle) < 0 {
        panic!("cannot prepare audio interface for use");
    }

    pcm_handle
}

pub(crate) unsafe fn audio_thread(mut controller: AudioController) {
    let mut buffer: Vec<f32> = vec![0.0; consts::PCM_BUFFER_SIZE as usize * 2];

    let pcm_handle = setup_pcm_device();

    loop {
        controller.handle_events();

        let frames_to_deliver = consts::PCM_BUFFER_SIZE as i64;
        controller.load_unload_sounds();
        controller.fill_audio_buffer(&mut buffer, frames_to_deliver as usize);

        // send filled buffer back to alsa
        let frames_writen =
            sys::snd_pcm_writei(pcm_handle, buffer.as_ptr() as *const _, frames_to_deliver as _);
        if frames_writen == -libc::EPIPE as i64 {
            println!("Underrun occured: -EPIPE, attempting recover");
            sys::snd_pcm_recover(pcm_handle, frames_writen as _, 0);
        }

        if frames_writen > 0 && frames_writen != frames_to_deliver {
            println!("Underrun occured: frames_writen != frames_to_deliver, attempting recover");
            sys::snd_pcm_recover(pcm_handle, frames_writen as _, 0);
        }
    }
}
