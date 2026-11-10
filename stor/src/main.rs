#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

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


fn parse_args() -> Result<Args, clap::Error>{
    Args::try_parse_from(env::args())
}

fn main() {
    println!("Hello, world!");
}
