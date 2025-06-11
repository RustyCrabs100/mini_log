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
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(not(feature = "no_std"))]
pub mod std_logger {
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
                        eprintln!(
                            "[WARNING]: Warning: {}; Warning ID: {}",
                            self.log[i], self.log_id[i]
                        )
                    }
                    LoggingType::Error => {
                        eprintln!(
                            "[ERROR]: Error: {}; Error ID: {}",
                            self.log[i], self.log_id[i]
                        );
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
}

/// The no_std implementation of std_logger
// #[cfg(feature = "no_std")]
pub mod no_std_logger {
    use core::{ffi::c_char, option::Option, result::Result, usize};

    /// Enum for Overflow Errors
    /// Implements Debug for debugging
    #[derive(Debug)]
    pub enum OverflowError {
        /// Enum Type for when a Log is over it's expected length.
        BufferEntryOverflow,
        /// Enum Type for when accessing beyond the length of an array.
        InvalidIndex(usize),
    }

    /// The maximum amount of Log's that can be made.
    pub const MAX_LOG_ENTRIES: usize = 1024;
    /// The maximum length for a Log
    pub const MAX_LOG_LENGTH: usize = 1024;

    /// The no_std implementation of LoggingType
    /// This does not implement any confort types.
    /// However, it does implement Copy and Clone for easy initalization.
    #[derive(Copy, Clone)]
    pub enum LoggingTypeNS {
        /// Error - For UNRECOVERABLE Error
        /// WARNING! Once the parse_logger_ns function finishes parsing,
        /// it will crash the program if it includes this type.
        /// This is intentional behavior.
        Error,
        /// Warning - Used for potentially hazardous behavior logging.
        Warning,
        /// Log - Used for basic information logging.
        Log,
        /// Marker - Used for marking when a certain point in a program happens.
        Marker,
    }

    pub struct LoggerNS {
        /// log - An array of an array of 1024 c_char's of 1024 arrays
        /// To better describe it, 1024 arrays can be put into this array.
        /// Inside those arrays, 1024 c_char's can be put into it.
        log: [[c_char; MAX_LOG_LENGTH]; MAX_LOG_ENTRIES],
        /// log_id - An array of 1024 usize's
        log_id: [usize; MAX_LOG_ENTRIES],
        /// log_type - An array of 1024 LoggingTypeNS's
        log_type: [LoggingTypeNS; MAX_LOG_ENTRIES],
        /// free_slots - an array of 1024 booleans for constant time lookup
        // TODO: Replace this with a u16 counter for 0(1) lookup time.
        free_slots: [bool; MAX_LOG_ENTRIES]
    }

    impl LoggerNS {
        /// Creates a new LoggerNS with initalized data
        /// Automatically sets all values to the following:
        ///  - log - A null-terminated array
        ///  - log_id - An array of 0
        ///  - log_type - An array of LoggingTypeNS::Marker
        ///  - free_slots - An array of 1024 true's (Tells search that the entire array is empty)
        pub fn new_logger_ns() -> Self {
            Self {
                log: [[0; 1024]; 1024],
                log_id: [0; 1024],
                log_type: [LoggingTypeNS::Marker; 1024],
                // To replace with u16 counter, set the counter value to 0
                free_slots: [true; 1024],
            }
        }

        /// Goes through the arrays, and gets the first empty slot.
        // Potential: Remove this Function (if the way slot accessing changes allows this)
        fn get_next_avaliable_slot(&mut self) -> Option<usize> {
            for i in 0..MAX_LOG_ENTRIES {
                if self.free_slots[i] {
                    // If the function is not removed, this would turn into: ` self.free_slot += 1 '
                    self.free_slots[i] = false;
                    return Some(i);
                }
            }
            None
        }

        /// Adds a marker. 
        /// Inputs : Option<[c_char; 1024], Option<usize>>
        pub fn add_marker(
            &mut self,
            mut message: Option<[c_char; MAX_LOG_ENTRIES]>,
            mut id: Option<usize>,
        ) -> Result<usize, OverflowError> {
            if message == None {
                let message_default = "Marker Place";
                let mut arr: [c_char; MAX_LOG_ENTRIES] = [0; 1024];
                for i in 0..message_default.len() {
                    arr[i] = message_default.as_bytes()[i] as c_char;
                }

                message = Some(arr);
            }
            

            if id == None {
                id = Some(0);
            }
            if let Some(slot) = self.get_next_avaliable_slot() {
                self.log[slot] = Option::expect(message, "ID is None, failed to change value");
                self.log_id[slot] = Option::expect(id, "ID is None, failed to change value");
                self.log_type[slot] = LoggingTypeNS::Marker;
                Ok(slot)
            } else {
                Err(OverflowError::BufferEntryOverflow)
            }
        }

        /// Adds a Log to your Logger.
        /// Inputs: [c_char; 1024], usize
        pub fn add_log(
            &mut self,
            message: [c_char; MAX_LOG_ENTRIES],
            id: usize,
        ) -> Result<usize, OverflowError> {
            if let Some(slot) = self.get_next_avaliable_slot() {
                self.log[slot] = message;
                self.log_id[slot] = id;
                self.log_type[slot] = LoggingTypeNS::Log;
                Ok(slot)
            } else {
                Err(OverflowError::BufferEntryOverflow)
            }
        }

        /// Adds a Warning to your Logger.
        /// Inputs: [c_char; 1024], usize
        pub fn add_warning(
            &mut self,
            message: [c_char; 1024],
            id: usize,
        ) -> Result<usize, OverflowError> {
            if let Some(slot) = self.get_next_avaliable_slot() {
                self.log[slot] = message;
                self.log_id[slot] = id;
                self.log_type[slot] = LoggingTypeNS::Warning;
                Ok(slot)
            } else {
                Err(OverflowError::BufferEntryOverflow)
            }
        }

        /// Adds an Error to your Logger.
        /// WARNING! These Errors are UNRECOVERABLE.
        /// Currently Recoverable Errors are not supported.
        /// Once the parser is done parsing through everything, if it find an Error, it will panic!
        /// Inputs: [c_char; 1024], usize
        pub fn add_error(
            &mut self,
            message: [c_char; 1024],
            id: usize,
        ) -> Result<usize, OverflowError> {
            if let Some(slot) = self.get_next_avaliable_slot() {
                self.log[slot] = message;
                self.log_id[slot] = id;
                self.log_type[slot] = LoggingTypeNS::Error;
                Ok(slot)
            } else {
                Err(OverflowError::BufferEntryOverflow)
            }
        }   

        /// Parses the LoggerNS.
        /// Inputs: 
        ///  Fn([c_char; 1024], usize, LoggingTypeNS) -> Result<(), OverflowError>
        ///  Fn([c_char; 1024], usize, LoggingTypeNS) -> ()
        pub fn parse_logger<
            S: Fn([c_char; 1024], usize, LoggingTypeNS) -> Result<(), OverflowError>,
            P: Fn([c_char; 1024], usize, LoggingTypeNS) -> (),
        > (&self, printer: S, crasher: P) -> Result<(), OverflowError> {
            let mut last_error: Option<usize> = None;

            for i in 0..self.log.len() {
                if !self.free_slots[i] {
                    match self.log_type[i] {
                        LoggingTypeNS::Marker | LoggingTypeNS::Log | LoggingTypeNS::Warning => {
                            if let Err(_) = printer(self.log[i], self.log_id[i], self.log_type[i]) {
                                return Err(OverflowError::BufferEntryOverflow);
                            }
                        }
                        LoggingTypeNS::Error => {
                            crasher(self.log[i], self.log_id[i], self.log_type[i]);
                            last_error = Some(self.log_id[i]);
                        }
                    }
                }    
            }

            if let Some(error_index) = last_error {
                return Err(OverflowError::InvalidIndex(error_index));
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod mini_log_tests {
    use super::std_logger;

    #[test]
    fn marking_test() {
        let mut logger = std_logger::Logger::new_logger();

        logger.add_marker(None, None);
        logger.add_marker(None, None);

        logger.parse_logger();
    }

    #[test]
    fn logging_test() {
        let mut logger = std_logger::Logger::new_logger();

        logger.add_log(std_logger::TEST_LOG, std_logger::TEST_LOG_ID);
        logger.add_log(std_logger::TEST_LOG, std_logger::TEST_LOG_ID);

        logger.parse_logger();
    }

    #[test]
    fn warning_test() {
        let mut logger = std_logger::Logger::new_logger();

        logger.add_warning(std_logger::TEST_WARN, std_logger::TEST_WARN_ID);
        logger.add_warning(std_logger::TEST_WARN, std_logger::TEST_WARN_ID);

        logger.parse_logger();
    }

    #[test]
    #[should_panic]
    fn error_test() {
        let mut logger = std_logger::Logger::new_logger();

        logger.add_error(std_logger::TEST_ERROR, std_logger::TEST_ERROR_ID);
        logger.add_error(std_logger::TEST_ERROR, std_logger::TEST_ERROR_ID);

        logger.parse_logger();
    }

    #[test]
    #[should_panic]
    fn full_test() {
        let mut logger = std_logger::Logger::new_logger();

        logger.add_marker(None, None);
        logger.add_log(std_logger::TEST_LOG, std_logger::TEST_LOG_ID);
        logger.add_warning(std_logger::TEST_WARN, std_logger::TEST_WARN_ID);
        logger.add_error(std_logger::TEST_ERROR, std_logger::TEST_ERROR_ID);

        logger.parse_logger();
    }

    use std::sync::Arc;
    use std::thread;

    #[test]
    fn multi_threading_test() {
        let logger = Arc::new(std_logger::Logger::new_logger());

        let mut logger_2nd_thread = Arc::clone(&logger);
        thread::spawn(move || {
            std_logger::Logger::parse_logger(&mut logger_2nd_thread);
        });
    }

    use std::sync::mpsc;
    use std::sync::Mutex;

    #[test]
    fn multi_threading_transfer_test() {


        let logger = Arc::new(Mutex::new(std_logger::Logger::new_logger()));

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
                logger_guard.add_log(std_logger::TEST_LOG, std_logger::TEST_LOG_ID);

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
        let logger = Arc::new(Mutex::new(std_logger::Logger::new_logger()));

        let logger_cloned = Arc::clone(&logger);
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut logger_guard = logger_cloned.lock().unwrap();
            logger_guard.add_log(std_logger::TEST_LOG, std_logger::TEST_LOG_ID);
            tx.send(logger_guard.clone()).unwrap();
        });

        rx.recv().unwrap();

        handle.join().unwrap();
    }
}
