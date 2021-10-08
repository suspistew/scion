#[cfg(target_os = "linux")]
#[path = "quad_linux_snd.rs"]
pub(crate) mod snd;

#[cfg(target_os = "macos")]
#[path = "dummy_snd.rs"]
pub(crate) mod snd;
