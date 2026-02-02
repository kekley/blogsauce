use jiff::civil::DateTime;
use json::JsonValue;
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError},
};

use crate::models::user::UserId;

#[derive(Debug, Clone, Copy)]
pub struct ShoutId {
    inner: i64,
}

impl From<ShoutId> for JsonValue {
    fn from(val: ShoutId) -> Self {
        JsonValue::Number(val.inner.into())
    }
}

impl FromSql for ShoutId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Integer(value) => Ok(Self { inner: value }),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for ShoutId {
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
    id: ShoutId,
    user_id: UserId,
    content: String,
    edited: bool,
    posted_on: DateTime,
}

impl Comment {
    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_datetime(&self) -> &DateTime {
        &self.posted_on
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
            user_id: row.get(1)?,
            content: (row.get::<usize, String>(2)?),
            edited: row.get(3)?,
            posted_on: row.get(4)?,
        })
    }

    pub fn get_id(&self) -> ShoutId {
        self.id
    }
}
