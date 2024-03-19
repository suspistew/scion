use wgpu::{CompositeAlphaMode, InstanceDescriptor, Surface, SurfaceConfiguration, TextureFormat};
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
    pub(crate) async fn new(window: &Window, mut scion_renderer: Box<dyn ScionRenderer>) -> Self {
        let _size = window.inner_size();

        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: backend,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        let (_size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).expect("Missed usage surface creation")).expect("Surface unsupported by adapter");
            (size, surface)
        };

        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
                .await
                .expect("No suitable GPU adapters found on the system!");

        let needed_limits =
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits());
        let trace_dir = std::env::var("WGPU_TRACE");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: needed_limits,
                },
                trace_dir.ok().as_ref().map(std::path::Path::new),
            )
            .await
            .expect("Unable to find a suitable GPU adapter!");

        let w = window.inner_size();

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: w.width * window.scale_factor() as u32,
            height: w.height * window.scale_factor() as u32,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![TextureFormat::Bgra8UnormSrgb],
        };
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
