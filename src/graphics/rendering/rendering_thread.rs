use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;
use hecs::Entity;
use log::{debug, info};
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, RenderPipeline, SamplerBindingType, SurfaceConfiguration};
use crate::core::audio_controller::AudioController;
use crate::core::components::{Square, Triangle};
use crate::core::components::shapes::line::Line;
use crate::core::components::shapes::polygon::Polygon;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::core::components::tiles::sprite::Sprite;
use crate::core::components::tiles::tilemap::Tilemap;
use crate::core::components::ui::ui_image::UiImage;
use crate::core::components::ui::ui_text::UiTextImage;
use crate::graphics::rendering::renderer_state::RendererState;
use crate::graphics::rendering::{RenderingInfos, RenderingUpdate};
use crate::graphics::rendering::gl_representations::GlUniform;

pub(crate) struct ScionRenderingThread {
    pub(crate) renderer_state: Option<RendererState>,
    pub(crate) render_receiver: Receiver<(Vec<RenderingUpdate>, Vec<RenderingInfos>)>,
}

impl ScionRenderingThread{
    pub(crate) fn run(mut self) {
        info!("Initializing rendering thread");
        loop {
            if let Ok((updates, rendering_infos)) = self.render_receiver.try_recv() {
                self.renderer_state.as_mut().unwrap().update(updates);
                match self.renderer_state.as_mut().unwrap().render(rendering_infos) {
                    Ok(_) => {}
                    Err(e) => log::error!("{:?}", e),
                }
            }
        }
    }

    pub fn new(renderer_state: Option<RendererState>, render_receiver: Receiver<(Vec<RenderingUpdate>, Vec<RenderingInfos>)>) -> Self{
        Self{
            renderer_state,
            render_receiver,
        }
    }
}


