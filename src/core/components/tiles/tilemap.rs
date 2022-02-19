use std::{collections::HashMap, ops::Range};

use legion::{world::SubWorld, Entity, EntityStore, World};
use serde::{Deserialize, Serialize};
use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::core::resources::asset_manager::AssetManager;
use crate::{
    core::{
        components::{
            animations::{Animation, Animations},
            material::Material,
            maths::{hierarchy::Parent, transform::Transform},
            tiles::sprite::Sprite,
        },
        resources::asset_manager::AssetRef,
    },
    rendering::Renderable2D,
    utils::maths::{Dimensions, Position},
};

#[derive(Debug)]
pub struct Pathing {
    pathing_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileEvent {
    event_type: String,
    properties: HashMap<String, String>,
}

impl TileEvent {
    pub fn new(event_type: String, properties: HashMap<String, String>) -> Self {
        if event_type.as_str() == "" {
            panic!("An event must have a type");
        }
        Self { event_type, properties }
    }

    pub fn event_type(&self) -> String {
        self.event_type.to_string()
    }

    pub fn properties(&mut self) -> &mut HashMap<String, String> {
        &mut self.properties
    }
}

pub(crate) struct Tile {
    pub(crate) position: Position,
    pub(crate) tilemap: Entity,
}

/// Struct representing a single tile in a tilemap. Needs to be returned in the
/// tile resolver function when creating a tilemap.
pub struct TileInfos {
    tile_nb: Option<usize>,
    animation: Option<Animation>,
    event: Option<TileEvent>,
    pathing_type: Option<String>,
}

impl TileInfos {
    /// Creates a new TileInfos struct
    pub fn new(tile_nb: Option<usize>, animation: Option<Animation>) -> Self {
        Self { tile_nb, animation, event: None, pathing_type: None }
    }

    /// Adds an event to the current tile.
    pub fn with_event(mut self, event: Option<TileEvent>) -> Self {
        self.event = event;
        self
    }

    /// Force a pathing value for this position. If this exists, the engine won't check in the
    /// tileset atlas to retrive pathing value for this tile
    pub fn with_pathing(mut self, pathing: String) -> Self {
        self.pathing_type = Some(pathing);
        self
    }
}

/// `TilemapInfo` regroups all the needed informations that a Tilemap needs to be created
pub struct TilemapInfo {
    dimensions: Dimensions,
    transform: Transform,
    tileset_ref: AssetRef<Material>,
}

impl TilemapInfo {
    pub fn new(
        dimensions: Dimensions,
        transform: Transform,
        tileset_ref: AssetRef<Material>,
    ) -> Self {
        Self { dimensions, transform, tileset_ref }
    }
}

/// `Tilemap` is `Scion` convenience component to create a full multi layered tilemap.
pub struct Tilemap {
    tile_entities: HashMap<Position, Entity>,
    events: HashMap<Position, TileEvent>,
    tileset_ref: AssetRef<Material>,
}

impl Tilemap {
    pub(crate) fn new(tileset_ref: AssetRef<Material>) -> Self {
        Self { tile_entities: Default::default(), events: HashMap::default(), tileset_ref }
    }

    /// Convenience fn to create a tilemap and add it to the world.
    /// tile_resolver is a function taking a 3D position as parameter and a `TileInfos`
    /// as a return. This way, the tilemap knows exactly what to add at which coordinates.
    pub fn create<F>(infos: TilemapInfo, world: &mut World, mut tile_resolver: F) -> Entity
    where
        F: FnMut(&Position) -> TileInfos,
    {
        let self_entity = Tilemap::create_tilemap(world, infos.tileset_ref, infos.transform);

        for x in 0..infos.dimensions.width() {
            for y in 0..infos.dimensions.height() {
                for z in 0..infos.dimensions.depth() {
                    let position = Position::new(x, y, z);
                    let tile_infos = tile_resolver(&position);

                    let entity = world.push((
                        Tile { position: position.clone(), tilemap: self_entity.clone() },
                        Parent(self_entity.clone()),
                    ));

                    if let Some(tile_nb) = tile_infos.tile_nb {
                        world.entry(entity).unwrap().add_component(Sprite::new(tile_nb));
                    }

                    if let Some(animation) = tile_infos.animation {
                        world
                            .entry(entity)
                            .unwrap()
                            .add_component(Animations::single("TileAnimation", animation));
                    }

                    if let Some(pathing) = tile_infos.pathing_type {
                        world
                            .entry(entity)
                            .unwrap()
                            .add_component(Pathing { pathing_type: pathing });
                    }

                    if let Some(event) = tile_infos.event {
                        world
                            .entry(self_entity)
                            .unwrap()
                            .get_component_mut::<Tilemap>()
                            .unwrap()
                            .events
                            .insert(position.clone(), event);
                    }

                    world
                        .entry(self_entity)
                        .unwrap()
                        .get_component_mut::<Tilemap>()
                        .unwrap()
                        .tile_entities
                        .insert(position, entity);
                }
            }
        }

        self_entity
    }

    /// Try to modify the sprite's tile at a given position
    pub fn modify_sprite_tile(
        &self,
        tile_position: Position,
        new_tile_nb: usize,
        world: &mut SubWorld,
    ) {
        if self.tile_entities.contains_key(&tile_position) {
            let mut entity = world
                .entry_mut(*self.tile_entities.get(&tile_position).unwrap())
                .expect("Unreachable registered entity in tilemap");
            if let Ok(sprite) = entity.get_component_mut::<Sprite>() {
                sprite.set_tile_nb(new_tile_nb);
            }
        }
    }

    pub fn retrieve_sprite_tile(
        &self,
        tile_position: &Position,
        world: &SubWorld,
    ) -> Option<usize> {
        if self.tile_entities.contains_key(&tile_position) {
            let entity = self.tile_entities.get(&tile_position).unwrap();
            if let Ok(entry) = world.entry_ref(*entity) {
                let sprite = entry.get_component::<Sprite>();
                if let Ok(sprite) = sprite {
                    return Some(sprite.get_tile_nb());
                }
            }
        }
        None
    }

    /// Retrieves the pathing value associated with this position in the tilemap
    pub fn retrieve_pathing(
        &self,
        tile_position: &Position,
        world: &SubWorld,
        asset_manager: &AssetManager,
    ) -> Option<String> {
        if self.tile_entities.contains_key(&tile_position) {
            let entity = self.tile_entities.get(&tile_position).unwrap();
            if let Ok(entry) = world.entry_ref(*entity) {
                let pathing = entry.get_component::<Pathing>();
                if let Ok(path_value) = pathing {
                    return Some(path_value.pathing_type.to_string());
                }
            }
        }
        if let Some(tileset) = asset_manager.retrieve_tileset(&self.tileset_ref) {
            if let Some(sprite) = self.retrieve_sprite_tile(tile_position, world) {
                let val = tileset.pathing.iter().filter(|(_k, v)| v.contains(&sprite)).next();
                if let Some(entry) = val {
                    return Some(entry.0.to_string());
                }
            }
        }
        None
    }

    /// Retrieves the mutable tile event associated with this position in the tilemap
    pub fn retrieve_event(&mut self, tile_position: &Position) -> Option<&mut TileEvent> {
        return self.events.get_mut(&tile_position);
    }

    fn create_tilemap(
        world: &mut World,
        tileset_ref: AssetRef<Material>,
        transform: Transform,
    ) -> Entity {
        world.push((Self::new(tileset_ref.clone()), tileset_ref, transform))
    }
}

impl Renderable2D for Tilemap {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        todo!()
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        todo!()
    }

    fn range(&self) -> Range<u32> {
        todo!()
    }

    fn topology() -> PrimitiveTopology {
        wgpu::PrimitiveTopology::TriangleList
    }

    fn dirty(&self) -> bool {
        todo!()
    }

    fn set_dirty(&mut self, _is_dirty: bool) {
        todo!()
    }
}
