// Copyright D. Evans (djve60+kolbot@gmail.com).

// Yes, global variables in rust.
// Messy but doesn't need exernal dependencies

use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

use crate::constants;


// The Once:new() can be overloaded for different types but it
// can't be reused for the same type multiple times.
// This means a single INIT can be used for u32 and a String but
// not 2 String types.
// So for readability I've make a lock counter for each function,
// and it also works for functional isolationism.


// This is a countdown. If it reaches zero check the connection.
// This should be reset when the connection to the game is good.
static HEARTBEAT_INIT: Once = Once::new();
static mut STD_ONCE_HEARTBEAT_LOCK: Option<Mutex<u32>> = None;
pub fn heartbeat_counter<'a>() -> &'a Mutex<u32> {
    HEARTBEAT_INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe.
        unsafe {
            *STD_ONCE_HEARTBEAT_LOCK.borrow_mut() = 
                Some(Mutex::new(
                    constants::CHAT_CHANNEL_HEARTBEAT_COUNTDOWN));
        }
    });
    // As long as this function is the only place with access to the 
    // static variable, giving out read-only borrow here is safe 
    // because it is guaranteed no more mutable references will exist 
    // at this point or in the future.
    unsafe { STD_ONCE_HEARTBEAT_LOCK.as_ref().unwrap() }
}


static PLAYER_ID_INIT: Once = Once::new();
static mut STD_ONCE_PLAYER_ID_LOCK: Option<Mutex<String>> = None;
pub fn player_id<'a>() -> &'a Mutex<String> {
    PLAYER_ID_INIT.call_once(|| {
        unsafe {
            *STD_ONCE_PLAYER_ID_LOCK.borrow_mut() = 
                Some(Mutex::new(String::new()));
        }
    });
    unsafe { STD_ONCE_PLAYER_ID_LOCK.as_ref().unwrap() }
}


static PWD_HASH_INIT: Once = Once::new();
static mut STD_ONCE_PWD_HASH_LOCK: Option<Mutex<String>> = None;
pub fn pwd_hash<'a>() -> &'a Mutex<String> {
    PWD_HASH_INIT.call_once(|| {
        unsafe {
            *STD_ONCE_PWD_HASH_LOCK.borrow_mut() = 
                Some(Mutex::new(String::new()));
        }
    });
    unsafe { STD_ONCE_PWD_HASH_LOCK.as_ref().unwrap() }
}
