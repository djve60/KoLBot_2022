// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of global constants for kolbot_2022.
// If it ends in "RE" it's a regular expression. All other 
// values should be obvious.

// Need to "use crate::constants".
// Refrenced with "constants::<NAME>".

// https://stackoverflow.com/questions/1732348/ is correct. But this is
// to parse semi-HTML fragments. So a regex is as good as anything else, and
// an XML parser doesn't work as it's not any sort of mark-up language
// at this point.
// See documentation/message.txt for the JSON payload.


use std::collections::HashMap;

// Crates from rust.io.
use lazy_static::lazy_static;

// For matching the landing page and finding the current server status.
pub const KOL_OPEN_STR: &str = "running";
pub const KOL_MAINT_STR: &str = "maintenance";
pub const KOL_DOWN_STR: &str = "unknown";

// HTTP scheme seperator.
pub const HTML_SCHEME_SEPERATOR: &str = "://";

// KoL server status.
pub const KOL_SERVER_ACTIVE_STR: &str = 
    "class=text type=text name=loginname";
pub const KOL_SERVER_MAINTENANCE_STR: &str = 
    "The system is currently down for nightly maintenance";

// Web frame strings.
pub const KOL_LOGIN_SUCCESS_STR: &str = "name=mainpane"; 
pub const KOL_LOGOUT_STR: &str = "You have been logged out";

// For maintence periods in KoL.
// At what minute time we expect rollover to finish at.
// 19:36 for Sun to Friday, 19:41 on Saturdays..
pub const GENERAL_MAINT: u16 = 36;
pub const SATURDAY_MAINT: u16 = 41;

// Need an empty strfor some calls.
pub const EMPTY_STR: &str = "";

// HTML hierarchy/directory delimiter.
pub const HTML_DELIMITER: &str = "/";

// Regular expression to match JSON strings to boolean logic.
pub const JSON_FALSE_STR: &str = "false";
pub const JSON_TRUE_STR: &str = "true";


// Regular expression to capture the account id and password hash.
// This is needed on most requests to the KoL servers.
pub const ACCOUNT_DATA_RE: &str = 
    r#"playerid = (\p{Nd}+),\p{White_Space}+pwdhash[^"]+["]([^"]+)"#;


// Regular expression to capture the chat channel's data.
pub const CHAT_EMPTY_CHAT_MESSAGE_RE: &str =
    r#"("msgs":\[\],"last")"#;
pub const CHAT_FULL_CHAT_MESSAGE_RE: &str =
    r#"\{"msg":"(.+?)","type":"(.+?)","mid":"(\d+)".+?"name":"(.+?)","id":"(\d+)".+?"format":"(\d+)","channel":"(.+?)""#;
// Event messages have servaral formats.
pub const CHAT_FULL_EVENT_MESSAGE_RE: &str =
    //r#"\{"type":"(event)","msg":".+?who=(\d+)'.+?"link":"?([^"]+)"?.+?"time":"(\d+)"#;
    r#"\{"type":"(event)","msg":".+?who=(\d+)'.+?>([^<]+).+link":"(.+?)","#;
pub const CHAT_CHANNEL_NAME_RE: &str = 
    r"channel:\p{White_Space}+'(.+?)',\p{White_Space}+msg:";
pub const CHAT_LAST_MESSAGE_ID_RE: &str = 
    r#"last":"(\p{Nd}+)","delay":"#;
pub const CHAT_AWAY_MESSAGE_STR: &str = 
    "You are now in away mode";


// Chat channel sleep time between checking for messages, in
// milliseconds. KoL code checks every 300ms.
//pub const CHAT_CHANNEL_MSEC_SLEEP_DURATION: u32 = 500;
pub const CHAT_CHANNEL_MSEC_SLEEP_DURATION: u32 = 5000;
// 500ms * 2 == 1 sec. Multiple by 60 for minutes. This makes the last number
// how many minutes to wait until checking connectivity.
pub const CHAT_CHANNEL_HEARTBEAT_COUNTDOWN: u32 = 500 * 2 * 60 * 15; 

pub const CHAT_CHANNEL_ROLLOVER_COMMAND: &str = "/rollover";
pub const CHAT_CHANNEL_ROLLOVER_CHANNEL: &str = "clan";
pub const CHAT_CHANNEL_ROLLOVER_TIME_RE: &str = 
    r#"It is currently .+R-(\p{Nd}{2}):(\p{Nd}{2}):(\p{Nd}{2})<"#;


// https://kol.coldfront.net/thekolwiki/index.php/Chat_Guide:_Miscellaneous
// Messages may have HTML code embedded by the server. Strip them out!
pub const CHAT_CLOSE_BOLD_STR: &str = 
    r#"<\/b>"#;
pub const CHAT_CLOSE_FONT_STR: &str = 
    r#"<\/font>"#;
pub const CHAT_CLOSE_ITALICS_STR: &str = 
    r#"<\/i>"#;
// Cartoon Harpoon uses "fb" at the end the post, but it can be truncated.
// On Safari uses the exact same code.
pub const CHAT_CARTOON_HARPOON_TEST_STR: &str = 
    //r#"<!--fb-->"#;
    r#"<i title="#;
// Need the whitespace to have the regex work but you need to replace
// the whitespace when fixing this effect.
pub const CHAT_CARTOON_HARPOON_RE: &str = 
    r#"<i title=\\"([^"]+)\\">.+?\p{White_Space}"#;
// Emotion Sickness turns individual words to italics.
// This means appended chat effect messages are regular text
// after this.
pub const CHAT_EMOTION_SICKNESS_TEST_STR: &str = 
    r#"<i>"#;
pub const CHAT_GOLDEN_GUM_TEST_STR: &str = 
    r#"<span style=\"color: #AE8419; font-weight: bold\"><!--gg-->"#;
pub const CHAT_GOLDEN_GUM_RE: &str = 
    r#"<span.+#AE8419.+gg-->(.+?)<"#;
pub const CHAT_HOLIDAY_FUN_TEST_STR: &str =
    r#"<!--hi--><font color="#;
// Thanks to VizerTim for pointing out the data inconsistency, from the 
// KoL server when HTML codes are used to replace symbols.
// Could be something as long as " &quote; ".
pub const CHAT_HOLIDAY_FUN_RE: &str = 
    r#">([^<>]+)[<"]"#;
pub const CHAT_LOLSIPOP_TEST_STR: &str = 
    r#";\"><!--lp-->"#;
pub const CHAT_LOLSIPOP_RE: &str = 
    r#"<!--lp-->(.+)<\\/span>"#;
// The CloudFront identifier may change. That's the part between the
// "https://" and "cloudfront.net". So it's ommited in the check.
pub const CHAT_LOV_EMOTIONIZER_TEST_STR: &str =
    r#"cloudfront.net\/otherimages\/12x12heart.png"#;
pub const CHAT_LOV_EMOTIONIZER_RE: &str =
    r#"<img [^>]+cloudfront.net\\/otherimages\\/12x12heart.png[^>]+?>"#;
// Jazz Hands is appended to the post, if it appears at all.
pub const CHAT_JAZZ_HANDS_TEST_STR: &str = 
    r#"<i>~jazz hands~</i>"#;
// The CloudFront identifier may change. That's the part between the
// "https://" and "cloudfront.net". So it's ommited in the check.
pub const CHAT_PIRATE_BELLOW_TEST_STR: &str =
    r#"cloudfront.net\/otherimages\/12x12skull.gif"#;
pub const CHAT_PIRATE_BELLOW_END_RE: &str =
    r#"<img [^>]+?cloudfront.net\\/otherimages\\/12x12skull.gif[^>]+?><\\/b>"#;
pub const CHAT_PIRATE_BELLOW_START_RE: &str =
    r#"<b><img [^>]+?cloudfront.net\\/otherimages\\/12x12skull.gif[^>]+?>"#;
// Purple Prose is another static HTML color override.
pub const CHAT_PURLE_PROSE_TEST_STR: &str =
    r#"<font color=\"purple\">"#;
// Red Pill is javascript, in the browser, by the looks of it.
pub const CHAT_RED_PILL_TEST_STR: &str = 
    r#"<!--js(redpill())-->"#;
pub const CHAT_RED_PILL_RE: &str = 
    r#"<!--js\\(redpill\\(\\)\\)-->([^<]+)<"#;
// Vampyric Cloake only needs the font coloring removed.
// See CHAT_CLOSE_FONT_STR for the other end.
pub const CHAT_VAMPYRIC_CLOAKE_TEST_STR: &str =
    r#"<font color=darkred>"#;
// The "V for Vivala mask" is very close to the vampyric cloake.
pub const CHAT_VIVALA_MASK_FONT_STR: &str =
    r#"<font color=red><b>"#;
pub const CHAT_VIVALA_MASK_TEST_STR: &str =
    r#"<!--viva-->"#;

// Eso Sí Que Es (effect from eating spaghetti con calaveras) changes 
// some letters to the the equivalent Spanish accented using HTML 
// character code.
lazy_static! {
    pub static ref SPANISH_LETTERS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("&#193;",  "A"); //Á
        m.insert("&#225;",  "a"); //á
        m.insert("&#201;",  "E"); //É
        m.insert("&#223;",  "e"); //é
        m.insert("&#205;",  "I"); //Í
        m.insert("&#237;",  "i"); //í
        m.insert("&#209;;", "N"); //Ñ
        m.insert("&#241;;", "n"); //ñ
        m.insert("&#211;",  "O"); //Ó
        m.insert("&#243;",  "o"); //ó
        m.insert("&#218;",  "U"); //Ú
        m.insert("&#250;",  "u"); //ú
        m.insert("&#220;",  "U"); //Ü
        m.insert("&#252;",  "u"); //ü
        m.insert("&#191;",   ""); //¿
        m.insert("&#161;",   ""); //¡
        m
    };
}

