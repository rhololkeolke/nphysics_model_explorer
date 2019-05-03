use crate::component;
use crate::resource;
use crate::resource::mouse_drag;
use amethyst::{
    core::{nalgebra as na, Transform},
    ecs::prelude::{Read, System, WriteStorage},
    renderer::ActiveCamera,
};
use sm::{AsEnum, Machine};

const MOUSE_SENSITIVITY: f64 = 0.001f64;

pub struct ArcBallCameraSystem;

impl<'s> System<'s> for ArcBallCameraSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        WriteStorage<'s, component::ArcBallCamera>,
        WriteStorage<'s, Transform>,
        Read<'s, resource::mouse_drag::Variant>,
    );

    fn run(
        &mut self,
        (active_camera, mut arc_ball_cams, mut transforms, mouse_drag): Self::SystemData,
    ) {
        if let Some(active_camera_entity) = active_camera.entity.as_ref() {
            if let Some(arc_ball_cam) = arc_ball_cams.get_mut(*active_camera_entity) {
                let mut dirty_transform = false;

                let delta = match (*mouse_drag).clone() {
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

                if let Some(delta) = delta {
                    dirty_transform = true;
                    let delta: na::Vector2<f32> = na::convert(-MOUSE_SENSITIVITY * delta);

                    arc_ball_cam.theta += delta.x;
                    arc_ball_cam.phi = (arc_ball_cam.phi + delta.y)
                        .min(std::f32::consts::PI)
                        .max(-std::f32::consts::PI);
                }

                if dirty_transform {
                    // set the new camera position
                    let sphere_translation: na::Vector3<f32> = match arc_ball_cam.target {
                        component::ArcBallTarget::Point(p) => p - na::Point3::<f32>::origin(),
                        component::ArcBallTarget::Entity(e) => *transforms
                            .get(e)
                            .expect("Getting target entity transform")
                            .translation(),
                    };

                    let sphere_pos = arc_ball_cam.sphere_position();

                    let transform = transforms
                        .get_mut(*active_camera_entity)
                        .expect("Getting active camera transform");
                    *transform.isometry_mut() = na::Isometry3::<f32>::identity();
                    transform.pitch_local(-arc_ball_cam.phi);
                    *transform.translation_mut() = sphere_pos + sphere_translation;

                    // transform.pitch_global(arc_ball_cam.phi);
                    // transform.yaw_global(arc_ball_cam.theta);
                    // transform.set_xyz(sphere_pos.x, sphere_pos.y, sphere_pos.z);
                    // transform.translate_xyz(
                    //     sphere_translation.x,
                    //     sphere_translation.y,
                    //     sphere_translation.z,
                    // );
                    // *transform.translation_mut() = sphere_pos; // + sphere_translation;
                    // transform.face_towards(-sphere_pos, na::Vector3::y());
                }
            }
        }
    }
}
