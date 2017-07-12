extern crate serde;
extern crate serde_json;

use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Read;
use iron::{Handler, status, IronResult, Response, Request, AfterMiddleware};
use iron::headers::ContentType;
use database::Database;
use router::Router;
use model::Post;
use std::error::Error;
use uuid::Uuid;

use model;

macro_rules! try_handler {
    ( $e: expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };
    ( $e: expr, $error:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ( $e: expr) => { $e.lock().unwrap() }
}

macro_rules! get_http_param {
    ( $r: expr, $e: expr ) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(val) => val,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            }
            None => return Ok(Response::with(status::InternalServerError)),
        }
    }
}

pub struct Handlers {
    pub feed: FeedHandler,
    pub make_post: MakePostHandler,
    pub post: PostHandler,
    pub index: IndexHandler,
    pub script: ScriptHandler,
}

impl Handlers {
    pub fn new(database: Database) -> Handlers {
        let database = Arc::new(Mutex::new(database));
        Handlers {
            feed: FeedHandler::new(database.clone()),
            make_post: MakePostHandler::new(database.clone()),
            post: PostHandler::new(database.clone()),
            index: IndexHandler::new(database.clone()),
            script: ScriptHandler::new(database.clone()),
        }
    }
}

pub struct FeedHandler {
    database: Arc<Mutex<Database>>,
}

impl FeedHandler {
    fn new(database: Arc<Mutex<Database>>) -> FeedHandler {
        FeedHandler { database: database }
    }
}

impl Handler for FeedHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = try_handler!(serde_json::to_string(lock!(self.database).posts()));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct MakePostHandler {
    database: Arc<Mutex<Database>>,
}

impl MakePostHandler {
    fn new(database: Arc<Mutex<Database>>) -> MakePostHandler {
        MakePostHandler { database: database }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MakePost {
    author_handle: String,
    summary: String,
    content: String,
}

impl Handler for MakePostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        println!("{}", payload);
        let post: MakePost = match serde_json::from_str(&payload) {
            Ok(stuff) => stuff,
            Err(e) => {
                println!("encountered error {}", e);
                MakePost {
                    author_handle: String::from("def name"),
                    summary: String::from("def short"),
                    content: String::from("def long"),
                }
            }
        };

        println!(
            "author: {}\nsummart: {}\ncontent: {}",
            post.author_handle,
            post.summary,
            post.content
        );

        let post = Post::from_post(
            &post.summary,
            &post.content,
            &model::Author::new(&post.author_handle),
        );
        lock!(self.database).add_post(post);

        Ok(Response::with((status::Created, payload)))
    }
}

pub struct PostHandler {
    database: Arc<Mutex<Database>>,
}

impl PostHandler {
    fn new(database: Arc<Mutex<Database>>) -> PostHandler {
        PostHandler { database: database }
    }

    fn find_post(&self, id: &Uuid) -> Option<Post> {
        let locked = lock!(self.database);
        let mut iterator = locked.posts().iter();
        iterator.find(|post| post.uuid() == id).map(
            |post| post.clone(),
        )
    }
}

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref post_id = get_http_param!(req, "id");

        let id = try_handler!(Uuid::parse_str(post_id), status::BadRequest);

        if let Some(post) = self.find_post(&id) {
            let payload = try_handler!(serde_json::to_string(&post), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with((status::NotFound)))
        }
    }
}

pub struct IndexHandler {
    database: Arc<Mutex<Database>>,
}

impl IndexHandler {
    fn new(database: Arc<Mutex<Database>>) -> IndexHandler {
        IndexHandler { database: database }
    }
}

impl Handler for IndexHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut file = match File::open("static/index.html") {
            Ok(result) => result,
            Err(e) => panic!("Couldn't open file index!"),
        };

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let mut response = Response::with((status::Ok, contents));
        response.headers.set(ContentType::html());
        return Ok(response);
    }
}

pub struct ScriptHandler {
    database: Arc<Mutex<Database>>,
}

impl ScriptHandler {
    fn new(database: Arc<Mutex<Database>>) -> ScriptHandler {
        ScriptHandler { database: database }
    }
}

impl Handler for ScriptHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut file = match File::open("static/yavascript.js") {
            Ok(result) => result,
            Err(e) => panic!("Couldn't open file yavascript!"),
        };

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let mut response = Response::with((status::Ok, contents));
        response.headers.set(ContentType::plaintext());
        return Ok(response);
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        match res.headers.get::<ContentType>() {
            None => res.headers.set(ContentType::json()),
            _ => (),
        }
        Ok(res)
    }
}
