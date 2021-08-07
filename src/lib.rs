mod hello;

use dashmap::DashMap;

use hello::{Request, Response}; 
use protobuf::Message;
use std::mem;
use std::os::raw::c_void;

thread_local!(static DASHMAP: DashMap<i32, usize> = DashMap::new());


#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// TODO: convert to 'handle' instead of 'c_void' (is i32 just happens to work on 64bit systems)
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> i32 {
    let mut buf = Vec::<u8>::with_capacity(size);
    let ptr = buf.as_mut_ptr() as *mut c_void as i32;

    DASHMAP.with(|map| {
        map.insert(ptr, size);
    });
    mem::forget(buf);
    return ptr as i32;
}

#[no_mangle]
pub extern "C" fn get_alloc_size(ptr: i32) -> i32 {
    let mut result:i32 = 0;
    DASHMAP.with(|map| {
        result = *map.get(&(ptr)).unwrap() as i32
    });
    return result;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: i32) {
    DASHMAP.with(|map| {
        map.remove(&ptr);
    });

    unsafe {
        let _buf = Vec::from_raw_parts(ptr as *mut u8, 0, 0);
        mem::forget(_buf);
    }
}

#[no_mangle]
pub extern "C" fn set_at(ptr: i32, offset: i32, val: i32) {
    unsafe {
        let uptr = ptr as *mut u8;
        *uptr.offset(offset as isize) = val as u8;
    }
}

#[no_mangle]
pub extern "C" fn get_at(ptr: *mut c_void, offset: i32) -> i32 {
    unsafe {
        let uptr = ptr as *mut u8;
        *uptr.offset(offset as isize) as i32
    }
}

#[no_mangle]
pub extern "C" fn call(req: *mut c_void, req_len: u32) -> *mut c_void {
    let req_bytes: Vec<u8> =
        unsafe { Vec::from_raw_parts(req as *mut u8, req_len as usize, req_len as usize) };

    let request: Request = Message::parse_from_bytes(&req_bytes).unwrap();

    // TODO: add call to the processing function
    let owned_string: String = "response to input string: ".to_owned();
    let mut response = Response::new();
    response.set_Message(owned_string + request.get_Message());
    
    let mut response_bytes = Message::write_to_bytes(&response).unwrap();

    // short circuit allocation/copy by relying on write_to_bytes usage of Vec to allocate buffer
    DASHMAP.with(|map| {
        map.insert(response_bytes.as_mut_ptr() as i32, response_bytes.len());
    });

    let result = response_bytes.as_mut_ptr() as *mut c_void;
    mem::forget(response_bytes);
    return result;

    // let ptr = alloc(response_bytes.len());

    // //TODO this loop can skip set_at and update array directly
    // for i in 0..response_bytes.len() {
    //     set_at(ptr, i as i32, response_bytes[i as usize] as i32);
    // }
    // return ptr as *mut c_void;
}
