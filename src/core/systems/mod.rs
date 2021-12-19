use crate::core::package::Package;
use crate::legion::systems::ParallelRunnable;
use crate::legion::{World, Resources};
use crate::core::state::GameState;

use animations_system::animation_executer_system;
use asset_ref_resolver_system::{asset_ref_resolver_system, MaterialAssetResolverFn};
use collider_systems::{compute_collisions_system, debug_colliders_system,
};
use default_camera_system::{default_camera_system, camera_dpi_system};
use hide_propagation_system::{hide_propagated_deletion_system, hide_propagation_system};
use hierarchy_system::children_manager_system;
use missing_ui_component_system::missing_ui_component_system;
use parent_transform_system::{dirty_child_system, dirty_transform_system};
use ui_text_system::ui_text_bitmap_update_system;
use crate::ScionBuilder;
use crate::core::components::ui::ui_image::UiImage;
use crate::core::components::ui::ui_text::{UiTextImage, UiText};
use crate::core::components::material::Material;
use crate::core::resources::events::Events;
use crate::core::resources::events::topic::TopicConfiguration;
use crate::core::resources::time::{TimerType, Timers, Time};
use crate::core::resources::asset_manager::AssetManager;
use crate::core::resources::inputs::inputs_controller::InputsController;
use crate::core::scene::SceneController;
use crate::core::resources::audio::Audio;

pub(crate) mod animations_system;
pub(crate) mod asset_ref_resolver_system;
pub(crate) mod collider_systems;
pub(crate) mod default_camera_system;
pub(crate) mod hide_propagation_system;
pub(crate) mod hierarchy_system;
pub(crate) mod missing_ui_component_system;
pub(crate) mod parent_transform_system;
pub(crate) mod ui_text_system;



pub(crate) struct InternalPackage;
impl Package for InternalPackage {
    fn prepare(&self, _world: &mut World, resources: &mut Resources) {

        let mut events = Events::default();
        events
            .create_topic("Inputs", TopicConfiguration::default())
            .expect("Error while creating topic for inputs event");

        let mut timers = Timers::default();

        if cfg!(feature = "hot-reload") {
            let _res = timers.add_timer("hot-reload-timer", TimerType::Cyclic, 5.);
        }

        resources.insert(Time::default());
        resources.insert(events);
        resources.insert(timers);
        resources.insert(AssetManager::default());
        resources.insert(InputsController::default());
        resources.insert(GameState::default());
        resources.insert(SceneController::default());
        resources.insert(Audio::default());
    }

    fn load(self, builder: ScionBuilder) -> ScionBuilder {
        builder.with_system(default_camera_system())
            .with_system(camera_dpi_system())
            .with_system(ui_text_bitmap_update_system())
            .with_system(debug_colliders_system())
            .with_flush()
            .with_system(children_manager_system())
            .with_system(hide_propagated_deletion_system())
            .with_flush()
            .with_system(hide_propagation_system())
            .with_system(missing_ui_component_system::<UiImage>())
            .with_system(missing_ui_component_system::<UiTextImage>())
            .with_system(missing_ui_component_system::<UiText>())
            .with_system(asset_ref_resolver_system::<Material, MaterialAssetResolverFn>())
            .with_system(animation_executer_system())
            .with_flush()
            .with_system(dirty_transform_system())
            .with_system(compute_collisions_system())
            .with_flush()
    }
}