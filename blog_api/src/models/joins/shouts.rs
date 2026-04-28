use json::{JsonValue, object};

use crate::models::{
    shout::ShoutId,
    user::{Color, UserId},
};

#[derive(Debug)]
pub struct JoinedShout {
    shout_id: ShoutId,
    user_id: UserId,
    content: String,
    user_display_name: String,
    user_color: Color,
    user_banned: bool,
}

impl JoinedShout {
    pub fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            shout_id: row.get(0)?,
            user_id: row.get(1)?,
            content: row.get(2)?,
            user_display_name: row.get(3)?,
            user_color: row.get(4)?,
            user_banned: row.get(5)?,
        })
    }
    pub fn get_shout_id(&self) -> ShoutId {
        self.shout_id
    }
    pub fn get_user_id(&self) -> UserId {
        self.user_id
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_user_display_name(&self) -> &str {
        &self.user_display_name
    }
    pub fn get_user_color(&self) -> Color {
        self.user_color
    }
    pub fn is_user_banned(&self) -> bool {
        self.user_banned
    }
    pub fn to_json(&self) -> JsonValue {
        object! {
            shout_id : self.shout_id.inner(),
            user_id:self.user_id.inner(),
            content:self.content.as_str(),
            user_display_name:self.user_display_name.as_str(),
            user_color:self.user_color.to_string()
        }
    }
}
