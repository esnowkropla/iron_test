#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;
extern crate logger;
extern crate staticfile;

extern crate env_logger;
extern crate serde;
extern crate chrono;
extern crate uuid;

mod model;
mod database;
mod handlers;

use std::path::Path;

use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use staticfile::Static;

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(None);

    let database = Database::new();

    let handlers = Handlers::new(database);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/feed", handlers.feed, "feed");
    router.post("/post", handlers.make_post, "make_post");
    router.get("/post/:id", handlers.post, "post");
    router.get("/", Static::new(Path::new("static/index.html")), "home");
    router.get("/*", Static::new(Path::new("static/")), "static");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
