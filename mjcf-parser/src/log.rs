use crate::built_info;
use lazy_static::lazy_static;
use slog;
use slog::o;
use slog::Drain;
use slog_stdlog;
use std::sync::{Arc, RwLock};

lazy_static! {
    pub static ref LOG: RwLock<Arc<slog::Logger>> = RwLock::new(Arc::new(create_root_logger(None)));
}

fn create_root_logger<L: Into<Option<slog::Logger>>>(logger: L) -> slog::Logger {
    let values = o!("mjcf-parser/version" => built_info::PKG_VERSION,
                    "mjcf-parser/commit" => match built_info::GIT_VERSION {
                        Some(commit) => commit,
                        None => "Unknown",
                    },
                    "mjcf-parser/profile" => built_info::PROFILE);
    match logger.into() {
        Some(logger) => slog::Logger::root(logger, values),
        None => slog::Logger::root(slog_stdlog::StdLog.fuse(), values),
    }
}

pub fn set_root_logger<L: Into<slog::Logger>>(logger: L) {
    let mut log = LOG.write().unwrap();
    *log = Arc::new(create_root_logger(Some(logger.into())));
}

pub fn drop_root_logger() {
    let mut log = LOG.write().unwrap();
    *log = Arc::new(create_root_logger(None));
}
