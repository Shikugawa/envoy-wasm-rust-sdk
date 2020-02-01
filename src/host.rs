use crate::context::{ensure_context, ensure_root_context};
use crate::envoy_log::Logger;

/// Always hook into host's logging system.
#[no_mangle]
fn _start() {
  Logger::init().unwrap();
}

/// Allow host to allocate memory.
#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
  let mut vec: Vec<u8> = Vec::with_capacity(size);
  unsafe {
    vec.set_len(size);
  }
  let slice = vec.into_boxed_slice();
  Box::into_raw(slice) as *mut u8
}

/// Allow host to free memory.
#[no_mangle]
fn free(ptr: *mut u8) {
  if !ptr.is_null() {
    unsafe {
      Box::from_raw(ptr);
    }
  }
}

#[no_mangle]
fn proxy_on_start() {
  ensure_root_context().on_start();
}

/// Low-level Proxy-WASM APIs for the host functions.
pub mod host {
  extern "C" {
    pub fn proxy_log(level: u32, message_data: *const u8, message_size: usize) -> u32;
  }
}
