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
    #[default]
    Log
}

/// A Logger Struct for containing logging values.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Logger {
    /// Contains the Logging Message.
    log: String,
    /// Contains the Logging ID.
    log_id: usize,
    /// Contains the Logging Type.
    log_type: LoggingType
}

impl Logger {

    /// Creates a new Logger Vector.
    pub fn new_logger() -> Vec<Logger> {
        vec![Logger { log: "Logger Initalized".to_string(), log_id: 0, log_type: LoggingType::Log}]
    }

    /// Adds a Log value to the Logger.
    pub fn add_log(
        log: &str,
        log_id: usize,
        collector: &mut Vec<Logger>, 
    ) {
        collector.push(Logger {
            log: log.to_string(), 
            log_id,
            log_type: LoggingType::Log,
        })
    }

    /// Adds a Warning value to the Logger.
    pub fn add_warning(
        log: &str,
        log_id: usize,
        collector: &mut Vec<Logger>
    ) {
        collector.push(Logger {
            log: log.to_string(),
            log_id,
            log_type: LoggingType::Warn
        });
    }

    /// Adds an Error value to the Logger.
    /// WARNING! This is designed for UNRECOVERABLE Errors.
    /// Currently, recoverable errors are not supported.
    pub fn add_error(
        log: &str,
        log_id: usize,
        collector: &mut Vec<Logger>
    ) {
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
                LoggingType::Log => println!("[LOG]: Info: {}; Info ID: {}", items.log, items.log_id),
                LoggingType::Warn => {
                    eprintln!("[WARNING]: Warning: {}; Warning ID: {}", items.log, items.log_id)
                }, 
                LoggingType::Error => {
                    eprintln!("[ERROR]: Error: {}; Error ID: {}", items.log, items.log_id);
                    last_error = Some(items)
                }
            }
        }

        if let Some(err) = last_error {
            panic!("[ERROR]: Final Error: Error: {}; Error ID: {}", err.log, err.log_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


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
}
