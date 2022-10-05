// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of simple parsing functions for kolbot_2022.

// Crates from rust.io.
use configparser::ini::Ini;
use regex::Regex;

// Local libraries.
use crate::constants;

// What state is the game before logging in?
pub fn parse_landing_page(kol_strings: &Ini, landing_page: &str, debug_level: u8) -> String {
    let system_status: String;

    let system_open = kol_strings.get("connection", "active_str").unwrap();
    let system_maint = kol_strings.get("connection", "maintenance_str").unwrap();

    if landing_page.contains(&system_open) {
        system_status = constants::KOL_OPEN_STR.to_string();
    } else if landing_page.contains(&system_maint) {
        system_status = constants::KOL_MAINT_STR.to_string();
    } else {
        system_status = constants::KOL_DOWN_STR.to_string();
    }

    if debug_level > 2 {
        log::info!("Sytem status is: {}", system_status);
    }
    system_status
}

// Are we logged in?
pub fn parse_login_page(kol_strings: &Ini, web_page: String, debug_level: u8) -> bool {
    let mut logged_in = false;

    let login_str = kol_strings.get("connection", "login_str").unwrap();

    if web_page.contains(&login_str) {
        logged_in = true;
    }

    if debug_level > 2 {
        log::info!("In parse_login_page().");
        log::info!("Login status is: {}", logged_in);
    }
    logged_in
}

// Have we logged out?
pub fn parse_logout_page(kol_strings: &Ini, web_page: String, debug_level: u8) -> bool {
    let mut logged_out = false;

    let logout_str = kol_strings.get("connection", "logout_str").unwrap();

    if web_page.contains(&logout_str) {
        logged_out = true;
    }

    if debug_level > 2 {
        log::info!("Logout status is: {}", logged_out);
    }

    logged_out
}

// Account data is needed on most actions.
pub fn get_account_data(web_frame: String) -> (String, String) {
    //pub fn get_account_data(web_frame: String) {

    //println!("WF: {:#>}", &web_frame);
    let account_data_re = Regex::new(constants::ACCOUNT_DATA_RE).unwrap();

    let account_data = account_data_re.captures(&web_frame).unwrap();

    (account_data[1].to_string(), account_data[2].to_string())
}

// Find the chat channels for this account.
pub fn get_chat_channels(web_frame: String) -> Vec<std::string::String> {
    let channel_name_re = Regex::new(constants::CHAT_CHANNEL_NAME_RE).unwrap();

    //let mut channel_name: &str = Vec::new();
    let mut channel_names = Vec::<String>::new();

    for channel in channel_name_re.captures_iter(&web_frame) {
        channel_names.push(channel[1].to_string());
    }

    channel_names
}

// Return messages and the "last message value".
// Need to work on splitting the messages into an
pub fn get_chat_message_data(chat_data: String) -> String {
    let chat_last_message_value_re = Regex::new(constants::CHAT_CHANNEL_LAST_MESSAGE_RE).unwrap();

    let last_message_value = chat_last_message_value_re.captures(&chat_data).unwrap();

    // This produces a waring when checking with clippy but
    // this is temporary output while developing the input
    // message handling.
    // Without to_string() there is the compile time error of
    // "doesn't have a size known at compile-time".
    println!("CD: {:#>}", last_message_value[0].to_string());

    last_message_value[1].to_string()
}
