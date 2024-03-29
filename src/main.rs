// Copyright D. Evans (djve60+kolbot@gmail.com).

// This is the main body of the kol_robot code, a program to
// allow an account to perform automated actions in the online game
// https://KingdomOfLoathing.com.

// See lib.rs for local modules.

// https://stackoverflow.com/questions/35559267/suppress-panic-output-in-rust-when-using-paniccatch-unwind
//use std::panic;
//use std::process;
//use std::error::Error;
//use std::sync::atomic::Ordering;
use std::{thread, time::Duration};


// This has to be hardcoded as this is needed before code can use
// "crate".
use kolbot_2022::global_constants;
use kolbot_2022::global_variables::pwd_hash;
//use kolbot_2022::globals::{player_id, pwd_hash};
//use crate::logging;
//use kolbot_2022::logging;


// Since this returns Ok(()) the body needs to return a Result.
fn main() -> Result<(),()> {

    // Handle signals here.
    let shutdown_flag = 
        kolbot_2022::signals::create_signal_watcher();

    // Let's get the settings to use.
    // Mainly hardwired so any issues we want to panic and end.
    let (configuration, debug_level) = 
        kolbot_2022::initialization::start();

    // So we can keep important messages using "log::<level>!".
    let logger = 
        kolbot_2022::logging::initialize_logging(
            &configuration, 
            global_constants::EMPTY_STR);

    // Login (or attmpt to).
    let (continue_running, mut client) =
        kolbot_2022::connection::login(
            &configuration, 
            debug_level);

    log::info!(
        "Logged in as {}",
        configuration.get("login", "account").unwrap()
    );

    let mut chat_last_message_value: u128 =
        kolbot_2022::chat::initialize_chat(
            &mut client,
            &configuration, 
            debug_level);
        
    // To be clear:
    // 1. this will only handle SIGINT, SIGQUIT, and SIGTERM.
    while continue_running && 
          !shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {

        // Do breakfast tasks, asyncronously.
        // thread::spawn(|| {
        //  Do breakfast and print results to log.
        // }

        // Check for messages.
        // This process does all the work and is driven by
        // posts to the Chat channel.
        // Do I want this to return or just raise a signal?
        kolbot_2022::chat::listen_to_chat(
            &mut client,
            &configuration, 
            &mut chat_last_message_value,
            shutdown_flag.clone(), 
            debug_level);
    }

    log::info!("Caught signal, shuting down.");
    println!("Caught signal, shuting down.");

    println!("pwd: Continue running {}", *pwd_hash().lock().unwrap());

    // Clean-up and send good-bye messages.
    // clean_up()

    kolbot_2022::chat::post_farwell_messages(
        &mut client,
        &configuration, 
        debug_level);

    // Give time for the async post to be sent.
    thread::sleep(Duration::from_secs(1));

    //println!("2: Continue running {}", continue_running);

    // Clean logout, hopefully.
    let logout_result =
        kolbot_2022::connection::logout(
            &mut client, 
            &configuration, 
            debug_level);

    log::info!("Logged out: {:?}", logout_result);

    // Shut down logging.
    //crate::logging::shutdown_logging(logger);
    kolbot_2022::logging::shutdown_logging(logger);

    Ok(())
}
