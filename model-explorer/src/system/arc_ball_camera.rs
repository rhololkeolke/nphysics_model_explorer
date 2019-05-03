use amethyst::{
    core::Transform,
    ecs::prelude::{Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::ActiveCamera,
};
use nalgebra as na;

pub struct ArcBallCameraSystem;

impl<'s> System<'s> for ArcBallCameraSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler>,
    );

    fn run(&mut self, (active_camera, mut transforms, input_handler): Self::SystemData) {
        let mut transform = transforms.get_mut(active_camera.entity).unwrap();

        
    }
}
