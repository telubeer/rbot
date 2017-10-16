use sessions::Sessions;
use teleborg::{Bot, Command};
use teleborg::objects::{Update, User, Message};
use std::sync::{Arc, Mutex};

pub struct Get {
    sessions: Arc<Mutex<Sessions>>
}

impl Get {
    pub fn new(sessions: Arc<Mutex<Sessions>>) -> Self {
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