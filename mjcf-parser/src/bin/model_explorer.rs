use clap::{App, Arg};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use nphysics3d::object::ColliderHandle;
use nphysics3d::world::World;
use nphysics_testbed3d::Testbed;
use nphysics_user_data::ColliderUserData;
use slog;
use slog::Drain;
use slog::{o, trace};
use slog_async;
use slog_term;
use std::fs;

// debug
use ncollide3d::shape::{Ball, ShapeHandle};
use nphysics3d::object::{ColliderDesc, RigidBodyDesc};

fn parse_level(level: &str) -> slog::Level {
    match level.trim().to_lowercase().as_str() {
        "trace" => slog::Level::Trace,
        "debug" => slog::Level::Debug,
        "info" => slog::Level::Info,
        "warn" => slog::Level::Warning,
        "error" => slog::Level::Error,
        "critical" => slog::Level::Critical,
        _ => panic!(
            "Unknown log level {}. Must be one of [trace, debug, info, warn, error, critical]",
            level
        ),
    }
}

fn make_logger(level: slog::Level, model_file: String) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = drain.filter_level(level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(
        drain,
        o!("model_file" => model_file,
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

pub fn set_collider_colors<N: na::RealField>(
    logger: &slog::Logger,
    testbed: &mut nphysics_testbed3d::Testbed,
) {
    trace!(logger, "Setting body colors");
    let mut collider_colors: Vec<(ColliderHandle, na::Point3<f32>)> = vec![];
    for collider in testbed.world().get().colliders() {
        trace!(logger, "Collider \"{}\"", collider.name(); "has_user_data" => collider.user_data().is_some());
        if let Some(user_data) = collider.user_data() {
            let geom_user_data = user_data.downcast_ref::<ColliderUserData<f32>>();
            trace!(logger, "Collider \"{}\"", collider.name();
                   "has_user_data" => collider.user_data().is_some(),
                   "has_geom_user_data" => geom_user_data.is_some());
        }
        if let Some(rgba) = collider
            .user_data()
            .and_then(|x| x.downcast_ref::<ColliderUserData<N>>())
            .and_then(|x| x.rgba)
        {
            let rgb = na::Point3::new(rgba.x, rgba.y, rgba.z);
            trace!(logger, "setting collider \"{}\" color", collider.name(); "rgb" => %rgb);
            collider_colors.push((collider.handle(), rgb));
        }
    }

    for (collider, rgb) in collider_colors {
        testbed.set_collider_color(collider, rgb);
    }
}

fn main() {
    let matches = App::new("Model Explorer")
        .version("0.1")
        .author("Devin Schwab <dschwab@andrew.cmu.edu>")
        .about("Interactively simulate an MJCF model using nphysics")
        .arg(
            Arg::with_name("MODEL_FILE")
                .help("Model file to load and simulate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("log_level")
                .short("l")
                .long("log-level")
                .value_name("LEVEL")
                .help("Set the logging level")
                .takes_value(true),
        )
        .get_matches();

    let logger = make_logger(
        parse_level(matches.value_of("log_level").unwrap_or("info")),
        matches.value_of("MODEL_FILE").unwrap().to_string(),
    );
    mjcf_parser::set_root_logger(logger.clone());

    let model_xml = fs::read_to_string(matches.value_of("MODEL_FILE").unwrap())
        .expect("Failed to read model file");

    let mut model_desc =
        MJCFModelDesc::parse_xml_string(&model_xml).expect("Failed to parse model file xml");

    // TODO(dschwab): get the gravity from the model desc
    let mut world = World::new();
    world.set_gravity(na::Vector3::z() * -9.81);

    // build the model desc
    model_desc.build(&mut world);

    // debug add a gravity sphere
    let sphere = ShapeHandle::new(Ball::new(0.3));
    let mut collider_desc = ColliderDesc::new(sphere)
        .density(1.0)
        .name(String::from("test"));
    collider_desc.set_user_data(Some(ColliderUserData::<f32>::default()));
    let mut rb_desc = RigidBodyDesc::new()
        .collider(&collider_desc)
        .translation(na::Vector3::new(0.0, 0.0, 0.0));
    rb_desc
        .set_translation(na::Vector3::new(0.0, 0.0, 4.0))
        .build(&mut world);
    rb_desc
        .set_translation(na::Vector3::new(0.0, 0.0, 2.0))
        .build(&mut world);

    // create the testbed
    let mut testbed = Testbed::new(world);
    testbed.look_at(
        na::Point3::new(2.0, 2.0, 2.0),
        na::Point3::new(0.0, 0.0, 0.0),
    );
    set_collider_colors::<f32>(&logger, &mut testbed);
    testbed.run();

    mjcf_parser::drop_root_logger();
}
