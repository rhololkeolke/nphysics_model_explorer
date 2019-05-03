use mjcf_parser;
use mjcf_parser::MJCFModelDesc;
use slog;
use slog::Drain;
use slog::{info, o};
use slog_async;
use slog_term;

const EMPTY_MODEL: &str = "<mujoco model=\"Empty Model\"></mujoco>";

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = drain.filter_level(slog::Level::Debug).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());
    mjcf_parser::set_root_logger(logger.clone());

    let model = MJCFModelDesc::<f32>::parse_xml_string(EMPTY_MODEL).unwrap();

    info!(logger, "Model name: {}", model.model_name);

    mjcf_parser::drop_root_logger();
}
