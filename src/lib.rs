/// Defined for log testing.
pub const TEST_LOG: &'static str = "Testing Log";
/// Testing Logging ID.
pub const TEST_LOG_ID: usize = 0000;

/// Defined for testing the warning system.
pub const TEST_WARN: &'static str = "Testing Warning";
/// Testing Warning System ID.
pub const TEST_WARN_ID: usize = 0001;

/// Defined for testing error handling.
pub const TEST_ERROR: &'static str = "Testing Error";
/// Testing Error Handling ID.
pub const TEST_ERROR_ID: usize = 0002;

/// A LoggingType Enum for defining the types of logs.
#[derive(Default, Clone, Debug, PartialEq)]
pub enum LoggingType {
    /// Error - Used for UNRECOVERABLE Errors, Panicing once the log parsing ends.
    Error,
    /// Warn - Used for potentially hazardous behavior.
    Warn,
    /// Log - Used for simple information printing.
    Log,
    /// Marker - Used for Marking and event
    /// Has no ID, and prints out "Marker Log"
    #[default]
    Marker,
}

/// A Logger Struct for containing logging values.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Logger {
    /// Contains the Logging Message.
    log: String,
    /// Contains the Logging ID.
    log_id: usize,
    /// Contains the Logging Type.
    log_type: LoggingType,
}

impl Logger {
    /// Creates a new Logger Vector.
    pub fn new_logger() -> Vec<Self> {
        vec![Logger {
            log: "Logger Initalized".to_string(),
            log_id: 0,
            log_type: LoggingType::Log,
        }]
    }

    /// Adds a Marker value to the Logger
    pub fn add_marker(collector: &mut Vec<Self>) {
        collector.push(Logger {
            log: "Marker Log".to_string(),
            ..Default::default()
        })
    }

    /// Adds a Log value to the Logger.
    pub fn add_log(log: &str, log_id: usize, collector: &mut Vec<Self>) {
        collector.push(Logger {
            log: log.to_string(),
            log_id,
            log_type: LoggingType::Log,
        })
    }

    /// Adds a Warning value to the Logger.
    pub fn add_warning(log: &str, log_id: usize, collector: &mut Vec<Self>) {
        collector.push(Logger {
            log: log.to_string(),
            log_id,
            log_type: LoggingType::Warn,
        });
    }

    /// Adds an Error value to the Logger.
    /// WARNING! This is designed for UNRECOVERABLE Errors.
    /// Currently, recoverable errors are not supported.
    pub fn add_error(log: &str, log_id: usize, collector: &mut Vec<Self>) {
        collector.push(Logger {
            log: log.to_string(),
            log_id,
            log_type: LoggingType::Error,
        })
    }

    /// Parses the Logger for Errors, Warnings, and Logs.
    /// Prints them out to the terminal as their defined types.
    pub fn parse_logger(collector: &Vec<Logger>) {
        let mut last_error: Option<&Logger> = None;

        for items in collector {
            match items.log_type {
                LoggingType::Marker => {
                    println!("[MARKER]: {}", items.log)
                }
                LoggingType::Log => {
                    println!("[LOG]: Info: {}; Info ID: {}", items.log, items.log_id)
                }
                LoggingType::Warn => {
                    eprintln!(
                        "[WARNING]: Warning: {}; Warning ID: {}",
                        items.log, items.log_id
                    )
                }
                LoggingType::Error => {
                    eprintln!("[ERROR]: Error: {}; Error ID: {}", items.log, items.log_id);
                    last_error = Some(items)
                }
            }
        }

        if let Some(err) = last_error {
            panic!(
                "[ERROR]: Final Error: Error: {}; Error ID: {}",
                err.log, err.log_id
            )
        }
    }
}

#[cfg(test)]
mod mini_log_tests {
    use super::*;

    #[test]
    fn marking_test() {
        let mut logger = Logger::new_logger();

        Logger::add_marker(&mut logger);
        Logger::add_marker(&mut logger);

        Logger::parse_logger(&logger);
    }

    #[test]
    fn logging_test() {
        let mut logger = Logger::new_logger();

        Logger::add_log(TEST_LOG, TEST_LOG_ID, &mut logger);
        Logger::add_log(TEST_LOG, TEST_LOG_ID, &mut logger);

        Logger::parse_logger(&logger);
    }

    #[test]
    fn warning_test() {
        let mut logger = Logger::new_logger();

        Logger::add_warning(TEST_WARN, TEST_WARN_ID, &mut logger);
        Logger::add_warning(TEST_WARN, TEST_WARN_ID, &mut logger);

        Logger::parse_logger(&logger);
    }

    #[test]
    #[should_panic]
    fn error_test() {
        let mut logger = Logger::new_logger();

        Logger::add_error(TEST_ERROR, TEST_ERROR_ID, &mut logger);
        Logger::add_error(TEST_ERROR, TEST_ERROR_ID, &mut logger);

        Logger::parse_logger(&logger);
    }

    #[test]
    #[should_panic]
    fn full_test() {
        let mut logger = Logger::new_logger();

        Logger::add_error(TEST_ERROR, TEST_ERROR_ID, &mut logger);
        Logger::add_warning(TEST_WARN, TEST_WARN_ID, &mut logger);
        Logger::add_log(TEST_LOG, TEST_LOG_ID, &mut logger);

        Logger::parse_logger(&logger);
    }

    use std::sync::Arc;
    use std::thread;

    #[test]
    fn multi_threading_test() {
        let mut logger = Arc::new(Logger::new_logger());

        let mut logger_2nd_thread = Arc::clone(&logger);
        thread::spawn(move || {
            Logger::parse_logger(&mut logger_2nd_thread);
        });
    }

    use std::sync::mpsc;
    use std::sync::Mutex;

    #[test]
    fn multi_threading_transfer_test() {
    

        let logger = Arc::new(Mutex::new(Logger::new_logger()));

        let n = 3;
        let (tx, rx) = mpsc::channel();
        let data = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..n {
            let logger_cloned = Arc::clone(&logger);
            let data_cloned = Arc::clone(&data);
            let tx_cloned = tx.clone();

            let handle = thread::spawn(move || {
                {
                    let mut logger_guard = logger_cloned.lock().unwrap();
                    Logger::add_log(TEST_LOG, TEST_LOG_ID, &mut logger_guard);
                }
                let mut data_in_thread = data_cloned.lock().unwrap();
                *data_in_thread += 1;

                if *data_in_thread == n {
                    let logger_final = logger_cloned.lock().unwrap();
                    tx_cloned.send(logger_final.clone()).unwrap();
                }
            });
            handles.push(handle);
        }

        rx.recv().unwrap();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn other_multi_threading_transfer_test() {
        let logger = Arc::new(Mutex::new(Logger::new_logger()));

        let logger_cloned = Arc::clone(&logger);
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut logger_guard = logger_cloned.lock().unwrap();
            Logger::add_log("Log1", 0, &mut logger_guard);
            tx.send(logger_guard.clone()).unwrap();
        });

        rx.recv().unwrap();

        handle.join().unwrap();
    }
}
