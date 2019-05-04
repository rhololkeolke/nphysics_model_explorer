use crate::component;
use crate::resource;
use crate::resource::mouse_drag;
use amethyst::{
    core::{nalgebra as na, Transform},
    ecs::prelude::{Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::{ActiveCamera, VirtualKeyCode},
};
use sm::{AsEnum, Machine};

const MOUSE_SENSITIVITY: f64 = 0.01f64;
const MOVE_SPEED: f32 = 0.1;

pub struct FPSCamera;

impl<'s> System<'s> for FPSCamera {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'s, ActiveCamera>,
        ReadStorage<'s, component::FPSCamera>,
        Read<'s, resource::mouse_drag::Variant>,
        Read<'s, InputHandler<(), ()>>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (active_camera, fps_cams, mouse_drag, input_handler, mut transforms): Self::SystemData,
    ) {
        if let Some(active_camera_entity) = active_camera.entity.as_ref() {
            if fps_cams.get(*active_camera_entity).is_some() {
                let mouse_delta = match (*mouse_drag).clone() {
                    mouse_drag::Variant::DraggingByClicked(s) => match s.state().as_enum() {
                        mouse_drag::StateVariant::Dragging(s) => {
                            Some(s.delta_end_position - s.delta_start_position)
                        }
                        _ => unreachable!(),
                    },
                    mouse_drag::Variant::FinishDraggingByNotClicked(s) => match s.state().as_enum()
                    {
                        mouse_drag::StateVariant::FinishDragging(s) => {
                            Some(s.end_position - s.delta_start_position)
                        }
                        _ => unreachable!(),
                    },
                    _ => None,
                };

                let transform = transforms
                    .get_mut(*active_camera_entity)
                    .expect("Failed to get active camera transform");
                if let Some(mouse_delta) = mouse_delta {
                    let mouse_delta: na::Vector2<f32> =
                        na::convert(-MOUSE_SENSITIVITY * mouse_delta);

                    transform.pitch_local(mouse_delta.y);
                    transform.yaw_local(mouse_delta.x);
                }

                if input_handler.key_is_down(VirtualKeyCode::W) {
                    transform.move_forward(MOVE_SPEED);
                }

                if input_handler.key_is_down(VirtualKeyCode::S) {
                    transform.move_backward(MOVE_SPEED);
                }

                if input_handler.key_is_down(VirtualKeyCode::A) {
                    transform.move_left(MOVE_SPEED);
                }

                if input_handler.key_is_down(VirtualKeyCode::D) {
                    transform.move_right(MOVE_SPEED);
                }
            }
        }
    }
}
