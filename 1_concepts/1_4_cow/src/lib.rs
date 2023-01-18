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
    pub fn build(args: &[String]) -> Config {
        if args.len() < 3 && args.len() > 1 {
            if args[1] == "--conf" {
                eprintln!("the path to conf file is not specified\n");
            } else {
                eprintln!("--conf command line flag is not specified\n");
            }
        }

        let mut res = Config::default();

        if args.len() >= 3 {
            if args[1] == "--conf" {
                res.file_path = Cow::Borrowed(&args[2]);
                return res;
            } else {
                eprintln!("invalid order command line arguments is specified\n");
            }
        }

        let success_case = env::var("APP_CONF").is_ok();

        if success_case {
            let app_conf = std::env::var("APP_CONF").expect("APP_CONF must be properly set.\n");
            if app_conf != "" {
                res.file_path = Cow::Owned(app_conf);
                return res;
            }
        }

        res
    }
}
