use scion::Scion;
use scion::config::scion_config::ScionConfigBuilder;
use scion::config::window_config::WindowConfigBuilder;
use scion::core::components::color::Color;

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_resizable(false)
                    .with_dimensions((768, 768))
                    .with_default_background_color(Some(Color::new_rgb(150, 100, 0)))
                    .get(),
            )
            .get(),
    )
        .run();
}