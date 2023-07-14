use std::{
    io::Error as IoError,
    str::Utf8Error,
    sync::mpsc::{RecvError, SendError},
};

use anyhow::{Context, Result as AnyResult};

use any_match::any_match;

fn main() {
    let anyhow_err = throw_io().unwrap_err();

    let as_string = stringify_and_print(anyhow_err);

    println!("{as_string}");

    let as_string = stringify_and_print(anyhow::anyhow!("This is a string lit"));

    println!("{as_string}");

    let as_string = stringify_and_print(with_context().unwrap_err());

    println!("{as_string}");

    let anyhow_err = throw_io().unwrap_err();
    any_match! { (anyhow_err):
        _ => {
            // idk why you would do this but you can
            println!("Do nothing");
        }
    }
}

fn stringify_and_print(anyhow_err: anyhow::Error) -> String {
    any_match! { (anyhow_err):
        (String as s) => {
            println!("Found a String");
            s
        }
        (&str as s) => {
            println!("Found str slice");
            s.to_string()
        }
        (IoError as io_error) => {
            println!("IOError");
            io_error.to_string()
        }
        (Utf8Error as utf8_error) => {
            println!("Error reading utf8 string from bytes");
            utf8_error.to_string()
        }
        (SendError<String> as send_error) => {
            println!("Error sending string on mpsc channel");
            send_error.to_string()
        }
        (RecvError as recv_error) => {
            println!("Error receiving on mpsc channel");
            recv_error.to_string()
        }
        uncaught_err => {
            println!("Catch All");
            format!("Not properly caught: {uncaught_err}")
        }
    }
}

fn with_context() -> AnyResult<()> {
    throw_string().context("Some Other wrapper message")
}

fn throw_string() -> AnyResult<()> {
    Err(anyhow::anyhow!(String::from("Hi")))
}

fn throw_io() -> AnyResult<()> {
    std::fs::read("./not a valid path")?;
    Ok(())
}
