extern crate chrono;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    summary: String,
    contents: String,
    author_handle: String,
    date_time: DateTime<Utc>,
    uuid: Uuid,
}

impl Post {
    pub fn new(
        summary: &str,
        contents: &str,
        author: &Author,
        date_time: DateTime<Utc>,
        uuid: Uuid,
    ) -> Post {
        Post {
            summary: summary.to_string(),
            contents: contents.to_string(),
            author_handle: author.handle.clone(),
            date_time: date_time,
            uuid: uuid,
        }
    }

    pub fn from_post(
        summary: &str,
        contents: &str,
        author: &Author) -> Post {
        Post {
            summary: summary.to_string(),
            contents: contents.to_string(),
            author_handle: author.handle.clone(),
            date_time: chrono::offset::Utc::now(),
            uuid: Uuid::new_v4()
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    handle: String,
}

impl Author {
    pub fn new(handle: &str) -> Author {
        Author { handle: handle.to_string() }
    }
}
