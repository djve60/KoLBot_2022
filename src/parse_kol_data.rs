// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of simple parsing functions for kolbot_2022.

// Crates from rust.io.

// Local libraries.
use crate::global_constants;

// What state is the game before logging in?
pub fn parse_landing_page(
    landing_page: &str, 
    debug_level: u8) -> String {

    let system_status: String;

    let system_open = 
        global_constants::KOL_SERVER_ACTIVE_STR.to_string();
    let system_maint = 
        global_constants::KOL_SERVER_MAINTENANCE_STR.to_string();

    if landing_page.contains(&system_open) {
        system_status = global_constants::KOL_OPEN_STR.to_string();
    } else if landing_page.contains(&system_maint) {
        system_status = global_constants::KOL_MAINT_STR.to_string();
    } else {
        system_status = global_constants::KOL_DOWN_STR.to_string();
    }

    if debug_level > 2 {
        log::info!("Sytem status is: {}", system_status);
    }
    system_status
}

// Are we logged in?
pub fn parse_login_page(
    web_page: String, 
    debug_level: u8) -> bool {

    let mut logged_in = false;

    let login_str = global_constants::KOL_LOGIN_SUCCESS_STR.to_string();

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
pub fn parse_logout_page(
    web_page: String, 
    debug_level: u8) -> bool {

    let mut logged_out = false;

    let logout_str = 
        global_constants::KOL_LOGOUT_STR.to_string();

    if web_page.contains(&logout_str) {
        logged_out = true;
    }

    if debug_level > 2 {
        log::info!("Logout status is: {}", logged_out);
    }

    logged_out
}

