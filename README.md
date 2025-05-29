# mini_log
A library for you to log all your warnings, errors, and information, all at once!

## How to use
```
// Imports everything
use mini_log::{
    LoggingType, Logger, TEST_LOG, TEST_LOG_ID,
    TEST_WARN, TEST_WARN_ID, TEST_ERROR, 
    TEST_ERROR_ID, 
};

// Initalizes the Logger
let mut logger = Logger::new_logger();

// To add in a log
Logger::add_log(TEST_LOG, TEST_LOG_ID, &mut logger);
// To add in a Warning.
Logger::add_warning(TEST_WARN, TEST_WARN_ID, &mut logger);
// To add in an Error.
Logger::add_error(TEST_ERROR, TEST_ERROR_ID, &mut logger);

// To parse the logger.
// Once this parses, it's going to print out the log,
// Error print the warning,
// Error print the errors, then panics on the last error.
Logger::parse_logger(&logger);
```

## Why would you use mini_log?
mini_log is used for minimalistic programs. mini_log offers the absolute bare minimum in 
Error Logging. If you don't need a fully fledged logger, this is the best choice! 

## Plans for the Future
 1. Adding no_std support
 2. Optimize the crate further.