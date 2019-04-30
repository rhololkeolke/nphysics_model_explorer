use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() {
    let args = Args::from_args();

    println!("args: {:#?}", args);
}
