use legion::{Resources, World};
use wgpu::{SurfaceConfiguration, TextureViewDescriptor};
use winit::{event::WindowEvent, window::Window};

use crate::{config::scion_config::ScionConfig, rendering::ScionRenderer};

pub(crate) struct RendererState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    scion_renderer: Box<dyn ScionRenderer>,
}

impl RendererState {
    pub(crate) async fn new(window: &Window, mut scion_renderer: Box<dyn ScionRenderer>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, /* Trace path */
            )
            .await
            .unwrap();

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width * window.scale_factor() as u32,
            height: size.height * window.scale_factor() as u32,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        scion_renderer.start(&device, &config);

        Self { surface, device, queue, config, scion_renderer }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub(crate) fn _input(&mut self, _event: &WindowEvent) -> bool {
        //todo!()
        false
    }

    pub(crate) fn update(&mut self, world: &mut World, resources: &mut Resources) {
        self.scion_renderer.update(world, resources, &self.device, &self.config, &mut self.queue);
    }

    pub(crate) fn render(
        &mut self,
        world: &mut World,
        config: &ScionConfig,
    ) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_frame()?.output;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.scion_renderer.render(world, config, &view, &mut encoder);
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
