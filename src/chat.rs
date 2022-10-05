// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of functions to read messages posted to the
// chat channels and act according to directives.

use std::thread;
use std::time::Duration;

// Crates from rust.io.
use configparser::ini::Ini;
use rand::Rng;
use reqwest::Client;
//use signal_hook::low_level::raise;

// Local libraries.
use crate::connection::get_kol_data;
use crate::constants;
use crate::macros;
use crate::parse_kol_data;

pub fn listen_to_chat(
    client: &mut Client,
    configuration: &Ini,
    _kol_strings: &Ini,
    debug_level: u8,
) {
    // player_id and pwd_hash are required when posting messages.
    // Not got that far yet.
    let (_player_id, _pwd_hash, mut chat_last_message_value) =
        open_chat_channels(client, configuration, debug_level);

    let mut random = rand::thread_rng();
    let raw_data = constants::EMPTY_STR.to_string();

    // Yes, loop forever.
    // To be done:
    // When the game's warning of rollover is 1 minute away
    // the message will generate an signal interrupt and the code will
    // close off messages.
    loop {
        // Check every ½sec.
        thread::sleep(Duration::from_millis(
            constants::CHAT_CHANNEL_MSEC_SLEEP_DURATION,
        ));

        let uri = configuration.get("login", "base_URI").unwrap()
            + &format!(
                macros::CHAT_CHANNEL_CHECK_MESSAGES_STRING!(),
                random.gen::<f32>(),
                chat_last_message_value
            );

        //let (chat_channel_data, client) =
        // Don't need the client object returned.
        let (chat_channel_data, _) = get_kol_data(client, uri, raw_data.clone(), debug_level);
        println!("CC Message {:#?}", chat_channel_data.body);

        chat_last_message_value = parse_kol_data::get_chat_message_data(chat_channel_data.body);
        println!("C Last Message {:#?}", chat_last_message_value);

        // Do something here.
        // Are we alive? How to handle network outage?
        // Panic and let the uxix service handle it?
    }

    // On traditional unix/POSIX systems SIGQUIT is 3.
    // raise(3).expect("Did not raise SIGQUIT!");
}

//
// Private functions below here.
//

// It is assumed that in the manual creation of the 'bot account
// that the chat channels are already subscribed to so this code
// just needs to open the chat URL and return the player ID, the
// password hash, and the current "last message value".
fn open_chat_channels(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8,
) -> (String, String, String) {
    let uri = configuration.get("login", "base_URI").unwrap()
        + constants::HTML_DELIMITER
        + &configuration.get("chat", "chat_URI").unwrap();

    let raw_data = constants::EMPTY_STR.to_string();

    let (chat_frame_data, _) = get_kol_data(client, uri, raw_data.clone(), debug_level);

    let (player_id, pwd_hash) = parse_kol_data::get_account_data(chat_frame_data.body.clone());

    // This is in the HTML code before chat is active.
    // Not really needed when running but very handy for debugging.
    let chat_channels = parse_kol_data::get_chat_channels(chat_frame_data.body);

    if debug_level > 1 {
        log::info!(
            "Player ID: {}\nPassword hash: {}\nChannel: {:#?}",
            player_id,
            pwd_hash,
            chat_channels
        );
    }

    // Open chat frame.
    let uri = configuration.get("login", "base_URI").unwrap()
        + &format!(
            macros::CHAT_CHANNEL_OPEN_STRING!(),
            player_id, pwd_hash, "clan"
        );

    //let (chat_channel_data, client) =
    // If this fails there are bigger issues.
    let (_, _) = get_kol_data(client, uri, raw_data.clone(), debug_level);

    let mut random = rand::thread_rng();

    // Send the initial message. This gives the "last message value".
    // Not to be confused with a unix timestamp. That is also in the
    // data but not needed by the client.
    // This also returns zero messages.
    let uri = configuration.get("login", "base_URI").unwrap()
        + &format!(
            macros::CHAT_CHANNEL_CHECK_MESSAGES_STRING!(),
            random.gen::<f32>(),
            0
        );

    let (chat_channel_data, _) = get_kol_data(client, uri, raw_data, debug_level);

    let chat_last_message_value = parse_kol_data::get_chat_message_data(chat_channel_data.body);

    (player_id, pwd_hash, chat_last_message_value)
}
