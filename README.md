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

### Multi-threaded usage
```
// Imports all the needed things
use std::sync{mpsc, Mutex, Arc};
use mini_log::{LoggingType, Logger}

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
```

## Why would you use mini_log?
mini_log is used for minimalistic programs. mini_log offers the absolute bare minimum in 
Error Logging. If you don't need a fully fledged logger, this is the best choice! 

## Plans for the Future
 1. Adding no_std support
 2. Optimize the crate further.