use std::io::{sink, stderr, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};
use std::{result, thread};

use lazy_static::lazy_static;
use log::{max_level, set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};
use utils::time::LocalTime;

use super::extract_guard;
use crate::init;
use crate::init::Init;
use thiserror::Error;

/// Type for returning functions outcome.
pub type Result<T> = result::Result<T, LoggerError>;

// Values used by the Logger.
const DEFAULT_MAX_LEVEL: LevelFilter = LevelFilter::Trace;

lazy_static! {
    static ref _LOGGER_INNER: Logger = Logger::new();

    /// Static instance used for handling human-readable logs.
    pub static ref LOGGER: &'static Logger = {
        set_logger(_LOGGER_INNER.deref()).expect("Failed to set logger");
        _LOGGER_INNER.deref()
    };
}

/// Logger representing the logging subsystem.
// All member fields have types which are Sync, and exhibit interior mutability, so
// we can call logging operations using a non-mut static global variable.
pub struct Logger {
    init: Init,
    // Human readable logs will be outputted here.
    log_buf: Mutex<Box<dyn Write + Send>>,
    show_level: AtomicBool,
    show_file_path: AtomicBool,
    show_line_numbers: AtomicBool,
    instance_id: RwLock<String>,
}

impl Logger {
    /// Creates a new instance of the current logger.
    fn new() -> Logger {
        Logger {
            init: Init::new(),
            log_buf: Mutex::new(Box::new(sink())),
            show_level: AtomicBool::new(true),
            show_line_numbers: AtomicBool::new(true),
            show_file_path: AtomicBool::new(true),
            instance_id: RwLock::new(String::new()),
        }
    }

    fn show_level(&self) -> bool {
        self.show_level.load(Ordering::Relaxed)
    }

    fn show_file_path(&self) -> bool {
        self.show_file_path.load(Ordering::Relaxed)
    }

    fn show_line_numbers(&self) -> bool {
        self.show_line_numbers.load(Ordering::Relaxed)
    }

    pub fn set_include_level(&self, option: bool) -> &Self {
        self.show_level.store(option, Ordering::Relaxed);
        self
    }

    pub fn set_include_origin(&self, file_path: bool, line_numbers: bool) -> &Self {
        self.show_file_path.store(file_path, Ordering::Relaxed);
        // If the file path is not shown, do not show line numbers either.
        self.show_line_numbers
            .store(file_path && line_numbers, Ordering::Relaxed);
        self
    }

    /// Sets the ID for this logger session.
    pub fn set_instance_id(&self, instance_id: String) -> &Self {
        let mut guard = extract_guard(self.instance_id.write());
        *guard = instance_id;
        self
    }

    pub fn set_max_level(&self, level: LevelFilter) -> &Self {
        set_max_level(level);
        self
    }

    /// Get the current thread's name.
    fn get_thread_name(&self) -> String {
        thread::current().name().unwrap_or("-").to_string()
    }

    /// Creates the first portion (to the left of the separator)
    /// of the log statement based on the logger settings.
    fn create_prefix(&self, record: &Record) -> String {
        let mut prefix: Vec<String> = vec![];

        let instance_id = extract_guard(self.instance_id.read());
        if !instance_id.is_empty() {
            prefix.push(instance_id.to_string());
        }

        // Attach current thread name to prefix.
        prefix.push(self.get_thread_name());

        if self.show_level() {
            prefix.push(record.level().to_string());
        };

        if self.show_file_path() {
            prefix.push(record.file().unwrap_or("unknown").to_string());
        };

        if self.show_line_numbers() {
            if let Some(line) = record.line() {
                prefix.push(line.to_string());
            }
        }

        format!("[{}]", prefix.join(":"))
    }

    /// if the max level hasn't been configured yet, set it to default
    fn try_init_max_level(&self) {
        // if the max level hasn't been configured yet, set it to default
        if max_level() == LevelFilter::Off {
            self.set_max_level(DEFAULT_MAX_LEVEL);
        }
    }

    pub fn configure(&self, instance_id: Option<String>) -> Result<()> {
        self.init
            .call_init(|| {
                if let Some(some_instance_id) = instance_id {
                    self.set_instance_id(some_instance_id);
                }

                self.try_init_max_level();

                // don't finish the initialization
                false
            })
            .map_err(LoggerError::Init)
    }


    pub fn init(&self, header: String, log_dest: Box<dyn Write + Send>) -> Result<()> {
        self.init
            .call_init(|| {
                let mut g = extract_guard(self.log_buf.lock());
                *g = log_dest;

                self.try_init_max_level();

                // finish init
                true
            })
            .map_err(LoggerError::Init)?;

        self.write_log(header, Level::Info);

        Ok(())
    }

    /// Handles the common logic of writing regular log messages.
    ///
    /// Writes `msg` followed by a newline to the destination, flushing afterwards.
    fn write_log(&self, msg: String, msg_level: Level) {
        let mut guard;
        let mut writer: Box<dyn Write> = if self.init.is_initialized() {
            guard = extract_guard(self.log_buf.lock());
            Box::new(guard.as_mut())
        } else {
            match msg_level {
                Level::Error | Level::Warn => Box::new(stderr()),
                _ => Box::new(stdout()),
            }
        };
        // Writes `msg` followed by newline and flushes, if either operation returns an error,
        // increment missed log count.
        // This approach is preferable over `Result::and` as if `write!` returns  an error it then
        // does not attempt to flush.
        if writeln!(writer, "{}", msg)
            .and_then(|_| writer.flush())
            .is_err()
        {
            // No reason to log the error to stderr here, just increment the metric.
            //METRICS.logger.missed_log_count.inc();
        }
    }
}

/// Describes the errors which may occur while handling logging scenarios.
#[derive(Debug, Error)]
pub enum LoggerError {
    /// Initialization Error.
    #[error("Logger initialization failure: {0}")]
    Init(init::Error),
}

/// Implements the "Log" trait from the externally used "log" crate.
impl Log for Logger {
    // This is currently not used.
    fn enabled(&self, _metadata: &Metadata) -> bool {
        unreachable!();
    }

    fn log(&self, record: &Record) {
        let msg = format!(
            "{} {} {}",
            LocalTime::now(),
            self.create_prefix(record),
            record.args()
        );
        self.write_log(msg, record.metadata().level());
    }

    // This is currently not used.
    fn flush(&self) {
        unreachable!();
    }
}