use std::error::Error as StdError;
use std::io::Error as IoError;

use any_match::any_match;

fn main() {
    // this should work for box dyn errors as well
    // there is a subtlety in that anyhow downcasting requires  Display + Debug + Send + Sync + 'static
    // while box dyn error downcasting requires Error + 'static
    // which is why String is not a valid match arm on box dyn error. it does not impl Error
    let box_dyn_err = throw_box_dyn_error_string().unwrap_err();
    let as_string = any_match! {(box_dyn_err):
        (IoError as io) => {
            println!("Found IoError");
            // another difference is that the type of `io` is `Box<IoError>` instead of an `IoError`
            io.to_string()
        }
        x => {
            // String will end up in the catch all because it cannot be used as a match arm for box dyn
            println!("Catch all downcast box");
            format!("Uncaught: {x}")
        }
    };

    println!("{as_string}");
}

fn throw_box_dyn_error_string() -> Result<(), Box<dyn StdError + 'static>> {
    Err(String::from("This is a box dyn error wrapping a String").into())
}
