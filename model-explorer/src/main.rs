use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use nphysics3d::world::World;
#[allow(unused_imports)]
use slog::{debug, error, info, trace, warn};
use slog::{o, Drain};
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use nphysics_testbed3d::Testbed;

fn parse_log_level(level: &str) -> Result<slog::Level, String> {
    match level.trim().to_lowercase().as_str() {
        "trace" => Ok(slog::Level::Trace),
        "debug" => Ok(slog::Level::Debug),
        "info" => Ok(slog::Level::Info),
        "warn" => Ok(slog::Level::Warning),
        "error" => Ok(slog::Level::Error),
        "critical" => Ok(slog::Level::Critical),
        _ => Err(format!(
            "Unknown log level '{}'. Must be one of [trace, debug, info, warn, error, critical]",
            level
        )),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Model Explorer",
    about = "Simulate and iteract with Nphysics model parsed from MJCF XML file"
)]
/// Specify an MJCF XML file to load the model and simulate it.
struct Args {
    #[structopt(parse(from_os_str))]
    /// The path to the MJCF XML file
    model_file: PathBuf,
    #[structopt(
        short = "l",
        long = "level",
        default_value = "info",
        parse(try_from_str = "parse_log_level")
    )]
    /// Log level. Must be one of [trace, debug, info, warn, error, critical].
    log_level: slog::Level,
}

fn make_logger(level: slog::Level, model_file: &Path) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = drain.filter_level(level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(
        drain,
        o!("model_file" => model_file.to_string_lossy().to_string(),
            "place" =>
           slog::FnValue(move |info| {
                       format!("{}:{} {}",
                               info.file(),
                               info.line(),
                               info.module(),
                               )
        })),
    )
}

fn main() {
    let args = Args::from_args();

    let logger = make_logger(args.log_level, &args.model_file);
    mjcf_parser::set_root_logger(logger.clone());

    let model_xml: String = fs::read_to_string(args.model_file).expect("Failed to read model file");

    let mut model_desc =
        MJCFModelDesc::<f32>::parse_xml_string(&model_xml).expect("Failed to parse model file xml");

    // TODO(dschwab): get gravity from model desc
    let mut world = World::new();
    world.set_gravity(na::Vector3::z() * -9.91);

    // build model desc
    model_desc.build(&mut world);

    // create the testbed
    let mut testbed = Testbed::new(world);
    testbed.look_at(
        na::Point3::new(2.0, 2.0, 2.0),
        na::Point3::new(0.0, 0.0, 0.0),
    );
    testbed.run();

    // run this to force a flush of logs
    mjcf_parser::drop_root_logger();
}
