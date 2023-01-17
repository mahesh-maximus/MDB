mod init;
mod logger;

use std::sync::LockResult;
pub use log::Level::*;
pub use log::{warn, *};

pub use crate::logger::{LoggerError, LOGGER};

/// Prefix to be used in log lines for functions/modules in Firecracker
/// that are not generally available.
const DEV_PREVIEW_LOG_PREFIX: &str = "[DevPreview]";

fn extract_guard<G>(lock_result: LockResult<G>) -> G {
    match lock_result {
        Ok(guard) => guard,
        // If a thread panics while holding this lock, the writer within should still be usable.
        // (we might get an incomplete log line or something like that).
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// Log a standard warning message indicating a given feature name
/// is in development preview.
pub fn log_dev_preview_warning(feature_name: &str, msg_opt: Option<String>) {
    match msg_opt {
        None => warn!(
            "{} {} is in development preview.",
            DEV_PREVIEW_LOG_PREFIX, feature_name
        ),
        Some(msg) => warn!(
            "{} {} is in development preview - {}",
            DEV_PREVIEW_LOG_PREFIX, feature_name, msg
        ),
    }
}