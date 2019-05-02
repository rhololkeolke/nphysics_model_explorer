use super::run_sim::RunSimState;
use crate::component::Collider as ColliderComponent;
use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::Transform,
    prelude::Builder,
    renderer::{Camera, Material, MaterialDefaults, Mesh, PosNormTex, Shape, Texture},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use ncollide3d::shape;
use nphysics3d::world::World;
use nphysics_user_data::ColliderUserData;

pub struct ConstructWorldState<N>
where
    N: na::RealField,
{
    model_desc: MJCFModelDesc<N>,

    // basic shape mesh
    sphere: Option<Handle<Mesh>>,
    cube: Option<Handle<Mesh>>,

    // basic materials
    default_albedo: Option<Handle<Texture>>,
}

impl<'a, N: na::RealField> ConstructWorldState<N> {
    pub fn new(model_desc: MJCFModelDesc<N>) -> Self {
        Self {
            model_desc,
            sphere: None,
            cube: None,
            default_albedo: None,
        }
    }
}

impl SimpleState for ConstructWorldState<f32> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // FIXME(dschwab): Move all of the standard mesh creation into
        // a resource that is created once in the entire application
        // lifecycle. Current implementation will duplicate the same
        // mesh assets when model is reloaded.

        if self.sphere.is_none() {
            self.sphere = Some(data.world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
                loader.load_from_data(Shape::Sphere(32, 32).generate::<Vec<PosNormTex>>(None), ())
            }))
        }

        if self.cube.is_none() {
            self.cube = Some(data.world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
                loader.load_from_data(
                    Shape::Cube.generate::<Vec<PosNormTex>>(Some((1.0, 1.0, 1.0))),
                    (),
                )
            }))
        }

        if self.default_albedo.is_none() {
            self.default_albedo = Some(data.world.exec(
                |loader: AssetLoaderSystemData<'_, Texture>| {
                    loader.load_from_data([0.5, 0.5, 0.5, 1.0].into(), ())
                },
            ))
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("ConstructWorldState update");
        let mat_defaults = data.world.read_resource::<MaterialDefaults>().0.clone();

        println!("Creating nphysics world");
        // TODO(dschwab): get gravity from model desc
        let mut world = World::<f32>::new();
        world.set_gravity(na::Vector3::z() * -9.81);

        self.model_desc.build(&mut world);

        println!("Constructing collider entities");
        for collider in world.colliders() {
            println!("Create material component");
            let material = if let Some(rgba) = collider
                .user_data()
                .and_then(|x| x.downcast_ref::<ColliderUserData<f32>>())
                .and_then(|x| x.rgba)
            {
                println!("Creating material");
                let albedo = data
                    .world
                    .exec(|loader: AssetLoaderSystemData<'_, Texture>| {
                        loader.load_from_data([rgba.x, rgba.y, rgba.z, rgba.w].into(), ())
                    });
                Material {
                    albedo,
                    ..mat_defaults.clone()
                }
            } else {
                println!("Using default material");
                Material {
                    albedo: self
                        .default_albedo
                        .as_ref()
                        .expect("default albedo not loaded")
                        .clone(),
                    ..mat_defaults.clone()
                }
            };

            let mut entity = data
                .world
                .create_entity()
                .with(ColliderComponent {
                    id: collider.handle(),
                })
                .with(material);

            println!("Create mesh component");
            let shape = collider.shape().as_ref();
            if let Some(_s) = shape.as_shape::<shape::Plane<f32>>() {
                // TODO(dschwab): Create an appropriate plane mesh

            } else if let Some(s) = shape.as_shape::<shape::Ball<f32>>() {
                println!("Creating sphere collider entity");

                let mut trans = Transform::default();
                trans.set_xyz(
                    collider.position().translation.x,
                    collider.position().translation.y,
                    collider.position().translation.z,
                );
                let scale = 2.0 * s.radius() + collider.margin();
                trans.set_scale(scale, scale, scale);

                entity = entity
                    .with(
                        // TODO(dschwab): How do I deal with errors?
                        self.sphere
                            .as_ref()
                            .expect("sphere mesh not loaded")
                            .clone(),
                    )
                    .with(trans);
            } else if let Some(_s) = shape.as_shape::<shape::Cuboid<f32>>() {
                // TODO(dschwab): Create an appropriate cube mesh
            }
            // TODO(dschwab): handle other shape cases

            entity.build();
        }

        // TODO(dschwab): Create lights

        // TODO(dschwab): Create multiple cameras from model desc
        println!("Create camera");
        let mut cam_trans = Transform::default();
        cam_trans.set_z(50.0);
        data.world
            .create_entity()
            .with(Camera::standard_3d(800.0, 600.0))
            .with(cam_trans)
            .build();

        Trans::Push(Box::new(RunSimState))
    }
}
