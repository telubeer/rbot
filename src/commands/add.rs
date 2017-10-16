use regex::Regex;
use sessions::Sessions;
use teleborg::{Bot, Command, NO_MARKUP};
use teleborg::objects::{Update, User, Chat, PhotoSize, InlineKeyboardMarkup, InlineKeyboardButton, ReplyKeyboardMarkup, Message};
use std::sync::{Arc, Mutex};

pub struct Add {
    sessions: Arc<Mutex<Sessions>>
}

impl Add {
    pub fn new(sessions: Arc<Mutex<Sessions>>) -> Self {
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
            self.process_message(bot, m);
        } else {
            println!("message {:?}", message);
        }
    }

    fn process_message(&self, bot: &Bot, m: Message,) {
        let message = m.clone();
        if let Some(user) = m.from {
            let mut guard = self.sessions.lock().unwrap();
            let id = user.id;
            let chat = m.chat.id;
            let is_session_not_started = !guard.sessions.contains_key(&id);
            if is_session_not_started {
                if let Some(text) = m.text {
                    if is_article(&text) {
                        guard.start(user, m.chat);
                        send_start_message(bot, &chat);
                        return;
                    }
                }
                bot.send_message(
                    &chat,
                    "Привет! Запостим?\nЧтобы найти материал отправь айди или ссылку",
                    None, None, None,
                    None, NO_MARKUP
                );
                return;
            }
            if let Some(text) = m.text {
                guard.add(id, text);
                reply_with_cancel_btn(bot, &message, "saved".as_ref());
            }
            if let Some(photo) = m.photo {
                if let Some(psize) = photo.first() {
                    if let Some(path) = psize.clone().file_path {
                        guard.add(id, path);
                        reply_with_cancel_btn(bot, &message, "saved".as_ref());
                    }
                }
            }
        }
    }

}

impl Command for Add {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.add(bot, update, args);
    }
}

fn get_publish_button() -> ReplyKeyboardMarkup {
    let mut buttons = Vec::<Vec<InlineKeyboardButton>>::new();
    let mut row = Vec::<InlineKeyboardButton>::new();
    row.push(InlineKeyboardButton::new(
        "Опубликовать".to_string(),
        None,
        None,
        None,
        None,
    ));
    buttons.push(row);
    ReplyKeyboardMarkup::new(
        buttons,
        Some(true),
        Some(false),
        Some(false)
    )
}



fn send_start_message(bot: &Bot, chat: &i64) {
    let r = bot.send_message(
        chat,
        "Нашли материал! Присылай текст или картинки, когда закончишь нажми кнопку \"Опубликовать\"                                ",
        None, None, None,
        None,
        Some(get_publish_button())
    );
    println!("start ? {:?}", r);
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


fn is_article(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\d]+").unwrap();
    }
    RE.is_match(text)
}