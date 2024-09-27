mod pomo;
use kittyaudio::{include_sound, Mixer};
use std::env;

const HELP: &str = "Usage: pomo-rs [COMMAND] [ARGUMENTS]
pomo-rs is a simple pomodoro app.
Example: pomo-rs start <task-name>
Available commands:
- start: starts a pomodoro
";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let command = args[1].as_str();
        match command {
            "start" => {
                // include a sound into the executable.
                // this type can be cheaply cloned.
                let sound = include_sound!("../assets/bell.ogg").unwrap();
                // create sound mixer
                let mut mixer = Mixer::new();
                mixer.init(); // use init_ex to specify settings
                let task_name = String::from(&args[2]);
                pomo::start(task_name, &mut mixer, &sound);
            },
            "--help" | "-h" | _ => println!("{}", HELP)
        }
    } else {
        println!("{}", HELP);
    }
}
