extern crate libc;
use std::ptr::NonNull;
use std::marker;
use libc::c_int;


#[repr(C)]
pub struct worker {
    _unused: [u8; 0],
}
    

extern {
    fn worker_new() -> *mut worker;
    fn worker_set_rust_object(w: *mut worker, p: *mut Worker);

    fn trigger_callback(p: *mut worker, n: c_int) -> c_int;
}

struct WorkerPtr(NonNull<worker>);
unsafe impl marker::Send for WorkerPtr {}

#[repr(C)]
pub struct Worker {
    ptr: WorkerPtr,
    cb: fn(i32) -> i32,
}

impl Worker {
    pub fn trigger_callback(&mut self, n: i32) -> i32 {
        println!("trigger_callback: {}", n);
        (self.cb)(n)
    }
}

fn new(cb: fn(i32) -> i32) -> Worker {
    let mut _ptr: *mut worker;
    _ptr = unsafe { worker_new() };

    let wrapper = WorkerPtr(NonNull::new(_ptr).unwrap());
    let w = Worker{
      ptr: wrapper,
      cb: cb,
    };

    let mut boxed_worker = Box::new(w);
    unsafe { worker_set_rust_object(_ptr, boxed_worker.as_mut())};
    *boxed_worker
}

fn main() {
    let test_cb = move |n: i32| {
        n*2
    };
    let mut _test_worker = new(test_cb);

    let n = 100;
    unsafe {
        let out = trigger_callback(_test_worker.ptr.0.as_ptr(), n as c_int);
        println!("got: {}", out);
    }
}

#[no_mangle]
pub extern fn rust_callback(w: *mut Worker, raw_n: c_int) -> c_int {
    let n = raw_n as i32;
    println!("rust_callback: {}", n);
    unsafe {
        let out = (*w).trigger_callback(n);
        out as c_int
    }
}