use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};
use amethyst_model_explorer::state::LoadModelState;
use amethyst_model_explorer::system::physics::PhysicsSystem;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Model Explorer",
    about = "Simulate and interact with Nphysics model parsed from MJCF XML file"
)]
/// Specify an MJCF XML file, load the model, and simulate it
struct Args {
    #[structopt(parse(from_os_str))]
    model_file: PathBuf,
}

fn main() -> amethyst::Result<()> {
    let args = Args::from_args();

    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with(PhysicsSystem::<f32>::default(), "physics_system", &[]);

    let mut game = Application::new("./", LoadModelState::new(args.model_file), game_data)?;

    game.run();

    Ok(())
}
