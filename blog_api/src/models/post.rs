use std::fmt::Write;

use rusqlite::{
    Row, ToSql,
    types::{FromSql, FromSqlError},
};

pub type PostIdentifier = heapless::String<255, u8>;

#[derive(Debug, Clone, Copy)]
pub struct PostId {
    inner: i64,
}

impl ToSql for PostId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.inner.into())
    }
}

impl FromSql for PostId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Integer(value) => Ok(PostId { inner: value }),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

/// A unique ID for a post alongside a unique identifier (usually the title)
pub struct Post {
    id: PostId,
    post_ident: PostIdentifier,
}

impl Post {
    pub fn from_sqlite_row(row: &Row<'_>) -> Result<Self, rusqlite::Error> {
        let ident_string = row.get::<usize, String>(1)?;
        let mut ident_inline = PostIdentifier::new();
        ident_inline
            .write_str(ident_string.as_str())
            .expect("Post Identifier String was too long");
        Ok(Self {
            id: row.get(0)?,
            post_ident: ident_inline,
        })
    }
    pub fn get_id(&self) -> PostId {
        self.id
    }

    pub fn get_ident(&self) -> &str {
        &self.post_ident
    }
}
