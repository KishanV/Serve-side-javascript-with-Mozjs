use std::collections::HashMap;  
use std::string::String;
use std::path::Path;
use linedeer::script::Instant;
use linedeer::script::My;
use js::jsval::PrivateValue;


pub struct Context{
	 pub counter: usize,
	 pub extension:HashMap<String,String>,
	 pub Js:Instant
}

impl Context {
    pub fn new() -> Context {
        let mut context = Context {
	        counter: 0,
	        extension: HashMap::new(),
	        Js:Instant::new()
	    };
        context.init(); 
        //context.Js.print(("print('okokok')")); 
        context
    }
} 


pub trait Counter {
    fn init(&mut self) ; 
    fn feed(&self,req: &str) -> (String,&str);
    fn increment(&mut self);
    fn get(&self) -> usize;  
}

impl Counter for Context {
    //fn new(&mut self) { self.counter += 1; }
     fn init(&mut self)  {
     	 //self.extantion.insert(String::from("html"), String::from("text/html"));
     	 self.extension.insert(String::from("html"), String::from("text/html"));
     	 self.extension.insert(String::from("js"), String::from("text/javascript"));
     	 self.extension.insert(String::from("ssjs"), String::from("ssjs/script"));
     	 self.extension.insert(String::from("_"), String::from("text/html"));  
     }
     
     fn feed(&self,req: &str) -> (String,&str) {  
		 /*let map: HashMap<&str, &str> = {
	        let mut m = HashMap::new();
	        m.insert("exe", "foo");
	        m.insert("exe", "bar");
	        m.insert("exe", "baz");
	        m
		    };*/
		 //let arg = String::from("var ok = print('While http reuest..!.');");
		 //self.Js.print(&arg); 
		 
		   
		 let urlPath = req; 
		 
		 let dir =  "./www/".to_string() + &urlPath; 
		 let file = Path::new(&dir);
		 let exist = file.exists();
		 
		 //by default
		 let mut contentType = "text/html"; 
		 
		 let mut forSend;
		 
		 match exist {
		 	true => {
		 		let isDir = file.is_dir();
		 		 match isDir {
		 		 	true => {
		 		 		//contentType = "text/html";
		 		 		//println!("is Dir....!");
		 		 		forSend = feed_dir(file.as_ref(),urlPath);
		 		 	},
		 		 	false => { 
		 		 		forSend = feed_file(file.as_ref());
		 		 		match file.extension() {
					    	Some(str) => { 
					    		  match str.to_str().unwrap() {
					    		  	"ssjs" => {
					    		  		//self.Js.evaluate_script(forSend.as_str());
					    		  		 forSend = self.Js.loadSrc(forSend.as_str()); 
					    		  		 
					    		  		/*for x in 0..100000 {
										    forSend.push_str("okokokok okok");
										};*/
					    		  		 
					    		  		unsafe{
					    		  			use std;
					    		  			//println!("C : reguler {:?}", (&(*self.Js.bx) as *const  My) as *mut std::os::raw::c_void);
					    		  			//self.Js.string.push_str(".o.");
					    		  		} 
					    		  		let fileName = file.file_name().unwrap().to_str().unwrap();
					    		  		let len = &fileName.len() - 5;
						    		  	let Name = &fileName[0..len];
						    		  	let fountChar = Name.find(".");
						    		  	match Name.find(".") {
						    		  		Some(num) => {
						    		  			//"text/html"
						    		  			contentType =  "text/html"; 
						    		  			if num == &Name.len() - 1 || num == 0{
							    		  			//println!("DOT {:?}",num);
						    		  			}else { 
							    		  	      //println!("Extantion {:?}",&Name[(num+1)..*(&Name.len())]);
							    		  	      //println!("FILENAME {:?}",Name);
							    		  	        let gotExtantion = self.extension.get(&String::from(&Name[(num+1)..*(&Name.len())]));
									    		  	match gotExtantion {
									    		  		Some(str) => {
									    		  			//println!("gotExtantion { }",str);
									    		  			contentType = str;
									    		  		},
									    		  		_ => () ,
									    		  	}
						    		  			} 
						    		  		},
						    		  		None => ()
						    		  	}	 
						    		  	
						    		  	//"text/html"
					    		  	},
					    		  	_ =>  {
						    		  	let gotExtantion = self.extension.get(&String::from(str.to_str().unwrap()));
						    		  	match gotExtantion {
						    		  		Some(str) => {
						    		  			//println!("gotExtantion { }",str);
						    		  			contentType = str;
						    		  		},
						    		  		_ => () ,
						    		  	}
						    		  	
					    		  	}//"text/html",;
					    		  }
					    	},
					    	None => ()
					    };
		 		 		//println!("File Extension.....! {:?}",file.extension());
				 		/*println!("File Extension.....! {}",{match file.extension() {
				 			Some(n) => n.to_str().unwrap(),
				 			_ => "_",
				 		}});*/ 
				 	}
		 		 }
		 	},
		 	false => {
		 		//println!("404 NOt found ok....!");
		 		//contentType = "text/html";
		 		forSend = feed404();
		 	},
		 };
		 (forSend,contentType)
		//self.Js.printF(&arg); 
		//self.Js.print("Kishan Devani");
	    /*res.status(200, "OK");   
	    res.add_length(forSend.len() as u64).unwrap();
	    res.add_header("Content-Type",contentType.as_bytes());  
	    res.done_headers().unwrap();
	    res.write_body(forSend.as_bytes());
	    res.done();*/
	    //println!("{:?}",string); 
	}
     
    fn increment(&mut self) { self.counter += 1; }
    fn get(&self) -> usize { self.counter }
}  

fn feed_dir(pathUrl:&Path,url:&str) -> String{
	use std::error::Error;
	use std::fs::File;
	use std::fs::DirEntry;
	use std::fs;
	//println!("inspection {:?}",pathUrl);
	let mut string = String::new();
	string.push_str("<html><head><title>Rust Server.</title></head><body>");
	match fs::read_dir(pathUrl) {
	    Err(why) => println!("!Erro in Dir responce{:?}", why.kind()),
	    Ok(paths) => for path in paths { 
	    	 //println!("{}", url);
	    	 let name = path.unwrap().path(); 
	    	 string.push_str("<a href='");
	    	 match url  {
	    	 	"/" => {
	    	 		//string.push_str("./");
	    	 	},
	    	 	_ => {
	    	 		//string.push_str(".");
	    	 		string.push_str(url);
	    	 		string.push_str("/");
	    	 	},
	    	 }
	    	 //string.push_str("/");
	    	 string.push_str(name.file_name().unwrap().to_str().unwrap()); 
	    	 string.push_str("'>"); 
	    	 string.push_str(name.file_name().unwrap().to_str().unwrap());
	    	 let file =  Path::new(&name);
	    	 if file.is_dir() == true {
	    	 	 string.push_str(" (dir)");
	    	 }else{ 
	    	 	//println!("{ }", fs::metadata(file).unwrap().len());
	    	 	string.push_str(" (");
	    	 	string.push_str(fs::metadata(file).unwrap().len().to_string().as_str());
	    	 	string.push_str(")");
	    	 }
	    	
	    	 string.push_str("</a><br>");
	         //println!("{:?}", name.file_name());
	    },
	}
	string.push_str("</body></html>");
	string
}

fn feed404() -> String{
	let data = String::from("404");
	data
}

fn feed_file(path:&Path) -> String{
	use std::fs::File;
	use std::io::Read; 
	let mut s = String::new();   
	match  File::open(path) { 
        Err(why) => {
	        s.push_str("File not found");
        },
        Ok(mut file) => {
        	file.read_to_string(&mut s);
        },
    }
	s
}