#[macro_use]
extern crate js;
extern crate libc; 
  
mod linedeer; 
use linedeer::serve::Context;
use linedeer::serve::Counter;

 
extern crate hyper;

use hyper::{Get, Post, StatusCode, RequestUri, Decoder, Encoder, HttpStream, Next};
use hyper::header::ContentLength;
use hyper::server::{Server, Handler, Request, Response,HttpListener}; 
static PHRASE: &'static [u8] = b"Hello World!";

struct Router {
	context:Context,
	data:String
}

impl Router{
	pub fn new() -> Router {
		Router {
			context:Context::new(),
			data:String::new()
		} 
    }
} 
 
impl Handler<HttpStream> for Router {
    fn on_request(&mut self, req: Request<HttpStream>) -> Next { 
        match *req.uri() {
	            RequestUri::AbsolutePath { ref path, .. } =>  {
		            //println!("Got {:?}",path);
		            //println!("{}",self.context.feed(path));
		            //self.data
		            self.data.push_str(path);
	            } 
             ,
            _ => () 
        };
        Next::write()
    }
    
    fn on_request_readable(&mut self, _: &mut Decoder<HttpStream>) -> Next {
        Next::write()
    }
    fn on_response(&mut self, response: &mut Response) -> Next {
        use hyper::header::ContentLength; 
        //println!("::std::thread::park");
         //::std::thread::park();
        //let mut data = self.context.feed(self.data.as_str());
        self.data.clear();
       /* for x in 0..1000000 {
			self.data.push_str("okokokok okok");
		};*/
        //self.data.push_str(data.as_str());
        self.data.push_str("okokokok okok");
        response.headers_mut().set(ContentLength(self.data.len() as u64));
        Next::write()
    }
    fn on_response_writable(&mut self, encoder: &mut Encoder<HttpStream>) -> Next { 
        let n = encoder.write(self.data.as_bytes()).unwrap();
        //debug_assert_eq!(n, PHRASE.len());
        Next::end()
    }
}

fn main() {
	/* let mut router = Router::new();
	 let listener = HttpListener::bind(&"127.0.0.1:3000".parse().unwrap()).unwrap();
	 Server::new(listener).handle(|_| Router::new()).unwrap(); */
	
	 let listener = HttpListener::bind(&"127.0.0.1:3000".parse().unwrap()).unwrap();
     let mut handles = Vec::new();
     for _ in 0..1 {
        let listener = listener.try_clone().unwrap();
        handles.push(::std::thread::spawn(move || {
        	 Server::new(listener).handle(|_| Router::new()).unwrap();
        }));
     }
     println!("Listening on http://127.0.0.1:3000");

     for handle in handles {
         handle.join().unwrap();
     }
} 