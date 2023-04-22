// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of functions to read messages posted to the
// chat channels and act according to directives.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::thread;
//use std::process::abort;

// Crates from rust.io.
use configparser::ini::Ini;
use rand::Rng;
use reqwest::Client;

// Local libraries.
use crate::channel_clan;
use crate::connection;
use crate::global_constants;
use crate::fortune;
use crate::global_variables::{player_id, pwd_hash, heartbeat_counter};
use crate::macros;
use crate::message_handling;
use crate::regular_expressions;
use crate::utilities;


pub fn initialize_chat(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8 ) -> u128 {

    // This sets the values for player_id and pwd_hash as
    // global variables.
    let chat_last_message_value: u128 =
        open_chat_channels(
            client,
            configuration,
            debug_level).parse().unwrap();

    // Does NOT need chat_last_message_value.
    post_hello_messages(client, configuration, debug_level);

    chat_last_message_value
}


pub fn listen_to_chat(
    client: &mut Client,
    configuration: &Ini,
    chat_last_message_value: &mut u128,
    shutdown_flag: Arc<AtomicBool>,
    debug_level: u8 ) {

    let mut heartbeat: u32 = *heartbeat_counter().lock().unwrap();
    let mut rng = rand::thread_rng();
    let raw_data = global_constants::EMPTY_STR.to_string();

    // Yes, loop forever.
    // To be done:
    // When the game's warning of rollover is 1 minute away
    // the message will generate an signal interrupt and the code will
    // close off messages.
    while !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {

        // Check every ½sec.
        thread::sleep(Duration::from_millis(
            u64::from(global_constants::CHAT_CHANNEL_MSEC_SLEEP_DURATION)
        ));

        let uri = configuration.get("login", "base_URI").unwrap()
            + &format!(
                macros::CHAT_CHANNEL_CHECK_MESSAGES_STRING!(),
                rng.gen::<f32>(),
                chat_last_message_value
            );

        // Don't need the client object returned.
        let (chat_channel_data, _) =
            connection::get_kol_data(
                client,
                uri,
                raw_data.clone(),
                debug_level);
        println!("CC Message {:#?}",
                 chat_channel_data.body.to_string() );

        // If no new messages then body may be empty.
        // This happens on logging on, as an example.
        if ! chat_channel_data.body.is_empty() {

            let mut chat_new_message_value =
                regular_expressions::get_chat_last_message_id(
                    chat_channel_data.body.clone() ).parse::<u128>().unwrap();

            if chat_last_message_value < &mut chat_new_message_value {
                // Chat's working. Reset the heatbeat counter.
                *heartbeat_counter().lock().unwrap() =
                    global_constants::CHAT_CHANNEL_HEARTBEAT_COUNTDOWN;

                *chat_last_message_value = chat_new_message_value;
            }

            // No, we're not worrying about joing when it's finished.
            // We want the code to continue running and handle new 
            // messages asynchronously.
            let client_thread = client.clone();
            let configuration_thread = configuration.clone();
            thread::spawn(move || {
                // Hand messages off to message handler code.
                message_handling::handle_messages(
                    chat_channel_data.body.clone(),
                    client_thread,
                    &configuration_thread,
                    debug_level.clone());
            });
        }

        // Decrement heatbeat counter.
        // No, you can't directly do "X = X - 1" for global variables when
        // treadng is supported. Alas.
        if heartbeat > 0 {
            *heartbeat_counter().lock().unwrap() = heartbeat - 1;
        }

        // If needed check connection.
        if  heartbeat < 1 {

            // This will logout and log back in if needed.
            *chat_last_message_value =
                check_heatbeat(
                    chat_last_message_value,
                    client,
                    configuration,
                    debug_level);

            // The global is updated in check_heatbeat.
            heartbeat = *heartbeat_counter().lock().unwrap();

        }
    }

    // On traditional unix/POSIX systems SIGQUIT is 3.
    // raise(3).expect("Did not raise SIGQUIT!");
}


pub fn post_generic_message(
    client: Client,
    configuration: &Ini,
    channel: &str,
    message: &str,
    player_id: &str,
    pwd_hash: &str,
    debug_level: u8 ) {

    let channel = channel.to_owned();
    let configuration = configuration.to_owned();
    let message = message.to_string().to_owned();
    let player_id = player_id.to_owned();
    let pwd_hash = pwd_hash.to_owned();

    // No, we're not worrying about joing when it's finished.
    // We want the code to continue running.
    thread::spawn(move || {

        let raw_data = global_constants::EMPTY_STR.to_string();
        let message = utilities::kolinze_html_parameters(message);

        let mut uri = format!(
            macros::CHAT_CHANNEL_POST_MESSAGE_STRING!(),
            player_id,
            pwd_hash,
            channel,
            message);

        uri = configuration.get("login", "base_URI").unwrap()
            + global_constants::HTML_DELIMITER
            + &uri;

        if debug_level > 1 {
            log::info!("URI: {}", uri);
        }

        let (_chat_channel_data, _) =
            connection::get_kol_data(
                &mut client.clone(),
                uri,
                raw_data,
                debug_level);
        //println!("Out: {:#?}", _chat_channel_data.body);
    });
}


pub fn post_farwell_messages(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8 ) {

    let chat_channels = configuration.get("chat", "listen_to").unwrap();

    for channel in chat_channels.split_whitespace() {

        // We don't really care if these succeed or not so there is
        // no JoinHandle or "move".
        // See https://doc.rust-lang.org/book/ch16-01-threads.html.
        match channel {
            "clan" =>
                post_generic_message(
                    client.clone(),
                    &configuration,
                    channel,
                    &configuration.get(&channel, "goodbye_msg").unwrap(),
                    &*player_id().lock().unwrap(),
                    &*pwd_hash().lock().unwrap(),
                    debug_level),
            "dreadsylvania" =>
                post_generic_message(
                    client.clone(),
                    &configuration,
                    channel,
                    &configuration.get(&channel, "goodbye_msg").unwrap(),
                    &*player_id().lock().unwrap(),
                    &*pwd_hash().lock().unwrap(),
                    debug_level),
            //"hobopolis" => (),
            //"slimetube" => (),
            //"talkie" => (),
            _ => ()
        }
    }
}

pub fn post_reconnect_messages(
    client: &mut Client,
    configuration: &Ini,
    reason: String,
    debug_level: u8 ) {

    let chat_channels = configuration.get("chat", "listen_to").unwrap();
    let message = "/me reconnected, reason is ".to_owned() + &reason;

    for channel in chat_channels.split_whitespace() {

        // We don't really care if these succeed or not so there is
        // no JoinHandle or "move".
        // See https://doc.rust-lang.org/book/ch16-01-threads.html.
        match channel {
            "clan" =>
                post_generic_message(
                    client.clone(),
                    &configuration,
                    channel,
                    &message,
                    &*player_id().lock().unwrap(),
                    &*pwd_hash().lock().unwrap(),
                    debug_level),
            //"dreadsylvania" => (),
            //"hobopolis" => (),
            //"slimetube" => (),
            //"talkie" => (),
            _ => ()
        }
    }
}

//
// Private functions below here.
//

// This check that the channels listed in the configuration
// are all being listened to. Extra channels are ignored.
fn check_listening_channels(
    client: &mut Client,
    mut live_channels: Vec<std::string::String>,
    configuration: &Ini,
    debug_level: u8) -> Vec<std::string::String> {

    let configuraton_channels = 
        configuration.get("chat", "listen_to").unwrap();

    for channel in configuraton_channels.split_whitespace() {
        if ! live_channels.contains(&channel.to_string()) {

            let open_channel_message = "/listen ".to_owned() + channel;

            post_generic_message(
                client.clone(),
                &configuration,
                "clan",
                &open_channel_message,
                &*player_id().lock().unwrap(),
                &*pwd_hash().lock().unwrap(),
                debug_level);

            live_channels.push(channel.to_string());
        }
    }

    live_channels
}

// This compares last message value after checking for a response
// after sending a "fortune".
// Can't use the chat command "/rollover", etc, as it does not update
// the message count.
// Depending on the result we try to exit and restart chat or
// exit the program and let the OS daemon handler do the restart.
fn check_heatbeat(
    chat_last_message_value: &mut u128,
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8) -> u128 {

    let mut chat_working: bool = false;

    // Is it just quiet? Make a post to check, and reset counter if
    // we get a response back.
    let mut chat_new_message_value: u128 =
        fortune::fortune_network_test(
            chat_last_message_value,
            client,
            configuration,
            debug_level);

    if chat_last_message_value < &mut chat_new_message_value {
        // Chat's working. Reset the heatbeat counter.
        *heartbeat_counter().lock().unwrap() =
            global_constants::CHAT_CHANNEL_HEARTBEAT_COUNTDOWN;
        *chat_last_message_value = chat_new_message_value;
        chat_working = true;
    }

    if chat_working == false {
        // Close chat and renew credentials, hopefully.
        (*chat_last_message_value, *client) =
            renew_credentials(
                client,
                configuration,
                debug_level);

        // Logout and log back in to refersh the network connection.
        let message =
            "Chat was unresponsive, credetials renewed.".to_string();

        log::info!("{}", message);
        post_reconnect_messages(
            client,
            configuration,
            message,
            debug_level );
    }

    *chat_last_message_value
}


// It is assumed that in the manual creation of the 'bot account
// that the chat channels are already subscribed to so this code
// just needs to open the chat URL and return the player ID, the
// password hash, and the current "last message value".
fn open_chat_channels(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8) -> String {

    let uri = configuration.get("login", "base_URI").unwrap()
        + global_constants::HTML_DELIMITER
        + &configuration.get("chat", "chat_URI").unwrap();

    let raw_data = global_constants::EMPTY_STR.to_string();

    let (chat_frame_data, _) =
        connection::get_kol_data(
            client,
            uri,
            raw_data.clone(),
            debug_level);

    // Extract the ID number and password hash from the HTML data.
    let (current_id, current_pwd_hash) =
        regular_expressions::get_account_id_and_pwdhash(
            &chat_frame_data.body);
    *player_id().lock().unwrap() = current_id.to_string();
    *pwd_hash().lock().unwrap() = current_pwd_hash.to_string();

    // This is in the HTML code before chat is active.
    // Not really needed when running but very handy for debugging.
    let mut chat_channels =
        regular_expressions::get_chat_channels(chat_frame_data.body);

    chat_channels =
        check_listening_channels(
            client,
            chat_channels,
            configuration,
            debug_level);

    if debug_level > 1 {
        log::info!(
            "\n Player ID: {}\n Password hash: {}\n Channel: {:#?}",
            *player_id().lock().unwrap(),
            *pwd_hash().lock().unwrap(),
            chat_channels
        );
    }

    // For some reason KoL wants a random number.
    let mut rng = rand::thread_rng();

    // Send the initial message. This gives the "last message value".
    // Not to be confused with a unix timestamp. That is also in the
    // data but not needed by the client.
    // This also returns zero messages.
    let uri = configuration.get("login", "base_URI").unwrap()
        + &format!(
            macros::CHAT_CHANNEL_CHECK_MESSAGES_STRING!(),
            rng.gen::<f32>(),
            0
        );
    let (chat_channel_data, _) =
        connection::get_kol_data(
            client,
            uri,
            raw_data,
            debug_level);
    let last_message_value =
        regular_expressions::get_chat_last_message_id(chat_channel_data.body);

    last_message_value
}

fn post_hello_messages(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8 ) {

    let chat_channels = configuration.get("chat", "listen_to").unwrap();

    for channel in chat_channels.split_whitespace() {

        // We don't really care if these succeed or not so there is
        // no JoinHandle or "move".
        // See https://doc.rust-lang.org/book/ch16-01-threads.html.
        match channel {
            //"clan" => return,
            "clan" =>
                channel_clan::post_hello(
                    client.clone(),
                    &configuration,
                    channel,
                    &*player_id().lock().unwrap(),
                    &*pwd_hash().lock().unwrap(),
                    debug_level),
            //"dreadsylvania" => (),
            //"hobopolis" => (),
            //"slimetube" => (),
            //"talkie" => (),
            //"villa" => () // Will relay villa back to talkie, only.
            _ => ()
        }
    }
}

// Things are totally messed up at this point. Probably a stale file handle.
// Renew the whole session.
fn renew_credentials(
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8) -> (u128, Client) {

    // Attempt to logout. This may not work if there is a stale file handle
    // but let's try and be nice.

    let logout_result =
        connection::logout(
            client,
            &configuration,
            debug_level);

    log::info!("Chat not updating.\n Logging out: {:?}", logout_result);

    let (_continue_running, mut client) =
        connection::login(
            &configuration,
            debug_level);

    let chat_last_message_value: u128 =
        initialize_chat(
            &mut client,
            &configuration,
            debug_level );

    thread::sleep(Duration::from_secs(2));

    (chat_last_message_value, client)
}
