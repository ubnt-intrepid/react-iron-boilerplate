extern crate dotenv;
extern crate tera;
extern crate iron;
extern crate mount as iron_mount;
extern crate router as iron_router;
extern crate staticfile as iron_staticfile;
extern crate iron_tera;

mod config;
mod route;

use dotenv::dotenv;
use iron::prelude::*;
use iron_mount::Mount;
use iron_staticfile::Static;

use config::AppConfig;


fn main() {
    dotenv().ok();
    let config = AppConfig::from_env_vars();

    let mut mount = Mount::new();
    mount.mount("/", route::web::Middleware::new());
    mount.mount("/api/v1/", route::api::Middleware::new());
    mount.mount("/assets/", Static::new("assets"));

    println!("Serving on {}", config.bind_address);
    println!("Press Ctrl+C to exit");
    Iron::new(mount)
        .http(&config.bind_address)
        .unwrap();
}
