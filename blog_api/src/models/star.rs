use crate::models::{post::PostId, user::UserId};

/// An association between a user who was starred a post
#[derive(Debug, Clone, Copy)]
pub struct Star {
    user_id: UserId,
    post_id: PostId,
}

impl Star {
    pub fn get_user_id(&self) -> UserId {
        self.user_id
    }
    pub fn get_post_id(&self) -> PostId {
        self.post_id
    }
    pub fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Star {
            user_id: row.get(0)?,
            post_id: row.get(1)?,
        })
    }
}
