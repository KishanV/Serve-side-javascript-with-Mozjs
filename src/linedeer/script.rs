extern crate js;
extern crate libc;

use js::jsapi::CallArgs;
use js::jsapi::CompartmentOptions;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JSContext; 
use js::jsapi::JSRuntime; 
use js::jsapi::JS_DefineFunction;
use js::jsapi::JS_EncodeStringToUTF8;
use js::jsapi::JS_NewGlobalObject;
use js::jsapi::JS_ReportError;
use js::jsapi::OnNewGlobalHookOption;
use js::jsapi::Value;
use js::jsval::UndefinedValue;
use js::jsapi::JS_AtomizeAndPinString;
use js::jsval::StringValue;
use js::jsval::Int32Value;
use js::jsapi::JSString;
use js::jsapi::MutableHandle;
use js::jsapi::Handle;
use js::jsapi::JSObject;
use js::jsapi::JS_ErrorFromException;
use js::jsapi::HandleObject;
use js::jsapi::GetWarningReporter;
use js::jsapi::JS_IsExceptionPending;
use js::jsapi::JS_GetPendingException;
use js::jsapi::JS_ClearPendingException;
use js::jsapi::JS_SetProperty;
use js::jsval::PrivateValue;
use js::jsapi::{JS_SetPrivate,JS_GetPrivate,JS_GetGlobalForObject,JS_GC,JS_SetOffthreadIonCompilationEnabled,JS_GetCurrentThread,CurrentThreadCanAccessRuntime};

use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS};
use std::ffi;
use std::ffi::CStr;
use std::ptr;
use std::str;
use std::string::String;
use std::slice::from_raw_parts;
use std;

 
pub struct My {
	 pub len:String 
}

pub struct Instant {
	 pub runtime:Runtime,
	 pub string:String,
	 pub context:*mut JSContext,
	 pub global:*mut JSObject,
	 pub bx:Box<My> 
}

 

impl Instant {
	pub fn new() -> Instant{ 
		let runtime = Runtime::new();
		let JScontext = runtime.cx();
		let h_option = OnNewGlobalHookOption::FireOnNewGlobalHook;
	    let c_option = CompartmentOptions::default();
	    let mut dataStr = String::from("");
	    let x = 10;
		let mut globle_root;
		
		unsafe {
			
			JS_SetOffthreadIonCompilationEnabled(runtime.rt(), false);
			let global = JS_NewGlobalObject(JScontext, &SIMPLE_GLOBAL_CLASS, ptr::null_mut(), h_option, &c_option);
	        //rooted!(in(JScontext) let global_root = global);
	        globle_root = global; 
	        rooted!(in(JScontext) let global_ob =  global);
	    	let global = global_ob.handle();
	    	let _ac = JSAutoCompartment::new(JScontext, global.get());
	    	
	    	
	    	/*rooted!(in(JScontext) let mut rval = UndefinedValue()); 
	    	rval.set(Int32Value(10 as i32));*/
	    	
	    	//rooted!(in(JScontext) let mut rval = PrivateValue((&dataStr).as_ptr() as *const libc::c_void));  
	    	/*let pointTo = dataStr.as_ptr();*/
	    	
	    	/*let strPtr = &dataStr as *const String; 
	    	let mut val = strPtr as *mut std::os::raw::c_void; 
	    	JS_SetPrivate(global.get(), val);
	    	println!("C : private {:?} {:?}", strPtr , val);*/
	    	
	    	  /* let mut data = val as *mut String;
	    	  let mut dataString:&mut String =  &mut *data;
	    	  dataString.push_str("la la laaa.");
              println!("C : JSgloble {:?}",dataString); */
              
	    	/* let mut v8Str = String::from_raw_parts(val as *mut u8, dataStr.len(), dataStr.capacity());
	    	 v8Str.push_str("ok ok ok");*/
	    	 
	    	
	    	 //println!("C {:?}",*strPtr);
	        //JS_SetProperty(JScontext,global_ob.handle(),b"html\0".as_ptr() as *const libc::c_char,rval.handle());
		}
		
		
		let mut instant:Instant = Instant{
			runtime:runtime,
			context:JScontext,
			string:dataStr,
			global:globle_root ,
			bx:Box::new(My{len:String::from("Kishan")})
		}; 
		
		 
		
		unsafe { 
			rooted!(in(JScontext) let global_ob = instant.global);
	    	let global = global_ob.handle(); 
	    	let data = &(*instant.bx) as *const My; 
	    	let val = data as *mut std::os::raw::c_void; 
	    	JS_SetPrivate(global.get(), val); 
	    	//println!("C : private {:?} ", data);
		}
		instant.load_js(); 
		//instant.evaluate_script("var html = '';\n \n 123;print();");
		instant
	}
	
	fn load_js(&mut self) {
        let  context = self.context; 
	    unsafe {  
	    	rooted!(in(self.context) let global_root = self.global);
	    	let global = global_root.handle();
	        //self.evaluate_script(String::from("var ok = print('Spidermonky is loaded.');"));
	        let _ac = JSAutoCompartment::new(self.context, global.get());
            let function = JS_DefineFunction(self.context, global, b"print\0".as_ptr() as *const libc::c_char,
                                         Some(print), 1, 0);
            assert!(!function.is_null()); 
            //warn!("Warning at \n");
	    }
    }
	
	//,jsString:String
	fn evaluate_script(&self,jsString:&str) { 
	    unsafe {  
	    	rooted!(in(self.context) let global_root = self.global);
	    	let global = global_root.handle(); 
			//let _ac = JSAutoCompartment::new(self.context, global.get());
	    	
	        //let javascript = jsScript;//"var ok = print('Spidermonky is loaded.');";
	        rooted!(in(self.context) let mut rval = UndefinedValue());
            let result = self.runtime.evaluate_script(global,jsString, "test", 1, rval.handle_mut());
            
            //println!("rval : {:?}",rval.get());
            
            match result {
            	Err(_) => { 
            		rooted!(in(self.context) let global_root = self.global);
			    	let global = global_root.handle();
		            from_native_error(self.context,self.runtime.rt(),global,);
            	},
            	Ok(_) => {
            		
            	},
            } 
	    }
    }
	 
	
	pub fn loadSrc(&self,jsString:&str) ->  String{  
		use std::thread;
		//let bool; 
		
		 let mut newStr = String::new();
		//if bool == true{
			self.evaluate_script(jsString); 
			use std::mem;  
		   
		    //println!("After call : {:?}",self.bx.len);
		    unsafe{
		    	let len = &self.bx.len as *const String;
		    	let mut data = len as *mut String;
			    mem::swap(&mut newStr, &mut *data);
		    } 
		//}
		 unsafe {  
			//bool = CurrentThreadCanAccessRuntime(self.runtime.rt());
			//if bool == true{
				rooted!(in(self.context) let global_root = self.global);
		    	let global = global_root.handle(); 
		        let _ac = JSAutoCompartment::new(self.context, global.get());
		        //println!("JS_GetCurrentThread() : {:?} : {:?}",JS_GetCurrentThread(),CurrentThreadCanAccessRuntime(self.runtime.rt()));
				JS_GC(self.runtime.rt());
			//}else{
				//println!("JS_GetCurrentThread() : {:?} : {:?}",JS_GetCurrentThread(),CurrentThreadCanAccessRuntime(self.runtime.rt()));
			//} 
		} 
		newStr
    } 
	
} 

 
unsafe fn from_native_error(cx: *mut JSContext,rt: *mut JSRuntime, object: HandleObject) {
    //let report = JS_ErrorFromException(cx, object);
    /*if report.is_null() {
         
    }else {*/
         //let err = GetWarningReporter(rt);
	     //println!("There is Error...! {:?}",err);
    /*}*/
    
    if !JS_IsExceptionPending(cx) {  
    	return; 
    }
    //println!("JS_IsExceptionPending is Error...!");
    
    
    rooted!(in(cx) let mut value = UndefinedValue());
    if !JS_GetPendingException(cx, value.handle_mut()) {
        //JS_ClearPendingException(cx);
        println!("Uncaught exception: JS_GetPendingException failed");
        return;
    }
    
    JS_ClearPendingException(cx);
    let error_info = if value.is_object() {
    	//println!("Error object...!");
    	
    	rooted!(in(cx) let object = value.to_object());
    	let report = JS_ErrorFromException(cx, object.handle());
        if report.is_null() {
        	//println!("report.is_null()");
            return ;
        }
    	let filename = {
            let filename = (*report).filename as *const u8;
            if !filename.is_null() {
                let length = (0..).find(|idx| *filename.offset(*idx) == 0).unwrap();
                let filename = from_raw_parts(filename, length as usize);
                String::from_utf8_lossy(filename).into_owned()
            } else {
                "none".to_string()
            }
        };
    	
    	//println!("Filename : {}" ,filename);
    	
    	let lineno = (*report).lineno;
        let column = (*report).column;
        //println!("at Line : {} : {}" ,lineno , column);
        
    	let message = {
            let message = (*report).ucmessage;
            let length = (0..).find(|idx| *message.offset(*idx) == 0).unwrap();
            let message = from_raw_parts(message, length as usize);
            String::from_utf16_lossy(message)
        };
    	
    	println!("Error at {}:{}:{} -> {}" ,filename,lineno,column,message); 
    	return;
    }; 
    println!("Uncaught exception: JS_GetPendingException failed");
} 
  
unsafe extern "C" fn print(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);  
    let globle = JS_GetGlobalForObject(context,args.callee());
     
     let vals = JS_GetPrivate(globle);
    //println!("from pointer : {:?}",vals);
     
    let mut data = vals as *mut My;
	let my:&mut My = &mut *data;  
	//dataString.push_str(" a ");
	
    //println!("from JSside : {:?}",my.len);
    
    //args.rval().set(Int32Value(10 as i32));
      
    if args._base.argc_ != 1 {
        JS_ReportError(context, b"puts() requires exactly 1 argument\0".as_ptr() as *const libc::c_char);
        return false;
    }  

    let arg = args.get(0);
    let js = js::rust::ToString(context, arg);
    rooted!(in(context) let message_root = js);
    let message = JS_EncodeStringToUTF8(context, message_root.handle());
    let message = CStr::from_ptr(message);
    my.len.push_str(str::from_utf8(message.to_bytes()).unwrap());
    //println!("{}", str::from_utf8(message.to_bytes()).unwrap());
	
	let filename_cstr = ffi::CString::new("puts(puts('yo yo'));".as_bytes()).unwrap(); 
    let jsstring = JS_AtomizeAndPinString(context,filename_cstr.as_ptr());
    rooted!(in(context) let message_root = jsstring);
    let ref data = *jsstring;
    let ht = data;
    
    /*//args.rval().set(Int32Value(10 as i32));*/
    *args.rval() =  *arg.ptr ;
    return true;
}