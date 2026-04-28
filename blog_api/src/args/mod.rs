use std::{ffi::OsString, path::PathBuf};

use quick_error::quick_error;

#[derive(Debug, PartialEq, Eq)]
pub struct Settings {
    pub database_path: PathBuf,
    pub splash_file_path: PathBuf,
    pub listen_port: u16,
    pub post_list_url: Option<String>,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SettingsBuilder {
    database_path: Option<PathBuf>,
    splash_file_path: Option<PathBuf>,
    listen_port: Option<u16>,
    post_list_url: Option<String>,
}

impl SettingsBuilder {
    pub fn set_sqlite_db_path(&mut self, path: PathBuf) {
        self.database_path = Some(path);
    }
    pub fn set_splash_file_path(&mut self, path: PathBuf) {
        self.splash_file_path = Some(path);
    }
    pub fn set_listen_port(&mut self, port: u16) {
        self.listen_port = Some(port);
    }
    pub fn set_post_list_url(&mut self, url: String) {
        self.post_list_url = Some(url);
    }
    pub fn build(self) -> Settings {
        const DEFAULT_DB_PATH: &str = "./db.sqlite";
        const DEFAULT_SPLASH_PATH: &str = "./splashes.txt";
        const DEFAULT_PORT: u16 = 3000;

        Settings {
            database_path: self.database_path.unwrap_or(DEFAULT_DB_PATH.into()),
            splash_file_path: self.splash_file_path.unwrap_or(DEFAULT_SPLASH_PATH.into()),
            listen_port: self.listen_port.unwrap_or(DEFAULT_PORT),
            post_list_url: self.post_list_url,
        }
    }
}

quick_error! {
    #[derive(Debug,PartialEq, Eq)]
    pub enum ArgumentParseError{
        UnknownFlag(flag: OsString){
            display("Unknown flag: {flag:?}")
        }
        InvalidValue(value: OsString){
            display("Invalid value: {value:?}")
        }
        MissingValue(flag: OsString){
            display("Missing value for flag: {flag:?}")
        }
    }
}

pub fn parse_settings_from_args(
    mut args: impl Iterator<Item = std::ffi::OsString>,
) -> Result<Settings, ArgumentParseError> {
    let mut settings_builder = Settings::builder();

    while let Some(arg) = args.next() {
        let Some(data) = args.next() else {
            return Err(ArgumentParseError::MissingValue(arg));
        };

        match arg {
            arg if arg == "-d" || arg == "--sqlite-db-path" => {
                settings_builder.set_sqlite_db_path(PathBuf::from(data));
            }
            arg if arg == "-p" || arg == "--listen-port" => {
                let port: u16 = data
                    .to_string_lossy()
                    .parse()
                    .map_err(|_| ArgumentParseError::InvalidValue(data))?;
                settings_builder.set_listen_port(port);
            }
            arg if arg == "-s" || arg == "--splash-file-path" => {
                settings_builder.set_splash_file_path(PathBuf::from(data));
            }
            arg if arg == "-j" || arg == "--posts-list-url" => {
                settings_builder.set_post_list_url(
                    data.into_string()
                        .map_err(ArgumentParseError::InvalidValue)?,
                );
            }

            x => return Err(ArgumentParseError::UnknownFlag(x)),
        }
    }

    Ok(settings_builder.build())
}
#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn short_args() {
        let args: [OsString; 6] = [
            "-d".into(),
            "./db.sqlite".into(),
            "-p".into(),
            "3000".into(),
            "-s".into(),
            "./splash.txt".into(),
        ];
        let settings = parse_settings_from_args(args.into_iter()).expect("Failed to parse args");

        assert_eq!(
            settings,
            Settings {
                database_path: PathBuf::from("./db.sqlite"),
                splash_file_path: PathBuf::from("./splash.txt"),
                listen_port: 3000,
                post_list_url: None
            }
        );
    }
    #[test]
    fn long_args() {
        let args: [OsString; 6] = [
            "--sqlite-db-path".into(),
            "./db.sqlite".into(),
            "--listen-port".into(),
            "3000".into(),
            "--splash-file-path".into(),
            "./splash.txt".into(),
        ];
        let settings = parse_settings_from_args(args.into_iter()).expect("Failed to parse args");

        assert_eq!(
            settings,
            Settings {
                database_path: PathBuf::from("./db.sqlite"),
                splash_file_path: PathBuf::from("./splash.txt"),
                listen_port: 3000,
                post_list_url: None
            }
        );
    }
    #[test]
    fn bad_flag() {
        let args: [OsString; 2] = ["--listen".into(), "3000".into()];
        let result = parse_settings_from_args(args.into_iter());

        assert_eq!(
            result,
            Err(ArgumentParseError::UnknownFlag("--listen".into()))
        );
    }
    #[test]
    fn bad_value() {
        let args: [OsString; 2] = ["--listen-port".into(), "3000s".into()];
        let result = parse_settings_from_args(args.into_iter());

        assert_eq!(
            result,
            Err(ArgumentParseError::InvalidValue("3000s".into()))
        );
    }
    #[test]
    fn missing_value() {
        let args: [OsString; 1] = ["--listen-port".into()];
        let result = parse_settings_from_args(args.into_iter());

        assert_eq!(
            result,
            Err(ArgumentParseError::MissingValue("--listen-port".into()))
        );
    }
    #[test]
    fn default_vals() {
        let args: [OsString; 0] = [];
        const DEFAULT_DB_PATH: &str = "./db.sqlite";
        const DEFAULT_SPLASH_PATH: &str = "./splashes.txt";
        const DEFAULT_PORT: u16 = 3000;

        let settings = parse_settings_from_args(args.into_iter()).expect("Failed to parse args");

        assert_eq!(
            settings,
            Settings {
                database_path: DEFAULT_DB_PATH.into(),
                splash_file_path: DEFAULT_SPLASH_PATH.into(),
                listen_port: DEFAULT_PORT,
                post_list_url: None
            }
        );
    }
}
