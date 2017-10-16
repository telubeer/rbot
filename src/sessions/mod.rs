use std::collections::HashMap;
use teleborg::objects::{Chat, User};

#[derive(Debug, Clone)]
pub struct Post {
    pub data: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Session {
    pub user: User,
    pub chat: Chat,
    pub post: Post
}

pub struct Sessions {
    pub sessions: HashMap<i64, Session>
}

impl Sessions {
    pub fn new() -> Self {
        Sessions {
            sessions: HashMap::new()
        }
    }

    pub fn start(&mut self, user: User, chat: Chat) {
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

    pub fn add(&mut self, id: i64, text: String) {
        if let Some(orig) = self.sessions.get_mut(&id) {
            orig.post.data.push(text);
        }
    }
}