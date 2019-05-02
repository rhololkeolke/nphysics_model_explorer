use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use super::run_sim::RunSimState;

pub struct ConstructWorldState<N>
where
    N: na::RealField,
{
    model_desc: MJCFModelDesc<N>,
}

impl<'a, N: na::RealField> ConstructWorldState<N> {
    pub fn new(model_desc: MJCFModelDesc<N>) -> Self {
        Self { model_desc }
    }
}

impl<N: na::RealField> SimpleState for ConstructWorldState<N> {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("ConstructWorldState update");
        // TODO(dschwab): Create the meshes

        // TODO(dschwab): Create the materials

        // TODO(dschwab): Create the collider components

        // TODO(dschwab): Create the collider entities

        Trans::Push(Box::new(RunSimState))
    }
}
