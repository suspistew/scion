use std::sync::Arc;
use log::info;
use wgpu::{Backends, CompositeAlphaMode, InstanceDescriptor, Surface, SurfaceConfiguration, TextureFormat};
use winit::{event::WindowEvent, window::Window};

use crate::{config::scion_config::ScionConfig, rendering::ScionRenderer};
use crate::core::world::GameData;

pub(crate) struct RendererState {
    surface: Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    scion_renderer: Box<dyn ScionRenderer>,
}

impl RendererState {
    pub(crate) async fn new(window: Arc<Window>, mut scion_renderer: Box<dyn ScionRenderer>) -> Self {
        let mut size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        info!("width height {}/{}", width, height);

        let backends = Backends::all();
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::util::gles_minor_version_from_env().unwrap_or_default();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            flags: wgpu::InstanceFlags::from_build_config().with_env(),
            dx12_shader_compiler,
            gles_minor_version,
        });

        let surface = instance.create_surface(window).expect("Surface creation failed");
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let mut config = surface
            .get_default_config(&adapter, width, height)
            .unwrap();

        surface.configure(&device, &config);
        scion_renderer.start(&device, &config);

        Self { surface, device, queue, config, scion_renderer }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, _scale_factor: f64) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub(crate) fn _input(&mut self, _event: &WindowEvent) -> bool {
        //todo!()
        false
    }

    pub(crate) fn update(&mut self, data: &mut GameData) {
        self.scion_renderer.update(data, &self.device, &self.config, &mut self.queue);
    }

    pub(crate) fn render(
        &mut self,
        data: &mut GameData,
        config: &ScionConfig,
    ) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.scion_renderer.render(data, config, &view, &mut encoder);

        self.queue.submit(Some(encoder.finish()));

        frame.present();
        Ok(())
    }
}
