use argparse::{ArgumentParser, Store, StoreTrue};
use colored::*;
use djangohashers::*;
use rpassword::read_password;
use std::io::{Write, stdout};
use std::process;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static DJANGO_VERSION: &str = "4.1";
static HELP_TEXT: &str = "Generates or validates password hashes used in Django Project.

Usage:

    djpass
        Prompts a password, generates a hash with default algorithm.

    djpass [password]
        Generates a hash with default algorithm.

    djpass [password] [-a <algorithm>]
        Generates a hash with defined algorithm.

    djpass [password] '[hash]'
        Verifies the hash against the password.
        WARNING: hash must be inside single quotes.

Algorithms:
- PBKDF2 (default)
- PBKDF2SHA1
- Argon2
- Scrypt
- BCryptSHA256
- BCrypt
- SHA1
- MD5
- UnsaltedSHA1
- UnsaltedMD5

Optional arguments:
-h, --help            Show this help message.
-v, --version         Show the version.
";

fn main() {
    // Arguments:
    let mut help = false;
    let mut version = false;
    let mut password = "".to_string();
    let mut algorithm = "".to_string();
    let mut hash = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut password).add_argument("password", Store, "");
        ap.refer(&mut algorithm).add_option(&["-a"], Store, "");
        ap.refer(&mut hash).add_argument("hash", Store, "");
        ap.refer(&mut help).add_option(&["-h", "--help"], StoreTrue, ""); // I prefer my own help.
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "");
        ap.parse_args_or_exit();
    }

    if help {
        println!("{}", HELP_TEXT);
        process::exit(0);
    }

    if version {
        println!("djpass {}, generates hashes for Django {}", VERSION, DJANGO_VERSION);
        process::exit(0);
    }

    match (!password.is_empty(), !algorithm.is_empty(), !hash.is_empty()) {
        (false, false, false) => {
            // No arguments, promts for password, generates hash with default algorithm:
            let mut stdout = stdout();
            print!("Password: ");
            stdout.flush().unwrap();
            let input = read_password().unwrap();
            if input.is_empty() {
                println!("{}", "Empty password.".red());
            } else {
                println!("{} {}", "Hash:".green(), make_password(&input));
            }
        }
        (true, false, false) => {
            // Password argument present, generates hash with default algorithm:
            println!("{} {}", "Hash:".green(), make_password(&password));
        }
        (true, true, false) => {
            // Password and algorithm arguments present, generates hash with as defined:
            let encoded = match algorithm.to_lowercase().as_ref() {
                "pbkdf2" => make_password_with_algorithm(&password, Algorithm::PBKDF2),
                "pbkdf2sha1" => make_password_with_algorithm(&password, Algorithm::PBKDF2SHA1),
                "argon2" => make_password_with_algorithm(&password, Algorithm::Argon2),
                "scrypt" => make_password_with_algorithm(&password, Algorithm::Scrypt),
                "bcryptsha256" => make_password_with_algorithm(&password, Algorithm::BCryptSHA256),
                "bcrypt" => make_password_with_algorithm(&password, Algorithm::BCrypt),
                "sha1" => make_password_with_algorithm(&password, Algorithm::SHA1),
                "md5" => make_password_with_algorithm(&password, Algorithm::MD5),
                "unsaltedsha1" => make_password_with_algorithm(&password, Algorithm::UnsaltedSHA1),
                "unsaltedmd5" => make_password_with_algorithm(&password, Algorithm::UnsaltedMD5),
                "crypt" => make_password_with_algorithm(&password, Algorithm::Crypt),
                _ => "".to_string(),
            };
            if encoded.is_empty() {
                println!("{}", "Algorithm not supported.".red());
            } else {
                println!("{} {}", "Hash:".green(), encoded);
            }
        }
        (true, _, true) => {
            // Password and hash arguments present, verifies the pair:
            if !algorithm.is_empty() {
                println!("{}", "Algorithm ignored for verification.".yellow());
            }
            if is_password_usable(&hash) {
                if check_password_tolerant(&password, &hash) {
                    println!("{}", "Password ok.".green());
                } else {
                    println!("{}", "Password does not match hash.".red());
                }
            } else {
                println!("{}", "Hash is not properly formatted.".red());
            }
        }
        (_, _, _) => unreachable!(),
    }
}
