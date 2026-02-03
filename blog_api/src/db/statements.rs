//Users
pub const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS users(
        id INTEGER NOT NULL,
        display_name TEXT NOT NULL,
        token TEXT NOT NULL UNIQUE,
        color INTEGER NOT NULL,
        PRIMARY KEY (id,display_name)
        UNIQUE (display_name COLLATE NOCASE)
    )";
pub const GET_USER_BY_TOKEN: &str = "SELECT * FROM users WHERE token=?";
pub const GET_USER_BY_ID: &str = "SELECT * FROM users WHERE id=?";
pub const INSERT_USER: &str = "INSERT INTO users (id,display_name,token,color) VALUES (NULL,?,?,?)";

//Post SQL
pub const CREATE_POST_TABLE: &str = "CREATE TABLE IF NOT EXISTS posts (
        id INTEGER NOT NULL PRIMARY KEY,
        identifier TEXT NOT NULL UNIQUE
    )";
pub const GET_POST_WITH_IDENT: &str = "SELECT * FROM posts WHERE identifier=?";
pub const INSERT_POST_WITH_IDENT: &str =
    "INSERT OR IGNORE INTO posts (id,identifier) VALUES (NULL,?)";

//Star SQL
pub const GET_STAR_COUNT_WITH_POST_ID: &str = "SELECT COUNT(*) FROM stars WHERE post_id=?";
pub const INSERT_STAR: &str = "INSERT INTO stars (user_id,post_id) VALUES (?,?)";
pub const CREATE_STAR_TABLE: &str = "CREATE TABLE IF NOT EXISTS stars(
        user_id INTEGER NOT NULL,
        post_id INTEGER NOT NULL,
        PRIMARY KEY (user_id,post_id)
        FOREIGN KEY (post_id) REFERENCES posts(id)
        FOREIGN KEY (user_id) REFERENCES users(id)
    )";
pub const IS_STARRED_BY_USER_ID: &str = "SELECT * FROM stars WHERE post_id=? AND user_id=?";

//Comments SQL
pub const CREATE_COMMENT_TABLE: &str = "CREATE TABLE IF NOT EXISTS comments (
        id INTEGER NOT NULL PRIMARY KEY,
        posted_under INTEGER NOT NULL,
        user_id INTEGER NOT NULL,
        content TEXT NOT NULL,
        edited INTEGER NOT NULL DEFAULT 0,
        created DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (posted_under) REFERENCES posts(id)
        FOREIGN KEY (user_id) REFERENCES users(id)
    )";
pub const INSERT_COMMENT: &str =
    "INSERT INTO comments (id,posted_under,user_id,display_name,content) VALUES (NULL,?,?,?,?)";
pub const GET_COMMENTS_WITH_POST_ID: &str = "SELECT * FROM comments WHERE posted_under=?";
pub const UPDATE_COMMENT: &str =
    "UPDATE comments SET content=?,edited=1,created=CURRENT_TIMESTAMP WHERE id=?";
pub const DELETE_COMMENT: &str = "DELETE FROM comments WHERE id=?";
pub const GET_COMMENT_WITH_ID: &str = "SELECT * FROM comments WHERE (id=?)";

//Shouts SQL
pub const CREATE_SHOUTS_TABLE: &str = "CREATE TABLE IF NOT EXISTS shouts (
        id INTEGER NOT NULL PRIMARY KEY,
        user_id INTEGER NOT NULL,
        content TEXT NOT NULL,
        edited INTEGER NOT NULL DEFAULT 0,
        created DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id)
    )";
pub const GET_SHOUT_WITH_ID: &str = "SELECT * FROM shouts WHERE (id=?)";
pub const GET_ALL_SHOUTS: &str = "SELECT * FROM shouts";
pub const INSERT_SHOUT: &str = "INSERT INTO shouts (id,user_id,content) VALUES (NULL,?,?)";
pub const UPDATE_SHOUT: &str =
    "UPDATE shouts SET content=?,edited=1,created=CURRENT_TIMESTAMP WHERE id=?";
pub const DELETE_SHOUT: &str = "DELETE FROM shouts WHERE id=?";
pub const GET_SHOUTS_BEFORE_ID: &str = "SELECT * FROM shouts WHERE (id<?)";
