use std::env;
use std::path::Path;

use parking_lot::Mutex;
use rusqlite::Connection;
use simple_config_parser::Config as Cfg;

pub struct App {
    pub db: Mutex<Connection>,
    pub cfg: Config,
}

pub struct Config {
    // Host
    pub host: String,
    pub port: u16,
    pub database: String,

    // Admin
    pub admin_login: String,
}

impl App {
    pub fn new() -> Self {
        let cfg = Config::load(
            env::args()
                .nth(1)
                .unwrap_or_else(|| "data/config.cfg".to_owned()),
        );
        let mut db = Connection::open(&cfg.database).unwrap();

        db.pragma_update(None, "journal_mode", "WAL").unwrap();
        db.pragma_update(None, "synchronous", "NORMAL").unwrap();
        let trans = db.transaction().unwrap();

        // Init tables
        for i in [
            include_str!("./sql/create_apps.sql"),
            include_str!("./sql/create_versions.sql"),
        ] {
            trans.execute(i, []).unwrap();
        }
        trans.commit().unwrap();

        App {
            db: Mutex::new(db),
            cfg,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        // this is what i get for using afire beta
        unsafe { std::mem::zeroed() }
    }
}

impl Config {
    fn load<T: AsRef<Path>>(file: T) -> Self {
        let cfg = Cfg::new().file(file).unwrap();

        Config {
            // Host
            host: cfg.get("host").unwrap(),
            port: cfg.get("port").unwrap(),
            database: cfg.get("database").unwrap(),

            // Admin
            admin_login: cfg.get("login").unwrap(),
        }
    }
}
