extern crate teleborg;

use teleborg::{Dispatcher, Bot, Updater, Command};
use teleborg::error::Error;
use teleborg::objects::{Update, Chat, User, PhotoSize, InlineKeyboardMarkup, InlineKeyboardButton, Message};
use std::env;
use std::collections::HashMap;
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
    dispatcher.add_command_handler("test", test, false);
    dispatcher.add_message_handler(add_command);
    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(None, None, None, None, dispatcher);
}

// Our first command handler
fn test(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    bot.reply_to_message(&update, "It works!").unwrap();
}// Our first command handler
// Our first command handler
fn start(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    bot.reply_to_message(&update, "It works!").unwrap();
}// Our first command handler

#[derive(Debug, Clone)]
struct Post {
    data: Vec<String>
}

#[derive(Debug, Clone)]
struct Session {
    user: User,
    chat: Chat,
    post: Post
}

struct Sessions {
    pub sessions: HashMap<i64, Session>
}

impl Sessions {
    fn new() -> Self {
        Sessions {
            sessions: HashMap::new()
        }
    }

    fn start(&mut self, user: User, chat: Chat) {
        let session = Session {
            user,
            chat,
            post: Post {
                data: Vec::new()
            }
        };
        let id = session.user.id;
        let data = self.sessions.entry(id).or_insert(session);
        data.post.data = Vec::new();
    }

    fn add(&mut self, id: i64, text: String) {
        if let Some(orig) = self.sessions.get_mut(&id) {
            orig.post.data.push(text);
        }
    }
}

struct Start {
    sessions: Arc<Mutex<Sessions>>
}

impl Start {
    fn new(sessions: Arc<Mutex<Sessions>>) -> Self {
        Start {
            sessions
        }
    }

    fn start(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        if let Some(m) = update.message {
            if let Some(user) = m.from {
                let id = user.id;
                let mut guard = self.sessions.lock().unwrap();
                guard.start(user, m.chat);
                if let Some(session) = guard.sessions.get(&id) {
                    println!("session started for {:?} in {:?}", session.user, session.chat);
                }
            }
        }
    }
}

impl Command for Start {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.start(bot, update, args);
    }
}

struct Get {
    sessions: Arc<Mutex<Sessions>>
}

impl Get {
    fn new(sessions: Arc<Mutex<Sessions>>) -> Self {
        Get {
            sessions
        }
    }

    fn start(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        let up = update.clone();
        if let Some(m) = update.message {
            if let Some(user) = m.from {
                let id = user.id;
                let guard = self.sessions.lock().unwrap();
                if let Some(session) = guard.sessions.get(&id) {
                    let message = format!("session data is {:?}", session.post.data);
                    bot.reply_to_message(&up, &message);
                }
            }
        }
    }
}

impl Command for Get {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.start(bot, update, args);
    }
}


struct Add {
    sessions: Arc<Mutex<Sessions>>
}

impl Add {
    fn new(sessions: Arc<Mutex<Sessions>>) -> Self {
        Add {
            sessions
        }
    }

    fn add(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        if let Some(qbc) = update.callback_query {
            println!("callback {:?}", qbc);
            return;
        }
        let up = update.clone();
        let message = update.clone().message.unwrap();
        if let Some(m) = update.message {
            if let Some(user) = m.from {
                if let Some(text) = m.text {
                    let id = user.id;
                    let mut guard = self.sessions.lock().unwrap();
                    guard.add(id, text);
                    reply_with_cancel_btn(&bot, &message, "saved".as_ref());
                }
                if let Some(photo) = m.photo {
                    if let Some(psize) = photo.first() {
                        if let Some(path) = psize.clone().file_path {
                            let id = user.id;
                            let mut guard = self.sessions.lock().unwrap();
                            guard.add(id, path);
                            reply_with_cancel_btn(&bot, &message, "saved".as_ref());
                        }
                    }
                }
            } else {
                println!("message {:?}", m);
            }
        }
    }
}

fn reply_with_cancel_btn(bot: &Bot, message: &Message, text: &str) {
    let message_id = message.message_id;
    let chat_id = message.chat.id;
    let mut buttons = Vec::<Vec<InlineKeyboardButton>>::new();
    let mut row = Vec::<InlineKeyboardButton>::new();
    row.push(InlineKeyboardButton::new(
        "Отмена".to_string(),
        None,
        Some("Отмена".to_string()),
        None,
        None,
    ));
    buttons.push(row);
    let markup = InlineKeyboardMarkup::new(buttons);
    let result: Message = bot.send_message(
        &chat_id,
        &text,
        None,
        None,
        None,
        Some(&message_id),
        Some(markup)
    ).unwrap();
}

impl Command for Add {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.add(bot, update, args);
    }
}