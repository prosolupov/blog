use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };
use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PostId(pub Uuid);

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
    title: String,
    content: String,
}

#[derive(Deserialize)]
pub struct UpdatePost {
    title: String,
    content: String,
}