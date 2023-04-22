// Copyright D. Evans (djve60+kolbot@gmail.com).

// Centralized regex handling as recommended in:
// https://docs.rs/regex/latest/regex/.
// Also other general string handling of the KoL messages for formatting.

// Anything here should be public.


// Crates from rust.io.
use lazy_static::lazy_static;
use regex::{NoExpand, Regex};

// Local libraries.
use crate::global_constants;


// Do not want to process messages that have nothing to do.
pub fn empty_message_test(
    text: &str) -> bool {

    lazy_static! {
        static ref RE_EMPTY: Regex = 
            Regex::new(global_constants::CHAT_EMPTY_CHAT_MESSAGE_RE).unwrap();
    }
    RE_EMPTY.is_match(text)
}


// Most calls need the account ID number and the password hash for the sesson.
pub fn get_account_id_and_pwdhash(
    text: &str) -> (String, String) {

    lazy_static! {
        static ref RE_AP: Regex = 
            Regex::new(global_constants::ACCOUNT_DATA_RE).unwrap();
    }

    let data = RE_AP.captures(text).unwrap();

    (data[1].to_string(), data[2].to_string())
}


// Find the chat channels for this account.
pub fn get_chat_channels(
    text: String) -> Vec<std::string::String> {

    lazy_static! {
        static ref RE_CC: Regex = 
            Regex::new(global_constants::CHAT_CHANNEL_NAME_RE).unwrap();
    }

    //let mut channel_name: &str = Vec::new();
    let mut channel_names =
        Vec::<String>::new();

    for channel in RE_CC.captures_iter(&text) {
        channel_names.push(channel[1].to_string());
    }

    channel_names
}


// Return the "last message value".
pub fn get_chat_last_message_id(
    text: String) -> String {

    lazy_static! {
        static ref RE_LMI: Regex = 
            Regex::new(global_constants::CHAT_LAST_MESSAGE_ID_RE).unwrap();
    }

    let last_message_value =
        RE_LMI.captures(&text).unwrap();

    last_message_value[1].to_string()
}


// Get the first word out of a possible string.
pub fn get_first_word(
    text: &str) -> String {

    lazy_static! {
        static ref RE_START_WORD: Regex = 
            //Regex::new(r#"^([^\p{White_Space}]+)"#).unwrap();
            Regex::new("^([^\\p{White_Space}]+)").unwrap();
    }

    match RE_START_WORD.captures(text) {
        Some(data) => {
            data[1].trim().to_string()
        }
        None => {
            println!("In default");
            text.to_string()
        }
    }
}


// Return the message payload as an iterative resource for "chat" messages.
pub fn get_public_messages_details(
    text: &String) -> regex::CaptureMatches<'static, '_> {

    lazy_static! {
        static ref RE_PMD: Regex =
            Regex::new(global_constants::CHAT_PUBLIC_MESSAGE_RE).unwrap();
    }

    RE_PMD.captures_iter(text)
}


// Return the message payload as an iterative resource for "private" messages.
pub fn get_private_messages_details(
    text: &String) -> regex::CaptureMatches<'static, '_> {

    lazy_static! {
        static ref RE_PVT_MD: Regex =
            Regex::new(global_constants::CHAT_PRIVATE_MESSAGE_RE).unwrap();
    }

    RE_PVT_MD.captures_iter(text)
}


// Return the message payload as an iterative resource for "event" messages.
pub fn get_event_messages_details(
    text: &String) -> regex::CaptureMatches<'static, '_> {

    lazy_static! {
        static ref RE_EMD: Regex =
            Regex::new(global_constants::CHAT_EVENT_MESSAGE_RE).unwrap();
    }

    RE_EMD.captures_iter(text)
}


// Remove the Cartoon Harpoon (CH) or On Safari effects.
pub fn remove_cartoon_harpoon_effect(
    text: &String) -> String {

    let mut text_new: String = text.to_string();

    lazy_static! {
        static ref RE_CH: Regex =
            Regex::new(global_constants::CHAT_CARTOON_HARPOON_RE).unwrap();
        static ref RE_APPENDED: Regex =
            Regex::new("<.+?$").unwrap();
    }

    for capture in RE_CH.captures_iter(text) {

        // Need to replace the whitespace that was used to make the
        // regex work correctly, as there is no assertions in rust's
        // regex parser.
        text_new = RE_CH.replace(
                        &text_new,
                        capture[1].to_string() + " " ).to_string();
    }

    // The appended <!--fb--> may be truncated, when posted from the server.
    // no idea why. Now to remove it. Since "<" must come from the server, as 
    // user post it comes up as "&lt;", this is now trivial.
    text_new = RE_APPENDED.replace(&text_new, "").to_string();

    text_new
}


pub fn remove_eso_si_que_es_effect (
    text: &String) -> String {

    let mut text_new: String = text.to_string();

    for (k, v) in global_constants::SPANISH_LETTERS.iter() {

        text_new = text_new.replace(k, v);
    }
    text_new.to_string()
}


// Remove the Golden Gum (GG) effect.
pub fn remove_golden_gum_effect(
    text: &String) -> String {

    lazy_static! {
        static ref RE_GG: Regex =
            Regex::new(global_constants::CHAT_GOLDEN_GUM_RE).unwrap();
    }

    match RE_GG.captures(text) {
        Some(data) => {
            data[1].trim().to_string()
        }
        None => {
            text.to_string()
        }
    }
}


// Remove the Holiday Fun (HF) effect.
pub fn remove_holiday_fun_effect (
    text: &String) -> String {

    lazy_static! {
        static ref RE_HF: Regex =
            Regex::new(global_constants::CHAT_HOLIDAY_FUN_RE).unwrap();
    }

    let mut clean_string = String::new();

    for capture in RE_HF.captures_iter(text) {
        clean_string.push_str(&capture[1].to_string());
    }

    if clean_string.is_empty() {
        text.trim().to_string()
    } else {
        clean_string
    }
}


pub fn remove_italics_effect(
    text: &String) -> String {

    let text = text.replace(
                global_constants::CHAT_EMOTION_SICKNESS_TEST_STR, "");

    let text = text.replace(
                global_constants::CHAT_CLOSE_ITALICS_STR, "");

    text.trim().to_string()
}


// Remove the Lolsipop (L) effect.
pub fn remove_lolsipop_effect(
    text: &String) -> String {

    lazy_static! {
        static ref RE_L: Regex =
            Regex::new(global_constants::CHAT_LOLSIPOP_RE).unwrap();
    }

    match RE_L.captures(text) {
        Some(data) => {
            data[1].trim().to_string()
        }
        None => {
            text.to_string()
        }
    }
}


// Remove the Pirate Bellow (PB) effect, and LOV Emotionizer (LE) effect.
// They can be cast together AND either can appear as the outermost or
// innermost effect. Whichever is cast first is the innermost effect.
pub fn remove_end_decorators_effect(
    text: &String) -> String {

    lazy_static! {
        static ref RE_LE: Regex =
            Regex::new(global_constants::CHAT_LOV_EMOTIONIZER_RE).unwrap();
        static ref RE_PB_END: Regex =
            Regex::new(global_constants::CHAT_PIRATE_BELLOW_END_RE).unwrap();
        static ref RE_PB_START: Regex =
            Regex::new(global_constants::CHAT_PIRATE_BELLOW_START_RE).unwrap();
    }

    let text = RE_LE.replace_all(text, NoExpand(""));

    let text = RE_PB_END.replace_all(&text, NoExpand(""));

    let text = RE_PB_START.replace_all(&text, NoExpand(""));

    text.trim().to_string()
}


// Remove the Red Pill (RP) effect.
pub fn remove_red_pill_effect(
    text: &String) -> String {

    lazy_static! {
        static ref RE_RP: Regex =
            Regex::new(global_constants::CHAT_RED_PILL_RE).unwrap();
    }

    match RE_RP.captures(text) {
        Some(data) => {
            data[1].trim().to_string()
        }
        None => {
            text.to_string()
        }
    }
}


// Remove the Purple Prose, Vampyric Cloake, and Vivala mask effects.
pub fn remove_vampyric_cloake_effect (
    text: &String) -> String {

    // Get rid of "<!--viva-->", for V for Vivala mask.
    let text = 
        text.replace(
            global_constants::CHAT_VIVALA_MASK_TEST_STR, "");

    // Get rid of "<font color=\"purple\">"
    let text = 
        text.replace(
            global_constants::CHAT_PURLE_PROSE_TEST_STR, "");

    // Get rid of "<font color=darkred>", for Vampyric Cloake.
    let text = 
        text.replace(
            global_constants::CHAT_VAMPYRIC_CLOAKE_TEST_STR, "");

    // Get rid of "<font color=red>", for V for Vivala mask.
    let text = 
        text.replace(
            global_constants::CHAT_VIVALA_MASK_FONT_STR, "");

    // Get rid of "</font>".
    let text = text.replace(
            global_constants::CHAT_CLOSE_FONT_STR, "");

    // Get rid of "</b>".
    let text = text.replace(
            global_constants::CHAT_CLOSE_BOLD_STR, "");

    // Get rid of "<!--fb-->".
    let text = text.replace(
            global_constants::CHAT_CARTOON_HARPOON_TEST_STR, "");

    text.to_string()
}


pub fn get_rollover_values(
    text: String) -> (u32, u32, u32) {

    lazy_static! {
        static ref RE_RT: Regex = 
            Regex::new(global_constants::CHAT_CHANNEL_ROLLOVER_TIME_RE).unwrap();
    }

    let time_values =
        RE_RT.captures(&text).unwrap();

    let hours: u32 = time_values[1].to_string().parse::<u32>().unwrap_or(0);
    let minutes: u32 = time_values[2].to_string().parse::<u32>().unwrap_or(0);
    let seconds: u32 = time_values[3].to_string().parse::<u32>().unwrap_or(0);

    (hours, minutes, seconds)
}
