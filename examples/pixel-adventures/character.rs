use std::collections::HashMap;
use std::time::Duration;
use scion::core::components::animations::{Animation, AnimationModifier};
use scion::core::components::material::Material;
use scion::core::resources::asset_manager::AssetRef;

pub struct Character{
    pub idle_right_asset_ref: AssetRef<Material>,
    pub idle_left_asset_ref: AssetRef<Material>,
    pub running_right_asset_ref: AssetRef<Material>,
    pub running_left_asset_ref: AssetRef<Material>,
    pub jump_asset_ref: AssetRef<Material>,
}

pub fn get_animations_character() -> HashMap<String, Animation> {
    let mut map: HashMap<String, Animation> = HashMap::new();
    map.insert("idle right".to_string(), Animation::looping(Duration::from_millis(600), vec![AnimationModifier::sprite(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0)]));
    map.insert("idle left".to_string(), Animation::new(Duration::from_millis(600), vec![AnimationModifier::sprite(vec![0, 10,9,8,7,6,5,4,3,2,1,0], 10)]));
    map.insert("run right".to_string(), Animation::new(Duration::from_millis(600), vec![AnimationModifier::sprite(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,11], 0)]));
    map.insert("run left".to_string(), Animation::new(Duration::from_millis(600), vec![AnimationModifier::sprite(vec![11,10,9,8,7,6,5,4,3,2,1,0], 11)]));
    map.insert("jump".to_string(), Animation::new(Duration::from_millis(200), vec![AnimationModifier::sprite(vec![0, 0, 0], 0)]));
    map
}