use std::{collections::HashMap, ops::Range};

use legion::{world::SubWorld, Entity, EntityStore, World};
use wgpu::util::BufferInitDescriptor;

use crate::{
    core::{
        components::{
            animations::{Animation, Animations},
            material::Material,
            maths::transform::Transform,
            tiles::sprite::Sprite,
        },
        resources::asset_manager::AssetRef,
    },
    rendering::scion2d::Renderable2D,
    utils::maths::{Dimensions, Position},
};

pub(crate) struct Tile {
    pub(crate) position: Position,
    pub(crate) tilemap: Entity,
}

/// Struct representing a single tile in a tilemap. Needs to be returned in the
/// tile resolver function when creating a tilemap.
pub struct TileInfos {
    tile_nb: Option<usize>,
    animation: Option<Animation>,
}

impl TileInfos {
    /// Creates a new TileInfos struct
    pub fn new(tile_nb: Option<usize>, animation: Option<Animation>) -> Self {
        Self { tile_nb, animation }
    }
}

#[derive(Default)]
pub struct Tilemap {
    tile_entities: HashMap<Position, Entity>,
}

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

impl Tilemap {
    pub fn create<F>(infos: TilemapInfo, world: &mut World, mut tile_resolver: F) -> Entity
    where
        F: FnMut(&Position) -> TileInfos, {
        let self_entity = Tilemap::create_tilemap(world, infos.tileset_ref, infos.transform);

        for x in 0..infos.dimensions.width() {
            for y in 0..infos.dimensions.height() {
                for layer in 0..infos.dimensions.number_of_layers() {
                    let position = Position::new(x, y, layer);
                    let tile_infos = tile_resolver(&position);

                    let entity = world
                        .push((Tile { position: position.clone(), tilemap: self_entity.clone() },));

                    if let Some(tile_nb) = tile_infos.tile_nb {
                        world.entry(entity).unwrap().add_component(Sprite::new(tile_nb));
                    }

                    if let Some(animation) = tile_infos.animation {
                        world.entry(entity).unwrap().add_component(Animations::single(
                            "TileAnimation",
                            animation,
                        ));
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

    /// Try to modify the sprite's tile at given position
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

    fn create_tilemap(
        world: &mut World,
        tileset_ref: AssetRef<Material>,
        transform: Transform,
    ) -> Entity {
        world.push((Self::default(), tileset_ref, transform))
    }
}

impl Renderable2D for Tilemap {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        todo!()
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor { todo!() }

    fn range(&self) -> Range<u32> { todo!() }

    fn dirty(&self) -> bool { todo!() }

    fn set_dirty(&mut self, _is_dirty: bool) { todo!() }
}
