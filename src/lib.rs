// Copyright D. Evans (djve60+kolbot@gmail.com).

/// CLI parsing and startup data initialization.
pub mod initialization;

/// The chat channel posts are what drives this code.
/// This keeps the polling, parsing, and spawning of threads
/// to enact any commands in the one file.
pub mod chat;

/// Everything related to the chat channel "clan".
pub mod channel_clan;

/// Keeping all the global constants in one file.
pub mod global_constants;

/// A collection of functions for HTTP calls to Kingdom Of Loathing.
pub mod connection;

/// The KoL acceptable fortune functions.
pub mod fortune;

/// Global variables with proper handling.
pub mod global_variables;

/// A collection of functions for message logging.
pub mod logging;

/// A collection of macros for reusing strings.
pub mod macros;

/// A collection of functions for handling and actioning KoL chat messages.
pub mod message_handling;

/// A collection of functions for getting data from KoL web pages.
pub mod parse_kol_data;

/// A collection of helper functions to avoid recompiling in loops.
pub mod regular_expressions;

/// The rules and constants for the clan basement areas.
pub mod rules;
pub mod rules_constants;

/// A function to create a signal handler.
pub mod signals;

/// A collection of simple functions required in a lot of places.
pub mod utilities;
