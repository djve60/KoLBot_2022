// Copyright D. Evans (djve60+kolbot@gmail.com).

// Read a file of one-line aphorisms.
// The source data came from https://github.com/shlomif/fortune-mod.
// This was reformated to single lines and anything longer than
// 199 characters removed. KoL chat only allows 200 characters.

// The code/method was copied, plagiarized, lifted, without authority, from
// https://towardsdatascience.com/interview-question-select-a-random-line-from-a-file-in-rust-c0a8cddcddfb

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::thread;

// Crates from rust.io.
use configparser::ini::Ini;
use rand::{Rng, RngCore};
use reqwest::Client;

// Local libraries.
use crate::connection;
use crate::chat::post_generic_message;
use crate::global_constants;
use crate::global_variables::{player_id, pwd_hash};
use crate::macros;
use crate::regular_expressions;


// No return value.
pub fn post_fortune(
    mut chat_channel: &str,
    user_id: u128,
    client: Client,
    configuration: &Ini,
    debug_level: u8) {

    let mut fortune = return_random_fortune(configuration);

    // This must be a private message.
    if chat_channel.is_empty() {
        chat_channel = "clan"; // Need to post somewhere.
        fortune = "/msg ".to_owned() + &user_id.to_string() + " " + &fortune;
    }

    post_generic_message(
        client.clone(),
        &configuration,
        &chat_channel,
        &fortune,
        &*player_id().lock().unwrap(),
        &*pwd_hash().lock().unwrap(),
        debug_level);
}


// This returns the last message value, for network connectivity.
pub fn fortune_network_test(
    chat_last_message_value: &mut u128,
    client: &mut Client,
    configuration: &Ini,
    debug_level: u8) -> u128 {

    let fortune = return_random_fortune(configuration);
    let raw_data = global_constants::EMPTY_STR.to_string();

    let uri = configuration.get("login", "base_URI").unwrap()
            + &format!(
                macros::CHAT_CHANNEL_CHECK_MESSAGES_STRING!(),
                rand::thread_rng().gen::<f32>(),
                chat_last_message_value
            );

    post_generic_message(
        client.clone(),
        &configuration,
        "clan",                     //Yes, hardcoded.
        &fortune,
        &*player_id().lock().unwrap(),
        &*pwd_hash().lock().unwrap(),
        debug_level);

    // Below here is for network connectivity check.
    // Give time for it to be processed.
    thread::sleep(Duration::from_secs(2));

    // Check what is posted.
    let (chat_channel_data, _) =
            connection::get_kol_data(
                client,
                uri,
                raw_data.clone(),
                debug_level);

    let chat_new_message_value =
                regular_expressions::get_chat_last_message_id(
                    chat_channel_data.body).parse::<u128>().unwrap();

    chat_new_message_value
}

/// return_random_fortune(), calls try_iterator_choose_skip(), which
/// calls try_nth().
fn return_random_fortune(
    configuration: &Ini) -> String {

    let path = configuration.get("fortune", "data_file").unwrap();

    let lines = BufReader::new(File::open(&path).unwrap()).lines();
    let mut rng = rand::thread_rng();
    let random_line = 
        try_iterator_choose_skip(lines, &mut rng).unwrap().unwrap();

    random_line
}

fn try_iterator_choose_skip<T, I, R, E>(
    mut iterator: I, rng: &mut R) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
    R: RngCore,
    E: Error,
{
    let mut offset = 1;
    let mut index1 = 1;
    let mut random_item = None;
    while let Ok(Some(item)) = try_nth(&mut iterator, offset - 1) {
        random_item = Some(item);
        let r: f32 = rng.gen();
        offset = ((r * (index1 as f32) / (1.0 - r)).ceil() as usize).max(1);
        index1 += offset;
    }

    Ok(random_item)
}

fn try_nth<I, T, E>(mut iterator: I, n: usize) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    for _ in 0..n {
        if iterator.next().transpose()?.is_none() {
            return Ok(None);
        }
    }
    iterator.next().transpose()
}
