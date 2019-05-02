use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use super::run_sim::RunSimState;

pub struct ConstructWorldState<'a, N>
where
    N: na::RealField,
{
    model_desc: MJCFModelDesc<'a, N>,
}

impl<'a, N: na::RealField> ConstructWorldState<'a, N> {
    pub fn new(model_desc: MJCFModelDesc<'a, N>) -> Self {
        Self { model_desc }
    }
}

impl<'a, N: na::RealField> SimpleState for ConstructWorldState<'a, N> {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("ConstructWorldState update");
        // TODO(dschwab): Create the meshes

        // TODO(dschwab): Create the materials

        // TODO(dschwab): Create the collider components

        // TODO(dschwab): Create the collider entities

        Trans::Push(Box::new(RunSimState))
    }
}
