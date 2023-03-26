use scion::config::scion_config::ScionConfigBuilder;
use scion::config::window_config::WindowConfigBuilder;
use scion::Scion;

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(WindowConfigBuilder::new()
                .with_dimensions((512, 448))
                .with_resizable(false)
                .get())
            .get(),
    )
        .run();
}
