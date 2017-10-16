extern crate teleborg;
#[macro_use] extern crate lazy_static;
extern crate regex;

mod commands;
mod sessions;

use sessions::Sessions;
use commands::{Add, Get, Start};
use teleborg::{Dispatcher, Bot, Updater};
use std::sync::{Arc, Mutex};

fn main() {
    let mut sessions = Arc::new(Mutex::new(Sessions::new()));
    let start_command = Start::new(sessions.clone());
    let get_command = Get::new(sessions.clone());
    let add_command = Add::new(sessions.clone());
    // Make sure you have your token
    // Creating a dispatcher which registers all the command and message handlers
    let mut dispatcher = Dispatcher::new();

    // Registering our command which we create below in the form as a function
    dispatcher.add_command_handler("start", start_command, false);
    dispatcher.add_command_handler("show", get_command, false);
    dispatcher.add_message_handler(add_command);
    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(None, None, None, None, dispatcher);
}
