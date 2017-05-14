use iron::{Chain, Request, Response, IronResult};
use iron::Handler;
use iron::status;
use iron_router::Router as IronRouter;
use iron_tera::{Template, TeraEngine};
use tera::Context;

pub struct Router(Chain);

impl Router {
    pub fn new() -> Self {
        let mut router = IronRouter::new();
        router.get("/", index, "index");

        let mut chain = Chain::new(router);
        chain.link_after(TeraEngine::new("templates/**/*"));

        Router(chain)
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}


// GET '/'
fn index(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Template::new("index.html", Context::default()))))
}
