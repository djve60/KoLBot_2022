// Copyright D. Evans (djve60+kolbot@gmail.com).

// Return the clan rule for a given non-combat adventure in basement area
// for a clan, in the game Kingdom of Loathing.


// use std::error::Error;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::time::Duration;
// use std::thread;

// Crates from rust.io.
use configparser::ini::Ini;
use reqwest::Client;

// Local libraries.
use crate::chat::post_generic_message;
use crate::global_variables::{player_id, pwd_hash};
use crate::rules_constants;


// No return value.
pub fn send_rule(
    request: &str,
    mut chat_channel: &str,
    user_id: u128,
    client: Client,
    configuration: &Ini,
    debug_level: u8) {

    let mut rule = validate_rule(request, chat_channel);

    // This must be a private message.
    if chat_channel.is_empty() {
        chat_channel = "clan"; // Need to post somewhere.
        rule = "/msg ".to_owned() + &user_id.to_string() + " " + &rule;
    }

    post_generic_message(
        client.clone(),
        &configuration,
        &chat_channel,
        &rule,
        &*player_id().lock().unwrap(),
        &*pwd_hash().lock().unwrap(),
        debug_level);
}


fn validate_rule(
    mut request: &str,
    channel: &str ) -> String {

    // Remove the keyword so just the requested rule is left.
    match request.split_once(' ') {
        Some((_, value)) => {
            request = value;
        }
        None => { 
            request = "";
        //None => { ();
        }
    }

    find_matching_rule(request, channel)
}


fn find_matching_rule(
    request: &str,
    channel: &str ) -> String {

    let mut count: u8 = 0;
    let mut final_rule: String = "".to_string();

    if channel.is_empty() {
        // Private message.
        for (nc_name, rule) in rules_constants::ALL_RULES.iter() {
            let lowercase_name = nc_name.to_lowercase();
            if lowercase_name.starts_with(&request.to_lowercase()) {
            //if nc_name.to_lowercase().starts_with(request.to_lowercase()) {
                count = count + 1;
                final_rule = rule.to_string();
            }
        }
    } else if channel.contains("slimetube") {
        for nc_name in rules_constants::SLIMETUBE_RULES.iter() {
            //if nc_name.starts_with(request) {
            let lowercase_name = nc_name.to_lowercase();
            if lowercase_name.starts_with(&request.to_lowercase()) {
            //if nc_name.to_lowercase().starts_with(request.to_lowercase()) {
                count = count + 1;
                final_rule = rules_constants::ALL_RULES.get(nc_name)
                    .unwrap()
                    .to_string();
            }
        }
    } else if channel.contains("hobopolis") {
        for nc_name in rules_constants::HOBOPOLIS_RULES.iter() {
            let lowercase_name = nc_name.to_lowercase();
            if lowercase_name.starts_with(&request.to_lowercase()) {
            //if nc_name.starts_with(request) {
            //if nc_name.to_lowercase().starts_with(request.to_lowercase()) {
                count = count + 1;
                final_rule = rules_constants::ALL_RULES.get(nc_name)
                    .unwrap()
                    .to_string();
            }
        }
    } else if channel.contains("dread") {
        for nc_name in rules_constants::DREADSYLVANIA_RULES.iter() {
            let lowercase_name = nc_name.to_lowercase();
            if lowercase_name.starts_with(&request.to_lowercase()) {
            //if nc_name.starts_with(request) {
            //if nc_name.to_lowercase().starts_with(request.to_lowercase()) {
                count = count + 1;
                final_rule = rules_constants::ALL_RULES.get(nc_name)
                    .unwrap()
                    .to_string();
            }
        }
    }

    if count < 1 {
        final_rule = 
            "Sorry, there is no match for your request.".to_string();
    } else if count > 1 {
        final_rule = 
            "Sorry, there are ".to_owned() + 
            &count.to_string() + 
            &" matches for your request. I don't know which one you want."
            .to_string();
    }

    // What if they just sent "rule" as they forgot? Print the help message.
    if request.is_empty() {
        final_rule = rules_constants::ALL_RULES.get("Help")
            .unwrap()
            .to_string();
    }
    
    final_rule
}
