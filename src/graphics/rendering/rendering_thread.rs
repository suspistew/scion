use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;
use hecs::Entity;
use log::{debug, info};
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPipeline};
use crate::core::audio_controller::AudioController;
use crate::graphics::rendering::renderer_state::RendererState;
use crate::graphics::rendering::{RenderingInfos, RenderingUpdate};
use crate::graphics::rendering::gl_representations::GlUniform;

pub(crate) fn rendering_thread( renderer_state: Option<RendererState>,
                                render_receiver: Receiver<(Vec<RenderingUpdate>, Vec<RenderingInfos>)>) {

    info!("Initializing rendering thread");
    loop {
        if let Ok(message) = render_receiver.try_recv() {
            info!("received rendering infos {} / {}", message.0.len(), message.1.len());

        }
    }
}


