extern crate argparse;
extern crate rpassword;
extern crate ansi_term;
extern crate djangohashers;

use std::process;
use std::io::{stdout, Write};
use argparse::{ArgumentParser, Store, StoreTrue};
use rpassword::read_password;
use ansi_term::Colour::{Green, Red, Yellow};
use djangohashers::*;


static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static DJANGO_VERSION: &'static str = "1.9";
static HELP_TEXT: &'static str = "Generates or validates password hashes used in Django Project.

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
        ap.refer(&mut help).add_option(&["-h", "--help"], StoreTrue, "");  // I prefer my own help.
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

    // No arguments, promts for password, generates hash with default algorithm:
    if password == "" && algorithm == "" && hash == "" {
        let mut stdout = stdout();
        print!("Password: ");
        stdout.flush().unwrap();
        let input = rpassword::read_password().unwrap();
        if input == "" {
            println!("{}", Red.paint("Empty password."));
        } else {
            println!("{} {}", Green.paint("Hash:"), make_password(&input));
        }
    }

    // Password argument present, generates hash with default algorithm:
    if password != "" && algorithm == "" && hash == "" {
        println!("{} {}", Green.paint("Hash:"), make_password(&password));
    }

    // Password and algorithm arguments present, generates hash with as defined:
    if password != "" && algorithm != "" && hash == "" {
        let encoded = match algorithm.to_lowercase().as_ref() {
            "pbkdf2" => make_password_with_algorithm(&password, Algorithm::PBKDF2),
            "pbkdf2sha1" => make_password_with_algorithm(&password, Algorithm::PBKDF2SHA1),
            "bcryptsha256" => make_password_with_algorithm(&password, Algorithm::BCryptSHA256),
            "bcrypt" => make_password_with_algorithm(&password, Algorithm::BCrypt),
            "sha1" => make_password_with_algorithm(&password, Algorithm::SHA1),
            "md5" => make_password_with_algorithm(&password, Algorithm::MD5),
            "unsaltedsha1" => make_password_with_algorithm(&password, Algorithm::UnsaltedSHA1),
            "unsaltedmd5" => make_password_with_algorithm(&password, Algorithm::UnsaltedMD5),
            _ => "".to_string(),
        };
        if encoded == "" {
            println!("{}", Red.paint("Algorithm not supported."));
        } else {
            println!("{} {}", Green.paint("Hash:"), encoded);
        }
    }

    // Password and hash arguments present, verifies the pair:
    if password != "" && hash != "" {
        if algorithm != "" {
            println!("{}", Yellow.paint("Algorithm ignored for verification."));
        }
        if is_password_usable(&hash) {
            if check_password_tolerant(&password, &hash) {
                println!("{}", Green.paint("Password ok."));

            } else {
                println!("{}", Red.paint("Password does not match hash."));
            }
        } else {
            println!("{}", Red.paint("Hash is not properly formatted."));
        }
    }

}
