

use std::sync::mpsc::{Receiver};



use log::{info};











use crate::graphics::rendering::{RendererEvent, RenderingInfos, RenderingUpdate};

use crate::graphics::rendering::renderer_state::RendererState;

pub(crate) struct ScionRenderingThread {
    pub(crate) renderer_state: Option<RendererState>,
    pub(crate) render_receiver: Receiver<(Vec<RendererEvent>, Vec<RenderingUpdate>, Vec<RenderingInfos>)>,
}

impl ScionRenderingThread{
    pub(crate) fn run(mut self) {
        info!("Initializing rendering thread");
        loop {
            if let Ok((mut events, updates, rendering_infos)) = self.render_receiver.try_recv() {
                events.drain(0..events.len()).for_each(|event|{
                    match event {
                        RendererEvent::ForceRedraw => {
                            // TODO
                        }
                        RendererEvent::Resize(physical_size, scale_factor) => {
                            self.renderer_state.as_mut().unwrap().resize(physical_size, scale_factor);
                        }
                    }
                });

                if !updates.is_empty() || !rendering_infos.is_empty() {
                    self.renderer_state.as_mut().unwrap().update(updates);
                    match self.renderer_state.as_mut().unwrap().render(rendering_infos) {
                        Ok(_) => {}
                        Err(e) => log::error!("{:?}", e),
                    }
                }
            }
        }
    }

    pub fn new(renderer_state: Option<RendererState>, render_receiver: Receiver<(Vec<RendererEvent>, Vec<RenderingUpdate>, Vec<RenderingInfos>)>) -> Self{
        Self{
            renderer_state,
            render_receiver,
        }
    }
}


