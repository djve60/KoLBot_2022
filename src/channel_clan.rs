// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of functions for the chat channel "clan".

use std::thread;

// Crates from rust.io.
use configparser::ini::Ini;
use reqwest::Client;

// Local libraries.
use crate::connection::get_kol_data;
use crate::global_constants;
use crate::macros;
use crate::utilities;

pub fn post_hello(
    client: Client,
    configuration: &Ini,
    channel: &str,
    player_id: &str,
    pwd_hash: &str,
    debug_level: u8 ) {

    let channel = channel.to_owned();
    let configuration = configuration.to_owned();
    let player_id = player_id.to_owned();
    let pwd_hash = pwd_hash.to_owned();

    thread::spawn(move || {

        let raw_data = global_constants::EMPTY_STR.to_string();

        let uri =
            configuration.get(&channel, "hello_msg").unwrap()
            + " "
            + &configuration.get("command", "command_prefix").unwrap();
        let uri = utilities::kolinze_html_parameters(uri);

        let uri = &format!(
            macros::CHAT_CHANNEL_POST_MESSAGE_STRING!(),
            player_id,
            pwd_hash,
            channel,
            uri);

        let uri = configuration.get("login", "base_URI").unwrap()
            + global_constants::HTML_DELIMITER
            + uri;

        if debug_level > 2 {
            log::info!("URI: {}", uri);
        }

        let (_chat_channel_data, _) = 
            get_kol_data(
                &mut client.clone(), 
                uri, 
                raw_data, 
                debug_level);
    });
}
