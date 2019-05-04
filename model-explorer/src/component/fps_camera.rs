use amethyst::ecs::prelude::{Component, NullStorage};

pub struct FPSCamera;

impl Default for FPSCamera {
    fn default() -> Self {
        Self {}
    }
}

impl Component for FPSCamera {
    type Storage = NullStorage<Self>;
}
