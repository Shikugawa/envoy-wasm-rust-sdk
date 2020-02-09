use crate::host::*;
use crate::types::*;
use std::os::raw::c_char;
use std::ptr::null_mut;

pub struct WasmData {
  data: *const c_char,
  len: usize,
}

impl WasmData {
  pub fn to_string(&self) -> String {
    unsafe { String::from_raw_parts(self.data as *mut u8, self.len, self.len) }
  }
}

// TODO(Rei Shimizu): Output pairs data should be non-readable
// e.g \n\n\n$:authoritylocalhost:8000:path/:methodGET...
// So we should destroy top change-line symbols and insert them to different headers.
// e.g :authority: localhost:8000\n:path: /\n:method: GET\n...
pub fn getHeaderMapPairs(htype: HeaderMapType) -> Box<WasmData> {
  let type_num = header_map_type_to_int(htype);
  let data_ptr: *mut c_char = null_mut::<c_char>();
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe {
    proxy_get_header_map_pairs(type_num, &data_ptr, size_ptr);
    Box::new(WasmData {
      data: data_ptr,
      len: *size_ptr,
    })
  }
}

pub fn getHeaderMapValue(htype: HeaderMapType, key: String) -> Box<WasmData> {
  let type_num = header_map_type_to_int(htype);
  let data_ptr: *mut c_char = null_mut::<c_char>();
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe {
    proxy_get_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.bytes().len(),
      &data_ptr,
      size_ptr,
    );
    Box::new(WasmData {
      data: data_ptr,
      len: *size_ptr,
    })
  }
}

pub fn getRequestHeaderPairs() -> Box<WasmData> {
  getHeaderMapPairs(HeaderMapType::RequestHeaders)
}

pub fn getRequestHeader(key: String) -> Box<WasmData> {
  getHeaderMapValue(HeaderMapType::RequestHeaders, key)
}
