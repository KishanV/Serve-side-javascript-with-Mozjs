#[macro_use]
extern crate js;
extern crate libc; 
  
mod linedeer; 
use linedeer::serve::Context;
use linedeer::serve::Counter;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::{Handler, AroundMiddleware,Protocol};
 
struct Router {
	context:Context
}

impl Router{
	pub fn new() -> Router {
		Router {
			context:Context::new()
		} 
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {  
    	Ok(Response::with((status::Ok, ""))) 
    }
}

fn main() {
	//let mut router = Router::new();
	
    Iron::new(Router::new()).listen_with("127.0.0.1:3000",1,Protocol::Http,None).unwrap(); 
    println!("ok");
} 