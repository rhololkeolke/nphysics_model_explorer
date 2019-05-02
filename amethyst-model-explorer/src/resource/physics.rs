use nalgebra as na;
use nphysics3d::world::World;

/// Stores an nphysics3d world
struct PhysicsResource<N>
where
    N: na::RealField,
{
    pub world: World<N>,
}

impl<N> PhysicsResource<N>
where
    N: na::RealField,
{
    /// Wrap an existing nphysics3d world
    fn new(world: World<N>) -> Self {
        PhysicsResource::<N> { world }
    }
}

impl<N> Default for PhysicsResource<N>
where
    N: na::RealField,
{
    /// Create an empty nphysics3d world
    fn default() -> Self {
        PhysicsResource::<N> {
            world: World::new(),
        }
    }
}
