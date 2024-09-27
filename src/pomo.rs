use std::io;
use std::time::SystemTime;
use std::{thread,time};
use std::io::{stdout, Write};
use notify_rust::Notification;
use kittyaudio::{Mixer, Sound};
use colored::Colorize;

#[derive(Debug)]
enum SessionType {
    Session,
    Break,
}

const SESSION_TIME: u64 = 25 * 60;
const BREAK_TIME: u64 = 5 * 60;
const SESSIONS: u8 = 4;

fn start_pomo_session(name: &str, session_type: SessionType, session_duration: u64) {
    let start_time = SystemTime::now();
    loop {
        let elapsed: u64 = start_time.elapsed().unwrap().as_secs(); 
        if elapsed >= session_duration {
            return;
        }
        let remaining = session_duration - elapsed;
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        let time = format!("{:02}:{:02}", minutes, seconds);
        print!("\r{} ({:?}) -> {}", name.magenta().bold(), session_type, time.yellow().bold());
        stdout().flush().unwrap();
        // Sleep because there is nothing to do for the next one second.
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn start(name: String, mixer: &mut Mixer, sound: &Sound) {
    for session in 1..1+SESSIONS {
        println!("Starting session {} for {}...", session, name.as_str().magenta().bold());
        play_audio(mixer, sound);
        start_pomo_session(name.as_str(), SessionType::Session, SESSION_TIME);
        println!("\nSession {} for {} ended", session, name.as_str().magenta().bold());
        show_notification("Pomodoro session ended 🍅!");
        play_audio(mixer, sound);
        if get_confirmation("Start the break?") {
            start_pomo_session(name.as_str(), SessionType::Break, BREAK_TIME);
            println!("\nThe break ended.");
            show_notification("break ended 🍅!");
            play_audio(mixer, sound);
        }
        if session != 1+SESSIONS && !get_confirmation("\nStart a new session?") {
            println!("Exiting {}... worked for {session} sessions", name.as_str().magenta().bold());
            return;
        }
    }
    show_notification("Pomodoro complete 🍅!");
}

fn get_confirmation(message: &str) -> bool {
    loop {
        print!("{} (y/n): ", message);
        stdout().flush().unwrap();
        let mut session_check = String::new();
        io::stdin()
            .read_line(&mut session_check)
            .expect("Failed to read");

        let session_check = session_check.trim().to_lowercase();

        match session_check.as_str() {
            "y" | "yes" => {
                return true;
            }
            "n" | "no" => {
                return false;
            }
            _ => {
                println!("Invalid input.");
                continue;
            }
        }
    }
}

fn play_audio(mixer: &mut Mixer, sound: &Sound) {
    mixer.play(sound.clone());
    mixer.wait(); // wait for all sounds to finish
}

fn show_notification(message: &str) {
    Notification::new()
        .summary("Pomodoro")
        .body(message)
        .timeout(0) // persistent notification
        .show()
        .unwrap();
}
