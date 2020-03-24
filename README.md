# cli

cli is a very simple command-line argument parser
NOTE: It is currently in-progress

## Usage
```rust
use cli::{App, Command, Context, Flag, FlagKind};
use std::env::args;
use std::process;

struct Greeter {
    greeting: String,
}

// initiate the app
let app: App<Greeter> = App::new(Greeter {greeting: String::from("Good morning"))
    .register(
        Command::new(
            "greet", "g", "greet someone",
            |inner: Greeter, c: Context| {
                if c.arg.len() == 0 {
                    if let Some(name) = c.get("n") {
                        println!("{}, {}", inner.greeting, name);
                    } else {
                        eprintln!("No name specified!");
                        process::exit(1);
                    }
                } else {
                    println!("{}, {}", inner.greeting, c.arg[0]);
                }
            }
        )
        .flag(Flag::new("name", "n", FlagKind::InputFlag, "someone's name"))
    );

// collect arguments
let args: Vec<String> = args().collect();

//run the app
app.run(args);
```