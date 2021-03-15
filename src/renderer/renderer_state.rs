use std::collections::HashMap;
use crate::renderer::bidimensional::triangle::triangle_pipeline;
use winit::window::Window;
use crate::renderer::ScionRenderer;
use winit::event::WindowEvent;

pub struct RendererState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipelines: HashMap<String,wgpu::RenderPipeline>,
    scion_renderer: Box<dyn ScionRenderer>
}

impl RendererState {
    pub(crate) async fn new(window: &Window, scion_renderer: Box<ScionRenderer>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let mut pipelines = HashMap::new();

        pipelines.insert("triangle".to_string(),triangle_pipeline(&device, &sc_desc));

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipelines: pipelines,
            scion_renderer
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        //todo!()
        false
    }

    pub(crate) fn update(&mut self) {
        //todo!()
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self
            .swap_chain
            .get_current_frame()?
            .output;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.5,
                                g: 0.2,
                                b: 0.7,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipelines.get("triangle").unwrap()); // 2.
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}