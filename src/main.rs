#[macro_use]
extern crate js;
extern crate libc; 
  
mod linedeer; 
use linedeer::serve::Context;
use linedeer::serve::Counter;

extern crate tiny_http; 
use std::sync::Arc;
use std::thread;

fn main(){
	use tiny_http::{Server, Response};
	let server = Arc::new(tiny_http::Server::http("0.0.0.0:3000").unwrap());
    println!("Now listening on port 3000");  

    let mut handles = Vec::new();

    for _ in 0 .. 8 {
        let server = server.clone(); 
        handles.push(thread::spawn(move || {
    			let context = Context::new();
	            for request in server.incoming_requests() {
	                //let response = tiny_http::Response::from_string("hello world".to_string());
	                let (data,contentType) =  context.feed(request.url());
	                let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &contentType[..]).unwrap();
	                let mut response = Response::from_string(data);
	                response.add_header(header);
	               // thread::spawn(move || {
		                let _ = request.respond(response);
	               // }); 
	            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
/*fn main(){
	use tiny_http::{Server, Response};
	let router = Router::new();
	let server = Server::http("0.0.0.0:8000").unwrap();
	
	for request in server.incoming_requests() {
	    println!("received request! url: {:?} ",  request.url()); 
	    let mut data = router.context.feed(request.url());
	    let response = Response::from_string(data);
	    request.respond(response);
	}
}*/