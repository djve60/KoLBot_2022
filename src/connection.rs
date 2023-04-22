// Copyright D. Evans (djve60+kolbot@gmail.com).

// Connection functions for kolbot_2022.

use std::{thread, time::Duration};

// Crates from rust.io.
// See Cargo.toml, is it time to remove chrono?
use chrono::{Datelike, Timelike, Utc};
use chrono_tz::US::Arizona;
use configparser::ini::Ini;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};

// Local libraries.
use crate::global_constants;
use crate::parse_kol_data;

// A structure to pass around the results from an HTTTP call.
// By default struct fields are private, not helpful in this case.
pub struct HttpResult {
    pub body: String,
    pub headers: HeaderMap,
    pub referer: String,
    pub status: StatusCode,
}

// Initialization top level function.
pub fn login (configuration: &Ini, debug_level: u8) -> (bool, reqwest::Client) {
    let mut retry: bool = true;
    let mut login_status: bool = false;

    let mut client = initialize_client(global_constants::EMPTY_STR);

    while retry {
        let (mut game_status, mut landing_page_data) =
            check_game_status(&mut client, configuration, debug_level);

        // Sleep until the daily maintenance period is finished.
        while game_status.eq(&global_constants::KOL_MAINT_STR.to_string()) {
            let sleep_time = get_rollover_sleep_length();
            thread::sleep(Duration::from_secs(sleep_time as u64));

            (game_status, landing_page_data) =
                check_game_status(&mut client, configuration, debug_level);
        }

        // May be a network issue during rollover.
        while game_status.eq(&global_constants::KOL_DOWN_STR.to_string()) {
            // Network error, or something else. Sleep for 2 minutes
            // and retry, forever. Sometimes a home network can be
            // down for hours.
            thread::sleep(Duration::from_secs(120));

            (game_status, landing_page_data) =
                check_game_status(&mut client, configuration, debug_level);
        }

        // Login!
        if game_status.eq(&global_constants::KOL_OPEN_STR.to_string()) {
            (login_status, client) = do_login(
                configuration,
                landing_page_data.referer,
                debug_level,
            );

            if debug_level > 2 {
                log::info!("Logged in: {:#?}", login_status);
            }
            // If we're not logged in something else is wrong so
            // stop retrying anyway.
            retry = false;
        }
    }
    (login_status, client)
}

pub fn logout(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8,) -> bool {

    let uri = configuration.get("login", "base_URI").unwrap()
        + global_constants::HTML_DELIMITER
        + &configuration.get("login", "logout_URI").unwrap();

    let raw_data = global_constants::EMPTY_STR.to_string();

    // As we're logging out we don't need the client, as returned
    // from get_kol_data, anymore.
    let (logout_page_data, _) = 
        get_kol_data(client, uri, raw_data, debug_level);

    parse_kol_data::parse_logout_page(
        logout_page_data.body, 
        debug_level)
}

// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
#[tokio::main]
pub async fn get_kol_data(
    client: &mut Client,
    uri: String,
    body_data: String,
    debug_level: u8 ) -> (HttpResult, &mut Client) {

    // For extreme debugging when you need full data.
    //println!("URI in GKD: {}", uri);
    //println!("Client in GKD: {:#?}", client);

    let response = client
        .post(uri)
        .body(body_data)
        .send()
        .await
        .expect("Failed to get response headers.");

    let response_headers = response.headers().clone();
    let response_status = response.status();
    let referer = response.url().clone();
    let referer = referer.to_string();

    let response_body = 
        response.text().await.expect("Failed to get the HTML body.");

    if debug_level > 2 {
        log::info!("Headers: {:#?}", response_headers);
        log::info!("Status {:#?}", response_status);
        log::info!("Referer {:#?}", referer);
        log::info!("Result: {:#?}", response_body);
    }

    // Not all calls need the client returned but it's needed to
    // be passed up to main.rs and then down to the chat library
    // for this to work.
    (
        HttpResult {
            body:       response_body,
            headers:    response_headers,
            status:     response_status,
            referer,
        },
        client,
    )
}

//
// Private functions below here.
//

// Intitialization of the client object.
// See:
// 1. https://en.wikipedia.org/wiki/List_of_HTTP_header_fields,
// 2. https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html
//fn initialize_client() -> Easy {
fn initialize_client(referer: &str) -> reqwest::Client {
    static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let mut headers = reqwest::header::HeaderMap::new();

    // Standard headers, as much as there is a standard.
    headers.append(
        reqwest::header::ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );
    // Using ACCEPT_ENCODING means the body is returned is compressed.
    // These are very small packets so lets be lazy and not accept
    // compression.
    //headers.append(
    //    reqwest::header::ACCEPT_ENCODING,
    //    HeaderValue::from_static(
    //        "gzip, deflate, br"));
    headers.append(
        reqwest::header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.append(
        reqwest::header::CONNECTION,
        HeaderValue::from_static("keep-alive"),
    );
    headers.append(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    // Yes, Referer: may be empty.
    // Not also the use of from_str().
    headers.append(
        reqwest::header::REFERER,
        HeaderValue::from_str(referer).unwrap(),
    );
    // DNT to be replaced by GPC at some point, hopefully.
    headers.append(reqwest::header::DNT, HeaderValue::from_static("1"));
    headers.append(reqwest::header::TE, HeaderValue::from_static("trailers"));
    headers.append(
        reqwest::header::UPGRADE_INSECURE_REQUESTS,
        HeaderValue::from_static("1"),
    );

    // Headers used by Firefox but not in the reqwest crate.
    headers.append("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.append("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.append("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.append("Sec-Fetch-User", HeaderValue::from_static("?1"));

    // Let the server have a unique identifier for this code base.
    headers.append(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("KoLBot_2022"),
    );
    headers.append(
        "x-powered-by",
        HeaderValue::from_static("Arrogance and beer."),
    );

    let client = reqwest::Client::builder()
        //let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .default_headers(headers)
        .no_brotli()
        .user_agent(USER_AGENT)
        .build();

    client.unwrap()
}

// Is the game open, is there even a network connection?
fn check_game_status(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8,
) -> (String, HttpResult) {
    let uri = configuration.get("login", "base_URI").unwrap();

    let raw_data = global_constants::EMPTY_STR.to_string();

    let (landing_page_data, _) = get_kol_data(client, uri, raw_data, debug_level);

    if debug_level > 2 {
        log::info!("URL headers: {:#?}", landing_page_data.headers);
    }

    let game_status =
        parse_kol_data::parse_landing_page(&landing_page_data.body, debug_level);

    if debug_level > 2 {
        log::info!("Game status: {:#?}", game_status);
    }

    (game_status, landing_page_data)
}

// Obviously trying to log into the game.
//fn do_login(configuration: &Ini, kol_strings: &Ini, referer: String, debug_level: u8) -> bool {
fn do_login(
    configuration: &Ini,
    referer: String,
    debug_level: u8,
) -> (bool, Client) {
    let mut client = initialize_client(&referer);

    let uri = configuration.get("login", "base_URI").unwrap()
        + global_constants::HTML_DELIMITER
        + &configuration.get("login", "login_URI").unwrap();

    let account = configuration.get("login", "account").unwrap();
    let password = configuration.get("login", "password").unwrap();

    let raw_data = "loggingin=Yup.&promo=&mrstore=&loginname=".to_owned()
        + account.as_str()
        + "&password="
        + password.as_str()
        + "&secure=0&submitbutton=Log In";

    let (login_page_data, _) = get_kol_data(&mut client, uri, raw_data, debug_level);

    //println!("LPD:\n{}", login_page_data.body);

    (
        parse_kol_data::parse_login_page(login_page_data.body, debug_level),
        client,
    )
}

// How long to sleep for.
// If 19:30 in Arizona the game should be back by 19:35, except on
// Saturdays when it's 19:40.
// If it's outside of these times set it to 2 minutes.
fn get_rollover_sleep_length() -> u16 {
    let sleep_length: u16;

    let game_datetime = Utc::now().with_timezone(&Arizona);
    let game_minutes = game_datetime.minute() as u16;

    // Using unsigned integers so we don't want an underflow error.
    if global_constants::GENERAL_MAINT > game_minutes {
        if game_datetime.weekday().to_string() != "Sun" {
            sleep_length = (global_constants::GENERAL_MAINT - game_minutes) * 60;
        } else {
            sleep_length = (global_constants::SATURDAY_MAINT - game_minutes) * 60;
        }
    } else {
        // Unplanned long maintenance, set to 3 minutes.
        sleep_length = 2 * 60;
    };

    sleep_length
}
