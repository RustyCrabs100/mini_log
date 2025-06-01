//! mini_log - A lightweight logger for all your needs, all at once! 
//! 
//! mini_log is used in situations where a fully fledged logger would be overkill or unneccesary. 
//! If you need a lightweight logger, this is the best choice for you!
//! 
//! To initalize, create a mutable variable equal to the return value of new_logger()
//! To add a value to the variable, call one of the following:
//!     add_marker(Option<&str>, Option<usize>),
//!     add_log(&str, usize),
//!     add_warning(&str, usize),
//!     or add_error(&str, usize).
//! 
//! We currently do not support no_std enviornments, but it is a big priority for future updates.
//! We do support multi-threading, but async has not yet been tested.

/// Used when a Marker is created with no info.
pub const INIT_MARK: &'static str = "Logging Enabled";
/// Used when a Marker is created with no ID.
pub const INIT_MARK_ID: usize = 0;

/// Used for testing the logging system.
pub const TEST_LOG: &'static str = "Testing Log";
/// Used for testing the logging ID system.
pub const TEST_LOG_ID: usize = 1;

/// Used for testing the warning system.
pub const TEST_WARN: &'static str = "Testing Warning";
/// Used for testing the warning ID system.
pub const TEST_WARN_ID: usize = 2;

/// Used for testing the error system.
pub const TEST_ERROR: &'static str = "Testing Error";
/// Used for testing the error ID system.
pub const TEST_ERROR_ID: usize = 3;

/// An enum providing types for logging
#[derive(Default, Clone, Debug, PartialEq)]
pub enum LoggingType {
    /// Error - Used for UNRECOVERABLE Errors. Panics when it's finished parsing.
    Error,
    /// Warning - Used for potentially hazardous behavior logging.
    Warning,
    /// Log - Used for basic information printing
    Log,
    /// Marker - Used for when you want to mark a point in your program. 
    /// Declared the default value.
    #[default]
    Marker,
}

/// A struct containing logging info.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Logger {
    /// log - A vector of strings for containing logging info.
    log: Vec<String>,
    /// log_id - A vector of usizes for containing logging ID's.
    log_id: Vec<usize>,
    /// log_type - A vector of LoggingType's for containing log types.
    log_type: Vec<LoggingType>,
}

impl Logger {
    /// Creates a new logger
    pub fn new_logger() -> Self {
        Self {
            log: vec![INIT_MARK.to_string()],
            log_id: vec![INIT_MARK_ID],
            log_type: vec![LoggingType::Marker],
        }
    }

    /// Adds a new Marker to your logger
    /// Can be called with values equal to None
    pub fn add_marker(&mut self, log: Option<&str>, log_id: Option<usize>) {
        let log_str = log.unwrap_or(INIT_MARK);
        let log_id_val = log_id.unwrap_or(INIT_MARK_ID);
        self.log.push(log_str.to_string());
        self.log_id.push(log_id_val);
        self.log_type.push(LoggingType::Marker);
    }

    /// Adds a new Log to your logger
    pub fn add_log(&mut self, log: &str, log_id: usize) {
        self.log.push(log.to_string());
        self.log_id.push(log_id);
        self.log_type.push(LoggingType::Log);
    }

    /// Adds a new Warning to your logger
    pub fn add_warning(&mut self, log: &str, log_id: usize) {
        self.log.push(log.to_string());
        self.log_id.push(log_id);
        self.log_type.push(LoggingType::Warning);
    }

    /// Adds a new Error to your logger
    pub fn add_error(&mut self, log: &str, log_id: usize) {
        self.log.push(log.to_string());
        self.log_id.push(log_id);
        self.log_type.push(LoggingType::Error);
    }

    /// Parses the Logger
    /// Behavior with the following:
    /// A Marker - Prints out the Marker Info.
    /// A Log - Prints out the Log Info and Log ID.
    /// A Warning - Error Prints the Warning Info and Warning ID.
    /// An Error - Error Prints the Error Info and Error ID, then panics.
    pub fn parse_logger(&self) {
        let mut last_error: Option<usize> = None;

        for i in 0..self.log.len() {
            match self.log_type[i] {
                LoggingType::Marker => {
                    println!("[MARKER]: {}", self.log[i])
                }
                LoggingType::Log => {
                    println!("[LOG]: Info: {}; Info ID: {}", self.log[i], self.log_id[i])
                }
                LoggingType::Warning => {
                    eprintln!("[WARNING]: Warning: {}; Warning ID: {}", self.log[i], self.log_id[i])
                }
                LoggingType::Error => {
                    eprintln!("[ERROR]: Error: {}; Error ID: {}", self.log[i], self.log_id[i]);
                    last_error = Some(i);
                }
            }
        }

        if let Some(idx) = last_error {
            panic!(
                "[ERROR]: Final Error: Error: {}; Error ID: {}",
                self.log[idx], self.log_id[idx]
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

        logger.add_marker(None, None);
        logger.add_marker(None, None);

        logger.parse_logger();
    }

    #[test]
    fn logging_test() {
        let mut logger = Logger::new_logger();

        logger.add_log(TEST_LOG, TEST_LOG_ID);
        logger.add_log(TEST_LOG, TEST_LOG_ID);

        logger.parse_logger();
    }

    #[test]
    fn warning_test() {
        let mut logger = Logger::new_logger();

        logger.add_warning(TEST_WARN, TEST_WARN_ID);
        logger.add_warning(TEST_WARN, TEST_WARN_ID);

        logger.parse_logger();
    }

    #[test]
    #[should_panic]
    fn error_test() {
        let mut logger = Logger::new_logger();

        logger.add_error(TEST_ERROR, TEST_ERROR_ID);
        logger.add_error(TEST_ERROR, TEST_ERROR_ID);

        logger.parse_logger();
    }

    #[test]
    #[should_panic]
    fn full_test() {
        let mut logger = Logger::new_logger();

        logger.add_marker(None, None);
        logger.add_log(TEST_LOG, TEST_LOG_ID);
        logger.add_warning(TEST_WARN, TEST_WARN_ID);
        logger.add_error(TEST_ERROR, TEST_ERROR_ID);

        logger.parse_logger();
    }

    use std::sync::Arc;
    use std::thread;

    #[test]
    fn multi_threading_test() {
        let logger = Arc::new(Logger::new_logger());

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
                
                let mut logger_guard = logger_cloned.lock().unwrap();
                logger_guard.add_log(TEST_LOG, TEST_LOG_ID);
                
                let mut data_in_thread = data_cloned.lock().unwrap();
                *data_in_thread += 1;

                if *data_in_thread == n {
                    tx_cloned.send(logger_guard.clone()).unwrap();
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
            logger_guard.add_log(TEST_LOG, TEST_LOG_ID);
            tx.send(logger_guard.clone()).unwrap();
        });

        rx.recv().unwrap();

        handle.join().unwrap();
    }
}
