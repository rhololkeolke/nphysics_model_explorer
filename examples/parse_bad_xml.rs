use mjcf_parser;
use mjcf_parser::MJCFModelDesc;
use slog;
use slog::Drain;
use slog::{error, o, warn};
use slog_async;
use slog_term;

const BAD_XML: &str = "<mujoco model=\"Empty Model\"";

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = drain.filter_level(slog::Level::Debug).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());
    mjcf_parser::set_root_logger(logger.clone());

    match MJCFModelDesc::<f32>::parse_xml_string(BAD_XML) {
        Ok(_) => warn!(logger, "Successfully parsed bad XML!"),
        Err(error) => error!(logger, "Failed to parse model file"; "reason" => %error),
    };

    mjcf_parser::drop_root_logger();
}
