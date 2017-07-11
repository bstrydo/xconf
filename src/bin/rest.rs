extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};

#[derive(RustcEncodable, RustcDecodable)]
struct Message { value: String}

fn main() {
    let messages = Arc::new(Mutex::new(vec![Message { value: "Hello, World".to_string()}]));
    let messages_get_id = messages.clone();
    let messages_post = messages.clone();
    let mut router = Router::new();

    router.get("/", move |_: &mut Request| get(&messages.lock().unwrap()[..]), "get");
    router.get("/:id", move |request: &mut Request| get_id(request, &messages_get_id.lock().unwrap()[..]), "get_id");
    router.post("/", move |request: &mut Request| post(request, &mut messages_post.lock().unwrap() ), "post");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn get(messages: &[Message]) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, json::encode(&messages).unwrap())))
}

fn get_id(request : &mut Request, messages: &[Message]) -> IronResult<Response> {
    match request.extensions.get::<Router>() {
        Some(parms) => match parms.find("id") {
            Some(id) => match id.parse::<usize>() {
                Ok(id) if id < messages.len() => Ok(Response::with((status::Ok, json::encode(&messages[id]).unwrap()))),
                Ok(_) => Ok(Response::with((status::NotFound, "please pass a valid id") ) ),
                Err(_) => Ok(Response::with((status::BadRequest, "please pass a valid id") ) ),
            },
            None => Ok(Response::with((status::BadRequest, "Error: can't extract id ") ) ),
        },
        None => Ok(Response::with( (status::BadRequest, "Error: can't extract the parameters ") ) ),
    }
}

fn post(request : &mut Request, messages: &mut Vec<Message>) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    messages.push(json::decode(&payload).unwrap());
    Ok(Response::with((status::Ok, json::encode(&messages[messages.len()-1]).unwrap() ) ) )
}
