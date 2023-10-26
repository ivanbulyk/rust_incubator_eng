#![allow(unused)]
use std::{borrow::Cow, env};
pub struct Config<'a> {
    pub file_path: Cow<'a, str>,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self {
            file_path: Cow::Borrowed("/etc/app/app.conf"),
        }
    }
}

impl Config<'_> {
    pub fn build() -> Config<'static> {
        let args: Vec<_> = std::env::args().collect();
        if args.len() < 3 && args.len() > 1 {
            if args[1] == "--conf" {
                eprintln!("the path to conf file is not specified\n");
            } else {
                eprintln!("--conf command line flag is not specified\n");
            }
        }

        if args.len() >= 3 {
            if args[1] == "--conf" {
                return Config {
                    file_path: args[2].clone().into(),
                };
            } else {
                eprintln!("invalid order command line arguments is specified\n");
            }
        }

        let success_case = env::var("APP_CONF").is_ok();

        if let Ok(app_conf) = env::var("APP_CONF") {
            if app_conf != "" {
                return Config {
                    file_path: Cow::Owned(app_conf),
                };
            }
        }

        Default::default()
    }
}
