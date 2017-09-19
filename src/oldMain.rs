#[macro_use]
extern crate js;
extern crate libc; 

mod linedeer;
extern crate rotor;
extern crate rotor_http;

use std::time::Duration;  
use rotor::{Scope, Time};
use rotor_http::server::{RecvMode, Server, Head, Response, Fsm};
use rotor::mio::tcp::TcpListener;

use linedeer::serve::Context;
use linedeer::serve::Counter;
 

enum HelloWorld { 
    Path(String) 
}

/*fn send_string(res: &mut Response, data: &[u8]) {
    res.status(200, "OK");
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}*/

impl Server for HelloWorld {
    type Seed = ();
    type Context = linedeer::serve::Context;
    
    fn headers_received(_seed: (), head: Head, _res: &mut Response, scope: &mut Scope<Context>) -> Option<(Self, RecvMode, Time)>
    {
        use self::HelloWorld::*;  
        let mut string = String::new();
        string.push_str(head.path);
        //println!("yoyo : {:?}",scope.unw);  
        Some((Path(string), RecvMode::Buffered(8024),scope.now() + Duration::new(10, 0)))
    }
    
    fn request_received(self, _data: &[u8], res: &mut Response, scope: &mut Scope<Context>) -> Option<Self>
    {
        use self::HelloWorld::*; 
        match self {
            Path(str) => {
            	   scope.feed(res,str);
            }
        }
        None
    }
    
    fn request_chunk(self, _chunk: &[u8], _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self>
    {
        unreachable!();
    }

    /// End of request body, only for Progressive requests
    fn request_end(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self>
    {
        unreachable!();
    }

    fn timeout(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<(Self, Time)>
    {
        unimplemented!();
    }
    
    fn wakeup(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self>
    {
        unimplemented!();
    }
}

fn main() {
	 
     println!("Starting http server on http://127.0.0.1:3000/");
   
     let event_loop = rotor::Loop::new(&rotor::Config::new()).unwrap();  
	 let contexct = Context::new(); 
     let mut loop_inst = event_loop.instantiate(contexct);
     loop_inst.add_machine_with(|scope| {
     	let lst = TcpListener::bind(&"127.0.0.1:3000".parse().unwrap()).unwrap();
	     Fsm::<HelloWorld, _>::new(lst, (), scope)
	 }).unwrap(); 
	 loop_inst.run();
	 
    
   /* println!("Press Enter to exit.");
    use std::io; 
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
	    Ok(n) => { 
	        //println!("{}", input);
	    }
	    Err(error) => println!("error: {}", error),
	}*/
}