use jiff::civil::DateTime;

use crate::models::{
    comment::CommentId,
    user::{Color, UserId},
};

#[derive(Debug)]
pub struct JoinedComment {
    comment_id: CommentId,
    user_id: UserId,
    content: String,
    edited: bool,
    updated_on: DateTime,
    user_display_name: String,
    user_color: Color,
    user_banned: bool,
}

impl JoinedComment {
    pub fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            comment_id: row.get(0)?,
            user_id: row.get(1)?,
            content: row.get(2)?,
            edited: row.get(3)?,
            updated_on: row.get(4)?,
            user_display_name: row.get(5)?,
            user_color: row.get(6)?,
            user_banned: row.get(7)?,
        })
    }
    pub fn get_comment_id(&self) -> CommentId {
        self.comment_id
    }
    pub fn get_user_id(&self) -> UserId {
        self.user_id
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn was_edited(&self) -> bool {
        self.edited
    }
    pub fn updated_on(&self) -> &DateTime {
        &self.updated_on
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
}
