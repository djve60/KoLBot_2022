// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of logging functions for kolbot_2022.

// Crates from rust.io.
use configparser::ini::Ini;
use flexi_logger::{
    Age::Day,
    //Cleanup::Never,
    Cleanup::KeepCompressedFiles,
    Criterion::Age,
    FileSpec,
    Logger,
    LoggerHandle,
    Naming::Timestamps,
};
use log::*;

// Local crates

// Start the logging system and return a handle to the logger.
pub fn initialize_logging(configuration: &Ini, mut log_name: &str) -> LoggerHandle {
    //println!("In logging: {:#?}", configuration);
    let log_dir = configuration.get("directories", "logging").unwrap();

    // If nothing is supplied use the crate name.
    if log_name.is_empty() {
        log_name = env!("CARGO_PKG_NAME");
        println!("Log name {}", log_name);
    }

    let logger =
        //Logger::try_with_str("debug")
        Logger::try_with_str("info")
            .unwrap_or_else(|e|
                panic!("Logger initialization failed with {}", e))
            .log_to_file(
                FileSpec::default()
                    .directory(log_dir)
                    .basename(log_name)
                    //.use_timestamp(true))
                    .use_timestamp(false))
            //usize::MAX is 18,446,744,073,709,551,615.
            .rotate(Age(Day), Timestamps, KeepCompressedFiles(usize::MAX))
            .start()
            .unwrap_or_else(|e|
                panic!("LoggerHandle initialization failed with {}", e));

    info!("Logging system initialized.");
    // Leaving the level calls in for other people, if they want to
    // see what happens when changing levels, like I did.
    // log::debug!("Test 1");
    // log::trace!("Test 2");
    // log::warn!("Test 3");
    // log::error!("Test 4");
    // log::info!("Test 5");

    logger
}

pub fn shutdown_logging(logger: LoggerHandle) {
    info!("Logging system shutting down.");
    logger.shutdown();
}
