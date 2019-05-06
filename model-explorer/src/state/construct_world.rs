use super::run_sim::RunSimState;
use crate::asset;
use crate::component::{self, Collider as ColliderComponent};
use crate::resource;
use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::{nalgebra as ana, Transform},
    prelude::Builder,
    renderer::{
        ActiveCamera, Camera, Material, MaterialDefaults, Mesh, PosNormTex, Shape, Texture,
    },
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use ncollide3d::{
    shape,
    transformation::{self, ToTriMesh},
};
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

        println!("Checking reload state");
        {
            let mut reload_model = data.world.write_resource::<resource::ReloadModel>();
            if let resource::ReloadModel::Reload = *reload_model {
                return Trans::Pop;
            } else {
                *reload_model = resource::ReloadModel::Run;
            }
        }

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

            println!("Create mesh component");
            let shape = collider.shape().as_ref();
            let mut trans = Transform::default();
            trans.set_xyz(
                collider.position().translation.x,
                collider.position().translation.y,
                collider.position().translation.z,
            );
            *trans.rotation_mut() = ana::Unit::new_unchecked(ana::Quaternion::new(
                collider.position().rotation.coords.x,
                collider.position().rotation.coords.y,
                collider.position().rotation.coords.z,
                collider.position().rotation.coords.w,
            ));

            let mesh: Handle<Mesh> = if let Some(_s) = shape.as_shape::<shape::Plane<f32>>() {
                // TODO(dschwab): Create an appropriate plane mesh
                unimplemented!()
            } else if let Some(s) = shape.as_shape::<shape::Ball<f32>>() {
                let scale = 2.0 * s.radius() + collider.margin();
                trans.set_scale(scale, scale, scale);

                self.sphere
                    .as_ref()
                    .expect("sphere mesh is not loaded")
                    .clone()
            } else if let Some(s) = shape.as_shape::<shape::Cuboid<f32>>() {
                trans.set_scale(
                    2.0 * s.half_extents().x + collider.margin(),
                    2.0 * s.half_extents().y + collider.margin(),
                    2.0 * s.half_extents().z + collider.margin(),
                );

                self.cube
                    .as_ref()
                    .expect("cuboid mesh is not loaded")
                    .clone()
            } else if let Some(s) = shape.as_shape::<shape::Capsule<f32>>() {
                let mesh = {
                    let mut mesh = s.to_trimesh((32, 32));
                    mesh.replicate_vertices();
                    mesh.recompute_normals();

                    asset::trimesh::to_mesh_data(&mesh)
                };

                data.world
                    .exec(|loader: AssetLoaderSystemData<'_, Mesh>| loader.load_from_data(mesh, ()))
            } else if let Some(_s) = shape.as_shape::<shape::HeightField<f32>>() {
                unimplemented!()
            } else if let Some(_s) = shape.as_shape::<shape::TriMesh<f32>>() {
                unimplemented!()
            } else if let Some(s) = shape.as_shape::<shape::ConvexHull<f32>>() {
                let mut chull = transformation::convex_hull(s.points());
                chull.replicate_vertices();
                chull.recompute_normals();

                let mesh = asset::trimesh::to_mesh_data(&chull);

                data.world
                    .exec(|loader: AssetLoaderSystemData<'_, Mesh>| loader.load_from_data(mesh, ()))
            } else {
                // TODO(dschwab): Better error handling
                panic!("Unsupported shape type!");
            };

            data.world
                .create_entity()
                .with(ColliderComponent {
                    id: collider.handle(),
                })
                .with(mesh)
                .with(trans)
                .with(material)
                .build();
        }

        // TODO(dschwab): Create lights

        // TODO(dschwab): Create multiple cameras from model desc
        println!("Create camera");
        let mut cam_trans = Transform::default();
        cam_trans.set_z(50.0);
        let fps_cam = component::FPSCamera::default();
        let cam_entity = data
            .world
            .create_entity()
            .with(Camera::standard_3d(800.0, 600.0))
            .with(cam_trans)
            .with(fps_cam)
            .build();
        (*data.world.write_resource::<ActiveCamera>()).entity = Some(cam_entity);

        Trans::Push(Box::new(RunSimState))
    }
}
