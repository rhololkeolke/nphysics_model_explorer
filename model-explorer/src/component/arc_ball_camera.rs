use amethyst::{
    core::nalgebra as na,
    ecs::prelude::{Component, Entity, VecStorage},
};

pub enum ArcBallTarget {
    Point(na::Point3<f32>),
    Entity(Entity),
}

pub struct ArcBallCamera {
    pub target: ArcBallTarget,
    pub distance: f32,
    pub theta: f32,
    pub phi: f32,
}

impl ArcBallCamera {
    pub fn new(target: ArcBallTarget, distance: f32, theta: f32, phi: f32) -> Self {
        Self {
            target,
            distance,
            theta,
            phi,
        }
    }

    pub fn sphere_position(&self) -> na::Vector3<f32> {
        let x = self.distance * self.theta.sin() * self.phi.cos();
        let y = self.distance * self.theta.sin() * self.phi.sin();
        let z = self.distance * self.theta.cos();

        na::Vector3::<f32>::new(x, y, z)
    }
}

impl Default for ArcBallCamera {
    fn default() -> Self {
        ArcBallCamera::new(
            ArcBallTarget::Point(na::Point3::<f32>::origin()),
            50.0,
            0.0,
            0.0,
        )
    }
}

impl Component for ArcBallCamera {
    type Storage = VecStorage<Self>;
}
