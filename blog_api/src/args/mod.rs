//#[derive(Debug, Parser)]
//pub struct Settings {
//    #[arg(
//        help = "The path where the database resides or will be created",
//        short = 'd',
//        long,
//        default_value = "./comments.sqlite"
//    )]
//    database_path: PathBuf,
//    #[arg(
//        help = "The path where the tab title splashes will be read from",
//        short = 's',
//        long,
//        default_value = "../splashes/splashes.txt"
//    )]
//    splashes_path: PathBuf,
//    #[arg(
//        help = "The port the server will listen on",
//        short = 'p',
//        long,
//        default_value_t = 3000
//    )]
//    listen_port: u16,
//    #[arg(
//        help = "The url where the list of posts will be fetched from. If empty, the post list will not be updated automatically",
//        short = 'j',
//        long,
//        default_value = ""
//    )]
//    json_posts_url: String,
//    #[arg(
//        help = "The interval at which the post list is updated from the url",
//        short = 'u',
//        long,
//        default_value_t = 300
//    )]
//    post_list_update_interval_secs: u32,
//
//    #[arg(
//        help = "The window for rate limiting the getUser endpoint",
//        short = 'r',
//        long,
//        default_value = "300"
//    )]
//    rate_limit_window_secs: u32,
//    #[arg(
//        help = "The interval at which rate limit entries are cleaned up",
//        short = 'c',
//        long,
//        default_value = "300"
//    )]
//    rate_limit_cleanup_interval_secs: u32,
//    #[arg(
//        help = "The number of allowed hits to the getUser endpoint before rate limiting",
//        short = 'l',
//        long,
//        default_value = "10"
//    )]
//    rate_limit: u32,
//}

use std::path::PathBuf;

struct Settings {
    database_path: PathBuf,
    splash_file_path: PathBuf,
    listen_port: u16,
}

fn parse_args() -> Result<Settings, ()> {
    let mut args = std::env::args();

    let mut db_path = None;
    let mut splash_file_path = None;
    let mut listen_port = None;

    loop {
        if let Some(arg) = args.next() {
            if let Some(stripped) = arg.strip_prefix("--") {
            } else if let Some(stripped) = arg.strip_prefix("-") {
            }
        }
    }
}
