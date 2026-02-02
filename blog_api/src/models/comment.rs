use jiff::civil::DateTime;
use json::JsonValue;
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError},
};

use crate::models::{post::PostId, user::UserId};

#[derive(Debug, Clone, Copy)]
pub struct CommentId {
    inner: i64,
}

impl From<CommentId> for JsonValue {
    fn from(val: CommentId) -> Self {
        JsonValue::Number(val.inner.into())
    }
}

impl FromSql for CommentId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Integer(value) => Ok(Self { inner: value }),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for CommentId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.inner.into())
    }
}

/// A comment! Has a unique ID, as well as an associated post and user. Users can enter an
/// arbitrary display name and the comment content. both of which are non-empty strings
/// The struct also includes some data for tracking whether or not the comment has been edited, and
/// the stored date will represent either the creation or edit date
#[derive(Debug)]
pub struct Comment {
    id: CommentId,
    posted_under: PostId,
    user_id: UserId,
    display_name: String,
    content: String,
    edited: bool,
    posted_on: DateTime,
}

impl Comment {
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_datetime(&self) -> &DateTime {
        &self.posted_on
    }
    pub fn get_post_id(&self) -> PostId {
        self.posted_under
    }
    pub fn get_user_id(&self) -> UserId {
        self.user_id
    }
    pub fn was_edited(&self) -> bool {
        self.edited
    }
    pub fn from_row(row: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Comment {
            id: row.get(0)?,
            posted_under: row.get(1)?,
            user_id: row.get(2)?,
            display_name: (row.get::<usize, String>(3)?),
            content: (row.get::<usize, String>(4)?),
            edited: row.get(5)?,
            posted_on: row.get(6)?,
        })
    }

    pub fn get_id(&self) -> CommentId {
        self.id
    }
}
