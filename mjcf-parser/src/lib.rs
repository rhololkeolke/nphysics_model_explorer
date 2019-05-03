mod attributes;
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
pub mod error;
mod log;
mod mjcf_model;
mod tags;

pub use log::{drop_root_logger, set_root_logger};
pub use mjcf_model::MJCFModelDesc;
