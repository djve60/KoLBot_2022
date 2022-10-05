// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of global constants for kolbot_2022.

// Need to "use crate::constants".
// Refrenced with "constants::<NAME>".

// For matching the landing page and finding the current server status.
pub const KOL_OPEN_STR: &str = "running";
pub const KOL_MAINT_STR: &str = "maintenance";
pub const KOL_DOWN_STR: &str = "unknown";

// HTTP scheme seperator.
pub const HTML_SCHEME_SEPERATOR: &str = "://";

// For maintence periods in KoL.
// At what minute time we expect rollover to finish at.
// 19:36 for Sun to Friday, 19:41 on Saturdays..
pub const GENERAL_MAINT: u16 = 36;
pub const SATURDAY_MAINT: u16 = 41;

// Need an empty strfor some calls.
pub const EMPTY_STR: &str = "";

// HTML hierarchy/directory delimiter.
pub const HTML_DELIMITER: &str = "/";

// Regular expression to capture the account id and password hash.
// This is needed on most requests to the KoL servers.
pub const ACCOUNT_DATA_RE: &str = r#"playerid = (\p{Nd}+),\p{White_Space}+pwdhash[^"]+["]([^"]+)"#;

// Regular expression to capture the chat channels.
// This comes from the HTML code.
// Example: "channel: 'clan', msg:".
pub const CHAT_CHANNEL_NAME_RE: &str = r"channel:\p{White_Space}+'(.+?)',\p{White_Space}+msg:";
pub const CHAT_CHANNEL_LAST_MESSAGE_RE: &str = r##"last":"(\p{Nd}+)","delay":"##;

// Chat channel sleep time between checking for messages.
// KoL does checks every 300.
pub const CHAT_CHANNEL_MSEC_SLEEP_DURATION: u64 = 500;
