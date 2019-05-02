use crate::component;
use crate::resource::physics::PhysicsWorld;
use amethyst::{
    core::Transform,
    ecs::prelude::{Read, ReadStorage, System, WriteStorage},
};
use nalgebra as na;
use std::marker::PhantomData;

pub struct PhysicsSystem<N>
where
    N: na::RealField,
{
    _phantom: PhantomData<N>,
}

impl<'s, N> System<'s> for PhysicsSystem<N>
where
    N: na::RealField,
{
    type SystemData = (
        Read<'s, PhysicsWorld<N>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, component::Collider>,
    );

    fn run(&mut self, (physics, mut transforms, colliders): Self::SystemData) {
        // TODO(dschwab): Implement me
    }
}
