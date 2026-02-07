pub mod statements;

use std::sync::Arc;
use std::{path::Path, str};

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use thiserror::Error;

use crate::models::ip::TruncatedIp;
use crate::models::shout::{Shout, ShoutId};
use crate::models::user::{User, UserId};
use crate::models::{
    comment::{Comment, CommentId},
    post::{Post, PostId},
    star::Star,
};
use statements::*;

pub struct CommentDb {
    conn: PooledConnection<SqliteConnectionManager>,
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("{0}")]
    SqliteError(#[from] rusqlite::Error),
}

impl CommentDb {
    pub fn create_db(path: &Path) -> Arc<Pool<SqliteConnectionManager>> {
        let sqlite_file = path;
        let sqlite_connection_manager = SqliteConnectionManager::file(sqlite_file);
        let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager)
            .expect("Failed to create r2d2 SQLite connection pool");
        let temp_conn = sqlite_pool.get().expect("Failed to make DB connection");
        temp_conn
            .execute(CREATE_COMMENT_TABLE, ())
            .expect("Failed to create comment table");

        temp_conn
            .execute(CREATE_POST_TABLE, ())
            .expect("Failed to create post table");

        temp_conn
            .execute(CREATE_STAR_TABLE, ())
            .expect("Failed to create star table");
        temp_conn
            .execute(CREATE_USER_TABLE, ())
            .expect("Failed to create user table");
        temp_conn
            .execute(CREATE_SHOUTS_TABLE, ())
            .expect("Failed to create shouts table");
        Arc::new(sqlite_pool)
    }
    pub fn from_pooled_conn(conn: PooledConnection<SqliteConnectionManager>) -> Self {
        Self { conn }
    }
    pub fn edit_comment(&self, id: CommentId, comment: &str) -> Result<(), rusqlite::Error> {
        let content = ammonia::clean(comment.trim());
        let mut statement = self.conn.prepare(UPDATE_COMMENT)?;

        statement.execute((content, id))?;

        Ok(())
    }

    pub fn delete_comment(&self, comment_id: CommentId) -> Result<(), rusqlite::Error> {
        let mut statement = self.conn.prepare(DELETE_COMMENT)?;

        statement.execute((comment_id,))?;

        Ok(())
    }

    pub fn get_comment_from_id(&self, id: i64) -> Result<Comment, rusqlite::Error> {
        let mut row = self
            .conn
            .prepare(GET_COMMENT_WITH_ID)
            .expect("Prepared statement for getting comment should be valid SQL");

        row.query_one((id,), Comment::from_row)
    }

    pub fn get_post_comments(&self, post_id: PostId) -> Result<Vec<Comment>, rusqlite::Error> {
        let mut comments_statement = self
            .conn
            .prepare(GET_COMMENTS_WITH_POST_ID)
            .expect("SQL statement for getting comments should be valid");

        let comment_rows = comments_statement.query_map((post_id,), Comment::from_row)?;

        Ok(comment_rows
            .flat_map(|result| result.ok())
            .collect::<Vec<_>>())
    }

    pub fn get_post_star_count(&self, post_id: PostId) -> Result<i32, rusqlite::Error> {
        let mut star_count_statement = self
            .conn
            .prepare(GET_STAR_COUNT_WITH_POST_ID)
            .expect("Star count SQL should be valid");
        star_count_statement
            .query_one((post_id,), |row| row.get(0))
            .map(|count: i32| count)
    }

    pub fn get_post_with_ident(&self, ident: &str) -> Result<Post, rusqlite::Error> {
        let mut row = self
            .conn
            .prepare(GET_POST_WITH_IDENT)
            .expect("Prepared statement for getting postId should be valid SQL");

        row.query_one((ident,), Post::from_row)
    }
    pub fn update_posts<S: AsRef<str>, I: IntoIterator<Item = S>>(&self, post_idents: I) {
        for ident in post_idents {
            let Ok(mut statement) = self.conn.prepare(INSERT_POST_WITH_IDENT) else {
                continue;
            };
            let _ = statement.execute((ident.as_ref(),));
        }
    }

    pub fn add_comment(
        &self,
        post_id: PostId,
        user_id: UserId,
        comment: &str,
    ) -> Result<(), rusqlite::Error> {
        let content = ammonia::clean(comment);
        let mut statement = self.conn.prepare(INSERT_COMMENT)?;

        statement.execute((post_id, user_id, content.trim()))?;
        Ok(())
    }

    pub fn star_post(&self, post_id: PostId, user_id: UserId) -> Result<(), rusqlite::Error> {
        let mut statement = self.conn.prepare(INSERT_STAR)?;
        statement.execute((user_id, post_id))?;
        Ok(())
    }
    pub fn is_post_starred_by(
        &self,
        post_id: PostId,
        user_id: UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut statement = self.conn.prepare(IS_STARRED_BY_USER_ID)?;
        Ok(statement
            .query_one((post_id, user_id), Star::from_row)
            .is_ok())
    }

    pub fn add_user(
        &self,
        display_name: &str,
        token: &str,
        ip: TruncatedIp,
    ) -> Result<(), rusqlite::Error> {
        let mut statement = self.conn.prepare(INSERT_USER)?;
        let mut rng = rand::rng();
        let color: u32 = rng.random();
        statement.execute((display_name.trim(), token.trim(), color, ip))?;
        Ok(())
    }

    pub fn get_user_from_token(&self, token: &str) -> Result<User, rusqlite::Error> {
        let mut statement = self.conn.prepare(GET_USER_BY_TOKEN)?;
        statement.query_one((token.trim(),), User::from_row)
    }

    pub fn get_shout_from_id(&self, shout_id: i64) -> Result<Shout, rusqlite::Error> {
        let mut row = self
            .conn
            .prepare(GET_SHOUT_WITH_ID)
            .expect("Prepared statement for getting shout should be valid SQL");

        row.query_one((shout_id,), Shout::from_row)
    }

    pub fn get_all_shouts(&self) -> Result<Vec<Shout>, rusqlite::Error> {
        let mut shouts_statement = self
            .conn
            .prepare(GET_ALL_SHOUTS)
            .expect("SQL statement for getting comments should be valid");

        let comment_rows = shouts_statement.query_map((), Shout::from_row)?;

        Ok(comment_rows
            .flat_map(|result| result.ok())
            .collect::<Vec<_>>())
    }

    pub fn add_shout(&self, user_id: UserId, content: &str) -> Result<(), rusqlite::Error> {
        let content = ammonia::clean(content.trim());
        let mut statement = self.conn.prepare(INSERT_SHOUT)?;
        statement.execute((user_id, content))?;
        Ok(())
    }
    pub fn edit_shout(&self, shout_id: ShoutId, content: &str) -> Result<(), rusqlite::Error> {
        let content = ammonia::clean(content.trim());
        let mut statement = self.conn.prepare(UPDATE_SHOUT)?;
        statement.execute((content, shout_id))?;

        Ok(())
    }
    pub fn delete_shout(&self, shout_id: ShoutId) -> Result<(), rusqlite::Error> {
        let mut statement = self.conn.prepare(DELETE_SHOUT)?;

        statement.execute((shout_id,))?;

        Ok(())
    }

    pub(crate) fn get_user_by_id(&self, user_id: UserId) -> Result<User, rusqlite::Error> {
        let mut row = self
            .conn
            .prepare(GET_USER_BY_ID)
            .expect("Prepared statement for getting user should be valid SQL");

        row.query_one((user_id,), User::from_row)
    }
}
