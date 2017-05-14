use iron::{Chain, Request, Response, IronResult};
use iron::{BeforeMiddleware, Handler};
use iron::status;
use iron_router::Router as IronRouter;
use iron_tera::{Template, TeraEngine};
use tera::Context;
use middleware::diesel::{DieselMiddlewareExt, DieselReqExt};

pub struct Router(Chain);

impl Router {
    pub fn new<T>(db: T) -> Self
        where T: 'static + BeforeMiddleware + DieselMiddlewareExt
    {
        let mut router = IronRouter::new();
        router.get("/", index::<T>, "index");

        let mut chain = Chain::new(router);
        chain.link_before(db);
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
fn index<T>(req: &mut Request) -> IronResult<Response>
    where T: DieselMiddlewareExt
{
    let _ = req.db_conn::<T::Conn>();
    Ok(Response::with((status::Ok, Template::new("index.html", Context::default()))))
}
