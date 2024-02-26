use bounce::Atom;
use serde::{Deserialize, Serialize};

use crate::utils::callback::ResponseCallback;

#[derive(Atom, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
}

impl Default for Topic {
    fn default() -> Self {
        Self {
            id: None,
            title: String::new(),
            description: String::new(),
        }
    }
}

impl Topic {
    pub fn new(id: Option<String>, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
        }
    }
}

pub trait TopicService {
    fn save(&self, topic: Topic, callback: ResponseCallback<(), String>);
}

pub trait TopicRepository {
    fn save(&self, topic: Topic, callback: ResponseCallback<(), String>);
}
