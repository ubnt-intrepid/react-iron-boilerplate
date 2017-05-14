extern crate dotenv;
extern crate tera;
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate iron;
extern crate mount as iron_mount;
extern crate router as iron_router;
extern crate staticfile as iron_staticfile;
extern crate iron_tera;

mod config;
mod middleware;
mod route;

use dotenv::dotenv;
use iron::prelude::*;
use iron_mount::Mount;
use iron_staticfile::Static;

use config::AppConfig;
use middleware::diesel::DieselSqliteMiddleware;
use route::{WebRouter, ApiRouter};

fn main() {
    dotenv().ok();
    let config = AppConfig::from_env_vars();

    let db = DieselSqliteMiddleware::new("app.db");

    let mut mount = Mount::new();
    mount.mount("/", WebRouter::new(db));
    mount.mount("/api/v1/", ApiRouter::new());
    mount.mount("/assets/", Static::new("assets"));

    println!("Serving on {}", config.bind_address);
    println!("Press Ctrl+C to exit");
    Iron::new(mount)
        .http(&config.bind_address)
        .unwrap();
}
