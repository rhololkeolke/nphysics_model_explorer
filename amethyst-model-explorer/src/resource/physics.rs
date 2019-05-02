use nalgebra as na;
use nphysics3d::world::World;

/// Stores an nphysics3d world
pub struct PhysicsWorld<N>
where
    N: na::RealField,
{
    pub world: World<N>,
}

impl<N> PhysicsWorld<N>
where
    N: na::RealField,
{
    /// Wrap an existing nphysics3d world
    pub fn new(world: World<N>) -> Self {
        PhysicsWorld::<N> { world }
    }
}

impl<N> Default for PhysicsWorld<N>
where
    N: na::RealField,
{
    /// Create an empty nphysics3d world
    fn default() -> Self {
        PhysicsWorld::<N> {
            world: World::new(),
        }
    }
}
