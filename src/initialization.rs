// Copyright D. Evans (djve60+kolbot@gmail.com).

/// Initialization functions for kolbot_2022.
/// There are multiple possible features that are located here
/// to make the code more readable.
use std::fs;
use std::io::Write;
use std::path::Path;
use std::str;

// Crates from rust.io.
use clap::Parser;
use configparser::ini::Ini;
use rpassword::read_password;

// Local libraries.
use crate::utilities;

//
// Globals here.
//
const DEFAULT_CONF_FILE: &[&str] = &[".", "data", "initialize.ini"];

#[derive(Parser, Debug)]
#[clap(version)]
#[clap(about = "kolbot_2022 is for automating actions in KoL.")]
#[clap(author, long_about = None)] //Read from Cargo.toml.
pub struct Args {
    /// Debug level.
    #[clap(short, long, action = clap::ArgAction::Count,
        help="Debug level, specified multiple times.\nOptional.")]
    pub debug_level: u8,

    /// Path to the config file.
    #[clap(
        short,
        long,
        value_parser,
        value_name = "PATH",
        help = "Path to the initial configuration file.\nOptional."
    )]
    pub config_file: Option<String>,

    /// Account name to use.
    #[clap(
        short,
        long,
        value_parser,
        value_name = "ACCOUNT",
        required(false),
        help = "Account name to run-as.\nOptional."
    )]
    pub account: Option<String>,
}

/// Initialization top level function.
pub fn start() -> (Ini, Ini, u8) {
    // What were the CLI arguments, on invocation.
    let args = get_cli_arguments();

    // What is in the configuration file.
    let mut configuration =
        read_startup_configuration(DEFAULT_CONF_FILE, args.config_file, args.debug_level);

    configuration = merge_cli_args_with_configuration(
        configuration,
        args.account.as_deref().unwrap_or("").to_string(),
        args.debug_level.to_string(),
    );

    // Ensure required directories are in place.
    configuration = create_directory_paths(configuration, args.debug_level);

    if args.debug_level > 2 {
        log::info!("In initialization::start()");
        log::info!(
            "Login account is: {:?}",
            configuration.get("login", "account").unwrap()
        );
    }

    // Confirm expected resources are available, like directories.
    configuration = confirm_resources(configuration, args.debug_level);

    // Read in the strings and regular expressions to be used.
    let kol_strings = get_kol_strings(&configuration);

    (configuration, kol_strings, args.debug_level)
}

//
// Private functions below here.
//

// Take the array in the config file and convert to a path as a String.
fn confirm_resources(mut configuration: Ini, debug_level: u8) -> Ini {
    // Do we have a password? If not, prompt for it.
    // Since this is a standalone application it doesn't have access
    // to a secure repository so normally it will be in the config file.
    let password = configuration.get("login", "password").unwrap();
    if password.is_empty() {
        print!(
            "Type a password for {}: ",
            configuration.get("login", "account").unwrap()
        );
        std::io::stdout().flush().unwrap();
        let password = read_password().unwrap();
        configuration.set("login", "password", Some(password));
    }

    // Are expected directories in place? If not, create them.
    let mut directories = String::new();
    let sections = configuration.sections();
    for section in sections {
        match section.as_str() {
            "directories" => directories = section,
            &_ => (),
        }
    }
    for dir in ["data", "logging", "status"] {
        let path = configuration.get(&directories, dir).unwrap();

        if debug_level > 3 {
            log::info!("Creating dir: {}", path);
        }

        if !Path::new(&path).exists() {
            fs::create_dir_all(&path).unwrap_or_else(|e| panic!("Error creating dir: {}", e));
        }
    }

    configuration
}

fn create_directory_paths(mut configuration: Ini, debug_level: u8) -> Ini {
    if debug_level > 3 {
        log::info!("In create_directory_paths().");
    }

    let mut directories = String::new();
    let sections = configuration.sections();
    let trim_chars: &[_] = &['[', ']'];

    for section in sections {
        if section.as_str() == "directories" {
            directories = section
        }
    }

    for dir in ["data", "logging", "status"] {
        let mut path = configuration.get(&directories, dir).unwrap();
        path = path.trim_matches(trim_chars).to_string();

        if debug_level > 3 {
            log::info!("  Config for {} is {}!", dir, path);
        }

        let str_vector: Vec<&str> = path.split(',').collect();

        configuration.set(
            &directories,
            dir,
            Some(utilities::get_path_from_array(&str_vector)),
        );
    }

    configuration
}

// Obviously get command line arguments.
fn get_cli_arguments() -> Args {
    let args = Args::parse();

    if args.debug_level > 1 {
        log::info!("CLI args-");
        //debug_string!(),
        log::info!("Conf file: {}", args.account.as_deref().unwrap_or(""));
        log::info!("Account: {}", args.config_file.as_deref().unwrap_or(""));
        log::info!("Debug level{}", args.debug_level);
    }
    args
}

// Get the strings and regular expressions.
//fn get_kol_strings(configuration: &Ini) -> u8 {
fn get_kol_strings(configuration: &Ini) -> Ini {
    // This has already been made into an path for the OS.
    let file_location: String = configuration.get("directories", "data").unwrap();

    let file_path =
        Path::new(&file_location).join(configuration.get("login", "strings_file").unwrap());

    // Read the stings and regular expressions file.
    let mut strings_and_regexs = Ini::new_cs();

    // Yes, the function complains if there is no assignment.
    let _map = strings_and_regexs.load(file_path).unwrap();

    strings_and_regexs
}

// Let CLI override defaults.
fn merge_cli_args_with_configuration(
    mut configuration: Ini,
    account: String,
    debug_level: String,
) -> Ini {
    configuration.set("login", "debug_level", Some(debug_level));

    if !account.is_empty() {
        configuration.set("login", "account", Some(account));
    }
    configuration
}

// Find the start-up configuration data and read it into HashMap.
fn read_startup_configuration(
    default_location: &[&str],
    cli_location: Option<String>,
    debug_level: u8,
) -> Ini {
    // Use CLI location, if supplied.
    let mut config_file_location = utilities::get_path_from_array(default_location);
    if !cli_location.as_deref().unwrap_or("").is_empty() {
        config_file_location = cli_location.as_deref().unwrap_or("").to_string();
    }

    if debug_level > 1 {
        log::info!("Configuration File Loc is {}", config_file_location);
    }

    // Read the config file.
    let mut configuration_details = Ini::new_cs();

    // Yes, the function complains if there is no assignment.
    let _map = configuration_details.load(config_file_location).unwrap();

    if debug_level > 1 {
        log::info!("In initialization::read_startup_configuration()");
        let sections = configuration_details.sections();
        log::info!("Sections are: {:?}", sections);
    }
    configuration_details
}
