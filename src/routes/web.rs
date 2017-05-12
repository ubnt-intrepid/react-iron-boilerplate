use iron::{Chain, Request, Response, IronResult};
use iron::Handler;
use iron::status;
use iron_router::Router;
use iron_tera::{Template, TeraEngine};
use tera::Context;

pub struct Middleware(Chain);

impl Middleware {
    pub fn new() -> Self {
        let mut router = Router::new();
        router.get("/", index, stringify!(index));
        router.get("/login", login, stringify!(login));

        let mut chain = Chain::new(router);
        chain.link_after(TeraEngine::new("templates/**/*"));

        Middleware(chain)
    }
}

impl Handler for Middleware {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}


// GET '/'
fn index(_req: &mut Request) -> IronResult<Response> {
    Ok(render(Context::new(), "index", "Index page"))
}

// GET '/login'
fn login(_req: &mut Request) -> IronResult<Response> {
    Ok(render(Context::new(), "login", "Log In"))
}


fn render(mut context: Context, name: &str, title: &str) -> Response {
    context.add("_title", &title);
    context.add("_name", &name);
    Response::with((status::Ok, Template::new("index.html", context)))
}
