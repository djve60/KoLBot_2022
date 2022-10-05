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
