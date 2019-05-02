use amethyst::ecs::prelude::{Component, DenseVecStorage};
use nphysics3d::object::ColliderHandle;

/// Component that stores the physics collider handle of the entity
struct Collider {
    id: ColliderHandle,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
