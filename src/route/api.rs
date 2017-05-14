use iron::{Request, Response, IronResult, Handler};
use iron_router::Router as IronRouter;

pub struct Router(IronRouter);

impl Router {
    pub fn new() -> Router {
        Router(IronRouter::new())
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}
