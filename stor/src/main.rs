#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use std::{env, process::ExitCode, fs, io};
use clap::{Parser, ValueEnum};
use serde::{Serialize, Deserialize};
use libsodium_rs::crypto_pwhash;

#[derive(Clone, ValueEnum)]
enum Action{
    Register,
    Read,
    Write,
    Create
}

#[derive(Parser)]
#[command(args_override_self = true, disable_help_flag = true)]
struct Args {
    #[arg(short = 'u', allow_hyphen_values = true)]
    user: Option<String>,

    #[arg(short = 'k', allow_hyphen_values = true)]
    key: Option<String>,

    #[arg(short = 'f', allow_hyphen_values = true)]
    file: Option<String>,

    #[arg(short = 'i', allow_hyphen_values = true)]
    input: Option<String>,

    #[arg(short = 'o', allow_hyphen_values = true)]
    output: Option<String>,

    #[arg(value_enum)]
    action: Option<Action>,

    text: Option<String>,
}

// Reference: Serde: Using derive
// https://serde.rs/derive.html
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    salt: [u8; 16],
    name_hashes: Vec<[u8; 32]>,
    nonce: [u8; 24],
    ciphertext: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct Db {
    users: Vec<User>,
}

// Reference: The match Control Flow Construct
// https://doc.rust-lang.org/book/ch06-02-match.html
fn parse_action(word: &str) -> Option<Action> {
    match word {
        "register" => Some(Action::Register),
        "read" => Some(Action::Read),
        "write" => Some(Action::Write),
        "Create" => Some(Action::Create),
        _ => None, // handel malformed input
    }
}
// Reference: Tutorial for the Derive API
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

fn check_args(args: &Args) -> bool{
    if args.user.is_none() {
        return false;
    }

    return match &args.action {
        Some(Action::Register) => {
            args.key.is_some() &&
            args.input.is_none() &&
            args.output.is_none() &&
            args.text.is_none()
        }
        Some(Action::Create) => {
            args.file.is_some() &&
            args.input.is_none() &&
            args.output.is_none() &&
            args.text.is_none()
        },
        Some(Action::Read) => {
            args.key.is_some() &&
            args.file.is_some() &&
            args.input.is_none() &&
            args.text.is_none()
        },
        Some(Action::Write) => {
            if args.input.is_some() && args.input.is_some() {
                return false;
            }
            if args.input.is_none() && args.input.is_none() {
                return false;
            }
            // Reference: Rust if let
            // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
            if let Some(text) = &args.text {
                if text.is_empty() {
                    return false;
                }
            }
            args.key.is_some() &&
            args.file.is_some() &&
            args.output.is_none()
        },
        None => false,
    };
}

fn parse_args() -> Result<Args, clap::Error>{
    Args::try_parse_from(env::args())
}

fn db_load() -> io::Result<Db> {
    match fs::read("enc.db") {
        Ok(raw) => { return db_parse(&raw)}
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                return Ok(Db {
                    users: Vec::new(),
                });
            }

            return Err(error);
        }
    }
}

fn db_parse(raw: &[u8]) -> io::Result<Db> {
    // Reference: Serde Json:
    // https://docs.rs/serde_json/latest/serde_json/
    let result:serde_json::Result<Db> =  serde_json::from_slice(raw);

    match result {
        Ok(db) => {
            for user in &db.users {
                if user.ciphertext.len() < 16 {
                    //println("ciphertext too short to encrypt!")
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData, "ciphertext too short to encrypt!"
                    ));
                }
            }

            return Ok(db);
        }
        Err(error) => {
            return Err(error.into());
        }
    }
       
}

fn db_save(db:&Db) -> io::Result<()> {
    let raw = serde_json::to_vec(db)?;
    return fs::write("enc.db" ,raw);
}

fn db_find(db: &Db, name: &str) -> Option<usize> {
    // Somehow it just occured to me I should have used a map
    // Oh well
    for i in 0..db.users.len() {
        if db.users[i].name == name {
            return Some(i);
        }
    }

    return None;
}

fn derive_key(key: &str, salt: &[u8]) -> libsodium_rs::Result<Vec<u8>> {
    // Reference: libsodium_rs Module crypto_pwhash
    // https://docs.rs/libsodium-rs/latest/libsodium_rs/crypto_pwhash/index.html
    return crypto_pwhash::pwhash(32, key.as_bytes(), salt, crypto_pwhash::OPSLIMIT_INTERACTIVE, crypto_pwhash::MEMLIMIT_INTERACTIVE, crypto_pwhash::ALG_ARGON2ID13);
}

fn main() {
    println!("Hello, world!");
}
