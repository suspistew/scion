use hecs::Entity;

/// A component creating a parent link to the wrapped entity
#[derive(Debug)]
pub struct Parent(pub Entity);

/// A component creating a link to the wrapped entities
/// This component will be automatically added to an entity by Scion
/// if a component references this entity with a [`Parent`] component
#[derive(Debug)]
pub struct Children(pub Vec<Entity>);
