use std::fs;

// Any struct that implements this trait can be registered and run by the Terminal
trait Command {
    // Returns the command's trigger word
    fn get_name(&self) -> &str;

    // Executes the command
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("pong!");
    }
}

struct CountCommand {}

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("times has been called {} times", self.count);
    }
}

struct ShoutCommand {}

impl Command for ShoutCommand {
    fn get_name(&self) -> &str {
        "shout"
    }

    fn exec(&mut self, args: &[&str]) {
        let message = args.join(" ").to_uppercase();
        println!("{}!!!", message);
    }
}

struct Terminal {
    // Box puts all items in the same Vec, ignoring that they have different sizes
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    // Starts with a blank terminal
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
        }
    }

    // Takes ownership of a boxed Command and pushes it into the vector
    fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    // Reads commands.txt
    fn run(&mut self) {
        let content = match fs::read_to_string("commands.txt") {
            Ok(text) => text,
            Err(err) => {
                println!("Error reading file: {}", err);
                return;
            }
        };

        // Iterate over the file text one line at a time
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            // If the line was blank or only whitespace, skip it and move to the next line
            if parts.is_empty() {
                continue;
            }

            // The first word is the command name; everything from index 1 onward is the arguments
            let cmd_name = parts[0];
            let args = &parts[1..];

            // Handle the built-in stop command
            if cmd_name == "stop" {
                println!("Stopping terminal.");
                break;
            }

            // Suggest stop if the user wrote it with wrong capitalization (e.g., "STOP")
            if cmd_name.eq_ignore_ascii_case("stop") {
                println!("Unknown command '{}'. Did you mean 'stop'?", cmd_name);
                continue;
            }

            // Search for a matching command in our registered list
            let mut found = false;
            for cmd in &mut self.commands {
                if cmd.get_name() == cmd_name {
                    cmd.exec(args);
                    found = true;
                    break;
                }
            }

            // If no exact match was found, check for capitalization typos
            if !found {
                let mut suggestion = None;

                for cmd in &self.commands {
                    if cmd.get_name().eq_ignore_ascii_case(cmd_name) {
                        suggestion = Some(cmd.get_name());
                        break;
                    }
                }

                if let Some(correct_name) = suggestion {
                    println!(
                        "Unknown command '{}'. Did you mean '{}'?",
                        cmd_name, correct_name
                    );
                } else {
                    println!("Error: Unknown command '{}'", cmd_name);
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(ShoutCommand {}));

    terminal.run();
}
