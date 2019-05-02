use super::ConstructWorldState;
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

// TODO(dschwab): Figure out how to do logging

pub struct LoadModelState {
    model_file: PathBuf,
}

impl LoadModelState {
    /// Create a new LoadModelState
    pub fn new(model_file: PathBuf) -> Self {
        Self { model_file }
    }

    /// Parse the specified model creating the construction
    /// descriptors
    fn load_model<N: na::RealField + From<f32> + FromStr>(&self) -> MJCFModelDesc<N>
    where
        // TODO(dschwab): Why is this necessary?
        <N as FromStr>::Err: std::fmt::Display,
    {
        println!("load_model called");

        let model_xml: String =
            fs::read_to_string(&self.model_file).expect("Failed to read model file");

        MJCFModelDesc::parse_xml_string(&model_xml).expect("Failed to parse model file xml")
    }
}

impl SimpleState for LoadModelState {
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("LoadModelState update");
        let model_desc = self.load_model::<f32>();

        Trans::Push(Box::new(ConstructWorldState::new(model_desc)))
    }
}
