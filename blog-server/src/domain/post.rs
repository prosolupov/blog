use crate::domain::user::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PostId(pub Uuid);

impl From<Uuid> for PostId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl PostId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Serialize)]
pub struct Post {
    id: PostId,
    title: String,
    content: String,
    author_id: UserId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}
#[derive(Serialize)]
pub struct ResponsePost {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdatePost {
    title: String,
    content: String,
}


#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}