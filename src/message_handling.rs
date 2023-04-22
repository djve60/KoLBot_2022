// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of functions to handle KoL chat messages.

use std::collections::HashMap;
//use std::process::abort;

// Crates from rust.io.
use configparser::ini::Ini;
use reqwest::Client;

// Local libraries.
use crate::global_constants;
use crate::fortune;
use crate::regular_expressions;
use crate::rules;


// This is the "value" for the messages, where the key is the message id.
#[derive(Clone)]
#[derive(Debug)]
pub struct MessageDetails {
    pub msg: String,
    pub status: String,     // This "type", and normally is "public".
    pub user_name: String,  // Who posted, the player name.
    pub user_id: u128,      // Who posted, the player ID number.
    pub format: u16,
    pub channel: String,
}

// Top level function for handling the chat messages.
// This is running in a separate thread, at this point.
pub fn handle_messages (
    chat_message_payload: String,
    client: Client,
    configuration: &Ini,
    debug_level: u8) {

    let empty_message: bool = 
        regular_expressions::empty_message_test(&chat_message_payload);

    let command_prefix: &str = 
        &configuration.get("command", "command_prefix").unwrap();

    // Only continue this thread if there is something to do.
    if ! empty_message {

        // Split the individual messages into a vector, for iteration.
        let mut message_details =
            identify_messages(
                configuration,
                chat_message_payload);

        // Reformat messages to remove HTML, decorators, etc.
        // This includes replacing the HTML escaped characters,
        // such as "&gt;" with "<".
        // Note that a loop uses an "immutable" reference. That means
        // a copy is needed for the loop so the original data, outside 
        // of the loop can be updated.
        let immutable_message_details = message_details.clone();
        for (message_id, message_parts) in &immutable_message_details {

            log::warn!("Message ID: {}", message_id);

            let cleaned_message: String =
                clean_up_message_formats(
                    message_parts.msg.clone());

            // If the message has been "cleaned-up" replace the original
            // orginal post with the clean version.
            if message_parts.msg.ne( &cleaned_message ) {

                if let Some(parts) = message_details.get_mut(&message_id) {
                    parts.msg = cleaned_message;
                }
            }

            // Find messages with actionable items, and dispatch them
            // as appropriate.
            action_or_discard_message(
                client.clone(),
                command_prefix,
                configuration,
                debug_level,
                message_parts.clone());
        }
    }
}


// Yes, this code is in it's own thread, so no need to "sub-thread", that 
// I can see, at this point.
fn action_or_discard_message( 
    client: Client,
    command_prefix: &str,
    configuration: &Ini,
    debug_level: u8,
    mut message_parts: MessageDetails) {

    println!("Message1 {}", message_parts.msg);

    // For direct commands from chat.
    if message_parts.msg.starts_with(command_prefix) {

        // Strip off the bot's command prefix, obviously.
        message_parts.msg = 
            message_parts.msg.strip_prefix(command_prefix).unwrap().to_string();

        // Need the first word for cases like "rule <adventure name>".
        let command =  
            regular_expressions::get_first_word(&message_parts.msg);

        println!("Command1: {}", command);

        // Not using "match" as it needs the full string.
        // Lets accept a unique substring, from the start.
        if command.starts_with(
            global_constants::COMMAND_SUBSTRINGS.get("fortune").unwrap() ) {

            fortune::post_fortune(
                &message_parts.channel,
                message_parts.user_id,
                client,
                configuration,
                debug_level);

        } else if command.starts_with(
            global_constants::COMMAND_SUBSTRINGS.get("rules").unwrap() ) {

            rules::send_rule(
                &message_parts.msg,
                &message_parts.channel,
                message_parts.user_id,
                client,
                configuration,
                debug_level);

        //    _ => ()
        //} else {
        // error message.
        }
    }
    // For implicit commands, from events.
}

// Split out the message details, clean-up message content, and return
// a HashMap, using the message id as the key, with the values being 
// a MessageDetails struct.
fn identify_messages (
    configuration: &Ini,
    message_payload: String ) -> HashMap<u128, MessageDetails> {

    println!("In identify_messages\n{:?}", message_payload);
    let mut message_list: HashMap< u128, MessageDetails> = HashMap::new();

    let mut forged_message_id: u128 = 0;
    let channel_list = configuration.get("chat", "listen_to").unwrap();

    // The data is JSON format by the time it gets to this point.
    // However we don't need the JSON structure, or the overhead of 
    // another create, so the data is being extracted by regular 
    // expressions.
    for message_parts in 
        regular_expressions::get_public_messages_details(&message_payload) {

        // For interactive debugging:
        // println!("Full chat mess: {}", message_parts[0].to_string());
        // println!("msg: {}", message_parts[1].to_string());
        // println!("chn: {}", message_parts[7].to_string());

        // For relay between channels this will need to be expanded
        // but when running multiple bots, most will not be relaying
        // messages between channels.
        // This is for messages from players. See the next section
        // for event notification messages.

        // If the channel is in list of public channels to listen to:
        // save the message.
        if channel_list.contains(&message_parts[7].to_string()) {

            message_list.insert(
                message_parts[3].parse::<u128>().unwrap(),  // Message id. 
                MessageDetails {
                    msg:        message_parts[1].to_string(),
                    status:     message_parts[2].to_string(),
                    user_name:  message_parts[4].to_string(),
                    user_id:    message_parts[5].parse::<u128>().unwrap(),
                    format:     message_parts[6].parse::<u16>().unwrap(),
                    channel:    message_parts[7].to_string(),
                }
            );
        }
    }

    // This is for private messages.
    for message_parts in 
        regular_expressions::get_private_messages_details(&message_payload) {

        if message_parts[1].to_string().contains("private") {

            // For interactive debugging:
            // println!("In private mess: {}", message_parts[0].to_string());
            // println!("msg {}", message_parts[2].to_string());
            // println!("Type {}", message_parts[1].to_string());
            // println!("Name {}", message_parts[3].to_string());

            message_list.insert(
                // Yes, need to forge number, private messages don't have a 
                // message id.
                forged_message_id,
                MessageDetails {
                    msg:        message_parts[2].to_string(),
                    status:     message_parts[1].to_string(),
                    user_name:  message_parts[3].to_string(),
                    user_id:    message_parts[4].parse::<u128>().unwrap(),
                    format:     0,
                    channel:    "".to_string(),
                }
            );
            forged_message_id = forged_message_id + 1;
        }
    }

    // This is for event messages.
    for message_parts in 
        regular_expressions::get_event_messages_details(&message_payload) {

        println!("In event mess: {}", message_parts[1].to_string());

        // If the link has "false", it's informational only, from an
        // event type message... then drop the message.
        // Also don't care about "idling" messages.
        if (! global_constants::JSON_FALSE_STR.
                to_string().to_lowercase().eq(
                    &message_parts[3].to_string().to_lowercase() ) ) ||
           (! message_parts[0].to_string().to_lowercase().contains(
                &global_constants::CHAT_AWAY_MESSAGE_STR.to_string().to_lowercase() ) )
        {
            // For interactive debugging:
            // println!("In event mess: {}", message_parts[0].to_string());
            // println!("Type {}", message_parts[1].to_string());

            message_list.insert(
                // Yes, need to forge number, events don't have a message id.
                forged_message_id,
                MessageDetails {
                    msg:        message_parts[5].to_string(),
                    status:     message_parts[1].to_string(),
                    user_name:  message_parts[4].to_string(),
                    user_id:    message_parts[3].parse::<u128>().unwrap(),
                    format:     0,
                    channel:    "".to_string(),
                }
            );
            forged_message_id = forged_message_id + 1;
        }
    }
    message_list
}


// There are a lot of chat effects that can make parsing a command, or 
// reposting difficult. This is a clean-up function on the posted message.
// See https://kol.coldfront.net/thekolwiki/index.php/Chat_Guide:_Miscellaneous
// Do not need to all effects, just ones that stop commands being understood
// because of effects that can be removed.
fn clean_up_message_formats (
    mut message: String) -> String {

    log::warn!("Original msg:\n{}**", message);

    // Can not, or will not, fix:
    //  1. B-b-brr!: changed at the server. While "*brr!*" can be removed
    //     the stuttering would need a LOT of examples to properly 
    //     correct the text.
    //  2. Bruised Jaw: changed at the server.
    //  3. Crown of Ed the Undying: Only affects the player and a
    //     bot is unlikely to have this ItoM equipped.
    //  4. Gothy: Not worth effort, not reliable and shouldn't affect
    //     commands. May change my mind later on.
    //  5. It's Ridiculous: Needs using a can of V-11 and that's rare.
    //  6. Puppet Strings: Has no effect on commands.
    //  7. Sword behind inappropriate prepositions: changed at the server.
    //  8. Staph of homophones:prepositions: changed at the server.
    //  9. SuperStar: Gemelli has not logged in since 2021.
    // 10. The Gray Plauge: No longer a valid effect.
    // 11. Totes: Only in PvP, therefore not relevant.
    // 12. Wanged: If will not affect commands, can be relayed.

    // Will fix if there is a source for the effect (but may already 
    // be fixed by one of the existing functions.:
    // 1. Best Effect Ever: no access to Best Joke Ever.
    // 2. There is No Spoon: the Atomic Comic is too expensive.

    // To be fixed later, if needed, in the message relay funcitons.
    // These effects are appeneded to messages, so they are not 
    // germane to processing commands.
    // 1. Canadianity. ",eh?$", and changes "out" to "oot".
    //    This should have no effect on commands to the bot.
    // 2. Drunkeness. "-hic-$"
    // 3. Jazz Hands. "<i>~jazz hands~<\\/i>$"
    // 4. Liar's pants: "<i><liar's pants message></i>$".
    // 5. Origami riding crop: "<i><origami riding crop message></i>$".
    // 6. Talk Like a Pirate "<i><Ahoy, Matey!|Arr.|Avast ye!|Yarr.></i>$".
    // 7. Tinnitus: "-rimshot-$"
    // 8. Unmotivated: "<i><Meh.|Sigh.|Whatever.></i>$".

    // Also, at this stage, not worrying about:
    // 1. HTML->symbol re-map (i.e. when reposting: turn "&gt;" to "<")
    //    as this is only needed when relaying messages, too.

    // Not fully debugged. It may be working with the currrent code.
    // 1. Purple Prose: If someone has it active I'll continue working 
    //    on this.

    // Some messages "stack" with each other. An example Pirate Bellow will
    // wrap Holiday Fun, and vice versa. 

    // Using bare "if" statments as chat has changed in the past and will
    // probably change in the future, meaning "else if" may be incorrect.


    // Yes, these two effects play nice with each other.
    // This needs to be first, to strip off the font overrides, and 
    // remove the images.
    if ( message.contains(
            &global_constants::CHAT_LOV_EMOTIONIZER_TEST_STR.to_string()) ) ||
       ( message.contains(
            &global_constants::CHAT_PIRATE_BELLOW_TEST_STR.to_string()) )
    {
        message =
            regular_expressions::remove_end_decorators_effect(&message);        
        log::warn!("RED: {}", message);
    }

    // On Safari effect uses this code too.
    if message.contains(
        &global_constants::CHAT_CARTOON_HARPOON_TEST_STR.to_string() ) 
    {
        message =
            regular_expressions::remove_cartoon_harpoon_effect(&message);
        log::warn!("CH {}**", message);
    }

    if message.contains(
        &global_constants::CHAT_GOLDEN_GUM_TEST_STR.to_string() ) 
    {
        message =
            regular_expressions::remove_golden_gum_effect(&message);
        log::warn!("GG: {}", message);
    }

    if message.contains(
             &global_constants::CHAT_HOLIDAY_FUN_TEST_STR.to_string() )
    {
        message =
            regular_expressions::remove_holiday_fun_effect(&message);
        log::warn!("HF: {}", message);
    }

    if message.contains(
             &global_constants::CHAT_EMOTION_SICKNESS_TEST_STR.to_string() )
    {
        message =
            regular_expressions::remove_italics_effect(&message);
        log::warn!("ES: {}", message);
    }

    if message.contains(
        &global_constants::CHAT_LOLSIPOP_TEST_STR.to_string())
    {
        message =
            regular_expressions::remove_lolsipop_effect(&message);
        log::warn!("LP: {}", message);
    }

    if message.contains(
             &global_constants::CHAT_RED_PILL_TEST_STR.to_string()) 
    {
        message =
            regular_expressions::remove_red_pill_effect(&message);
        log::warn!("RP: {}", message);
    }

    if ( message.contains(
             &global_constants::CHAT_PURLE_PROSE_TEST_STR.to_string()) ) ||
       ( message.contains(
             &global_constants::CHAT_VAMPYRIC_CLOAKE_TEST_STR.to_string()) ) ||
       ( message.contains(
             &global_constants::CHAT_VIVALA_MASK_TEST_STR.to_string()) ) 
    {
        message =
            regular_expressions::remove_vampyric_cloake_effect(&message);
        log::warn!("VC: {}", message);
    }

    let mut found_spanish_char: bool = false;
    for (k, _v) in global_constants::SPANISH_LETTERS.iter() {

        if message.contains(k) && (! found_spanish_char) {
            found_spanish_char = true;
            message =
                regular_expressions::remove_eso_si_que_es_effect(&message);
            log::warn!("SC: {}", message);
        }
    }

    // Replace HTML escaped characters as the very last thing. 
    // Needed for relaying in the "talkie" channel.

    message
}
