use iron_router::Router;

pub struct Middleware;

impl Middleware {
    pub fn new() -> Router {
        Router::new()
    }
}
