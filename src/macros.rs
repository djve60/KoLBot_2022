// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of global macros for kolbot_2022.

// Need to "use crate::macros".
// Refrenced with "macros::<NAME>".

// Chat channel strings.
//macro_rules! CHAT_CHANNEL_OPEN_STRING {
//    () => {
//        "/submitnewchat.php?playerid={}&pwd={}&graf=/who+{}&j=1"
//    };
//}
//pub(crate) use CHAT_CHANNEL_OPEN_STRING;

macro_rules! CHAT_CHANNEL_CHECK_MESSAGES_STRING {
    () => {
        "/newchatmessages.php?aa={}&j=1&lasttime={}"
    };
}
pub(crate) use CHAT_CHANNEL_CHECK_MESSAGES_STRING;


//graf is channel name and a message.
macro_rules! CHAT_CHANNEL_POST_MESSAGE_STRING {
    () => {
        "submitnewchat.php?playerid={}&pwd={}&graf=%2F{} {}&j=1"
    };
}
pub(crate) use CHAT_CHANNEL_POST_MESSAGE_STRING;

