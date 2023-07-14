# any_match


```rust
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
```
