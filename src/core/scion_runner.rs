use std::sync::{Arc, mpsc};
use std::sync::mpsc::Receiver;
use std::thread;
use log::info;

use winit::dpi::{PhysicalSize, Size};
use winit::window::Window;

use crate::core::resources::time::Time;
use crate::core::scene::{SceneAction, SceneMachine};
use crate::core::scheduler::Scheduler;
use crate::core::world::GameData;
use crate::graphics::rendering::renderer_state::RendererState;
use crate::graphics::rendering::scion2d_renderer::scion_renderer::ScionRenderer2D;
use crate::graphics::rendering::{RenderingInfos, RenderingUpdate, ScionRenderer};
use crate::graphics::rendering::rendering_thread::rendering_thread;
use crate::graphics::windowing::WindowingEvent;
use crate::utils::frame_limiter::{FrameLimiter, FrameLimiterConfig};

pub struct ScionRunner {
    pub(crate) game_data: GameData,
    pub(crate) scheduler: Scheduler,
    pub(crate) layer_machine: SceneMachine,
    pub(crate) renderer: Option<RendererState>,
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) main_thread_receiver: Option<Receiver<WindowingEvent>>,
    pub(crate) scion_renderer: ScionRenderer2D,
}

impl ScionRunner {
    pub(crate) fn launch_game_loop(mut self) {
        self.setup();
        let mut frame_limiter = FrameLimiter::new(FrameLimiterConfig::default());
        let (render_sender, render_receiver) = mpsc::channel::<(Vec<RenderingUpdate>, Vec<RenderingInfos>)>();
        let renderer = self.renderer.take();

        thread::spawn(move || { rendering_thread(renderer, render_receiver); });

        loop {
            if frame_limiter.is_min_tick() {
                let frame_duration = self
                    .game_data
                    .get_resource_mut::<Time>()
                    .expect("Time is an internal resource and can't be missing")
                    .frame();
                self.game_data.timers().add_delta_duration(frame_duration);
                self.layer_machine.apply_scene_action(SceneAction::Update, &mut self.game_data);
                self.scheduler.execute(&mut self.game_data);
                self.layer_machine.apply_scene_action(SceneAction::LateUpdate, &mut self.game_data);
                self.update_cursor();
            }

            if frame_limiter.is_fixed_update() {
                self.layer_machine.apply_scene_action(SceneAction::FixedUpdate, &mut self.game_data);
                frame_limiter.fixed_tick();
            }

            if frame_limiter.render_unlocked() {
                let updates = self.scion_renderer.prepare_update(&mut self.game_data);
                let rendering_infos = ScionRenderer2D::prepare_rendering(&mut self.game_data);
                let _r = render_sender.send((updates, rendering_infos));
                /*self.renderer.as_mut().unwrap().update(&mut self.game_data);
                match self.renderer.as_mut().unwrap().render(&mut self.game_data) {
                    Ok(_) => {}
                    Err(e) => log::error!("{:?}", e),
                }
                */
                frame_limiter.render();
            }

            if frame_limiter.is_min_tick() {
                self.game_data.inputs().reset_inputs();
                self.game_data.events().cleanup();
                self.layer_machine
                    .apply_scene_action(SceneAction::EndFrame, &mut self.game_data);
                frame_limiter.tick();
            }
        }
    }

    pub(crate) fn setup(&mut self) {
        self.game_data.insert_resource(crate::core::resources::window::Window::new(
            (self.window.as_ref().unwrap().inner_size().width, self.window.as_ref().unwrap().inner_size().height),
            self.window.as_ref().unwrap().scale_factor(),
        ));
        self.layer_machine.apply_scene_action(SceneAction::Start, &mut self.game_data);
    }
    fn update_cursor(&mut self) {
        let mut window = self.game_data.window();
        if let Some(icon) = window.new_cursor() {
            let w = self.window.as_mut().expect("A window is mandatory to run this game !");
            w.set_cursor_icon(*icon);
        }
        if let Some(dimensions) = window.new_dimensions() {
            let w = self.window.as_mut().expect("A window is mandatory to run this game !");
            let _r = w.request_inner_size(Size::Physical(PhysicalSize::new(dimensions.0 * window.dpi() as u32,
                                                                           dimensions.1 * window.dpi() as u32)));
        }
        window.reset_future_settings()
    }
}
