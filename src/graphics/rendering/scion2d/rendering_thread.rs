use std::sync::mpsc::{Receiver};
use log::{info};
use crate::graphics::rendering::{RendererEvent, RenderingInfos, RenderingUpdate};
use crate::graphics::rendering::scion2d::window_rendering_manager::ScionWindowRenderingManager;

pub(crate) struct ScionRenderingThread {
    pub(crate) window_rendering_manager: Option<ScionWindowRenderingManager>,
    pub(crate) render_receiver: Receiver<(Vec<RendererEvent>, Vec<RenderingUpdate>, Vec<RenderingInfos>)>,
}

impl ScionRenderingThread{
    pub(crate) fn run(mut self) {
        info!("Initializing rendering thread");
        loop {
            if let Ok((mut events, updates, rendering_infos)) = self.render_receiver.recv() {
                events.drain(0..events.len()).for_each(|event|{
                    match event {
                        RendererEvent::ForceRedraw => {
                            // TODO
                        }
                        RendererEvent::Resize(physical_size, scale_factor) => {
                            self.window_rendering_manager.as_mut().unwrap().resize(physical_size, scale_factor);
                        }
                    }
                });

                if !updates.is_empty() || !rendering_infos.is_empty() {
                    self.window_rendering_manager.as_mut().unwrap().update(updates);
                    match self.window_rendering_manager.as_mut().unwrap().render(rendering_infos) {
                        Ok(_) => {}
                        Err(e) => log::error!("{:?}", e),
                    }
                }
            }
        }
    }

    pub fn new(window_rendering_manager: Option<ScionWindowRenderingManager>, render_receiver: Receiver<(Vec<RendererEvent>, Vec<RenderingUpdate>, Vec<RenderingInfos>)>) -> Self{
        Self{
            window_rendering_manager,
            render_receiver,
        }
    }
}


