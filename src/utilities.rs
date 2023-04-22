// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of simple functions for kolbot_2022.

// OS independent path manipulaton.
use std::path::PathBuf;

/// get_path_from_array takes an array, iterates over it's contencts,
/// builds an OS path, and returns the result as a string.
/// Calling this with an empty array returns an empty string.
pub fn get_path_from_array(path_array: &[&str]) -> String {
    let path: PathBuf = path_array.iter().collect();
    //println!("Path from array: {:?}", path);

    let path_string: String = path.to_str().unwrap_or("").to_string();
    // println!("Path string: {}", path_string);

    path_string
}

/// KoL requires all posts to a chat channel use the "+" instead of a
/// space. And only encodes some characters. Seesh!
pub fn kolinze_html_parameters(mut parameter_string: String) -> String {

    // Replace "#" with "%23"
    parameter_string = parameter_string.replace("#","%23");

    // Replace "&" with "%26"
    parameter_string = parameter_string.replace("&","%26");

    // Replace "+" with "%2B"
    parameter_string = parameter_string.replace("+","%2B");

    // Replace ":" with "%3A".
    parameter_string = parameter_string.replace("?","%3A");

    // Replace "=" with "%3D".
    parameter_string = parameter_string.replace("=","%3D");

    // Replace "?" with "%3F".
    parameter_string = parameter_string.replace("?","%3F");

    // Replace "@" with "%40".
    parameter_string = parameter_string.replace("@","%40");

    // Finally replace " " with "+".
    parameter_string.replace(" ","+")
}

/// KoL rollover timestamps are a countdown timer. It is expected that
/// the first set of parameters are the earlier values. This means it 
/// result in the larger value.
/// Anything else results in zero being returned. Time does not run in reverse.
pub fn rollover_time_difference_in_seconds(
    hrs_earlier: u32,
    mins_earlier: u32,
    secs_earlier: u32,
    hrs_later: u32,
    mins_later: u32,
    secs_later: u32) -> u32 {

    let earlier_time = 
        (hrs_earlier * 3600) + (mins_earlier * 60) + secs_earlier;

    let later_time = 
        (hrs_later * 3600) + (mins_later * 60) + secs_later;

    let mut time_difference: u32 = 0; 
    if earlier_time > later_time {
        time_difference = earlier_time - later_time;
    };

    time_difference
}
