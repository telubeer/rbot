use sessions::Sessions;
use teleborg::{Bot, Command};
use teleborg::objects::{Update, User, Chat, Message};
use std::sync::{Arc, Mutex};

pub struct Start {
    sessions: Arc<Mutex<Sessions>>
}

impl Start {
    pub fn new(sessions: Arc<Mutex<Sessions>>) -> Self {
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