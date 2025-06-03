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

// To add a marker
logger.add_marker(None, None);
// To add a log 
logger.add_log(TEST_LOG, TEST_LOG_ID);
// To add a Warning
logger.add_warning(TEST_WARN, TEST_WARN_ID);
// To add an Error
// WARNING! Once the logger is finished parsing, if it detected 
// an Error, it will crash your program (This is Intentional)
logger.add_error(TEST_ERROR, TEST_ERROR_ID);

// Parses the logger
logger.parse_logger();
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
    logger_guard.add_marker(None, None);
    tx.send(logger_guard.clone()).unwrap();
});

rx.recv().unwrap();

handle.join().unwrap();
```

### Where to find mini_log's documentation?
There are 2 choices for finding mini_log's documentation
 1. Go to [docs.rs](https://docs.rs/mini_log/latest/mini_log/)
 2. Clone the repository, and type cargo docs --open.

## Why would you use mini_log?
mini_log is used for minimalistic programs. mini_log offers the absolute bare minimum in 
Error Logging. If you don't need a fully fledged logger, this is the best choice! 

## Plans for the Future
 1. Adding no_std support
 2. Adding Recoverable Error Handling.
 3. Optimize the crate further.