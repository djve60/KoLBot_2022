// Copyright D. Evans (djve60+kolbot@gmail.com).

/// CLI parsing and startup data initialization.
pub mod initialization;

/// The chat channel posts are what drives this code.
/// This keeps the polling, parsing, and spawning of threads
/// to enact any commands in the one file.
pub mod chat;

/// Keeping all the global constants in one file.
pub mod constants;

/// A collection of functions for HTTP calls to Kingdom Of Loathing.
pub mod connection;

/// A collection of functions for message logging.
pub mod logging;

/// A collection of macros for reusing strings.
pub mod macros;

/// A collection of functions for getting data from KoL web pages.
pub mod parse_kol_data;

/// A function to create a signal handler.
pub mod signals;

/// A collection of simple functions required in a lot of places.
pub mod utilities;
