use super::ConstructWorldState;
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};
use mjcf_parser::MJCFModelDesc;
use std::path::PathBuf;

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
    fn load_model(&self) {
        println!("load_model called");
        // TODO(dschwab): Implement me!

        // TODO(dschwab): How to get command line args for the model
        // file?
    }
}

impl SimpleState for LoadModelState {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("LoadModelState update");
        // TODO(dschwab): Call load_model

        // FIXME(dschwab): Remove me once load_model implemented
        let empty_model = "<mujoco model=\"Empty World\"><worldbody/></mujoco>";
        let model_desc = MJCFModelDesc::<f32>::parse_xml_string(empty_model)
            .expect("Failed to parse empty model string");

        Trans::Push(Box::new(ConstructWorldState::new(model_desc)))
    }
}
