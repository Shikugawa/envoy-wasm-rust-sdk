extern crate libc;

use crate::context::ensure_root_context;

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
pub fn proxy_on_vm_start(_context_id: u32, _vm_configuration_size: u32) -> u32 {
  ensure_root_context(_context_id).lock().unwrap().on_start();
  1
}

/// Low-level Proxy-WASM APIs for the host functions.
extern "C" {
  pub fn proxy_log(level: u32, message_data: *const u8, message_size: usize) -> u32;
  pub fn proxy_get_property(
    _path_ptr: *const u8,
    _path_size: usize,
    _value_ptr_ptr: *const *const u8,
    _value_size_ptr: *mut usize,
  ) -> u32;
}
