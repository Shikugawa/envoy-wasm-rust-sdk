use crate::host::*;
use crate::types::*;
use log::error;
use std::collections::HashMap;
use std::convert::TryFrom;
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

// ======================= Low-Level Proxy API Wrapper =============================

// TODO(Rei Shimizu): Output pairs data should be non-readable
// e.g \n\n\n$:authoritylocalhost:8000:path/:methodGET...
// So we should destroy top change-line symbols and insert them to different headers.
// e.g :authority: localhost:8000\n:path: /\n:method: GET\n...
pub fn get_header_map_pairs(htype: HeaderMapType) -> Result<Box<WasmData>, String> {
  let type_num = header_map_type_to_int(htype);
  let data_ptr: *mut c_char = null_mut::<c_char>();
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe {
    let code = proxy_get_header_map_pairs(type_num, &data_ptr, size_ptr);
    match WasmResult::try_from(code) {
      Ok(r) => match r {
        WasmResult::Ok => Ok(Box::new(WasmData {
          data: data_ptr,
          len: *size_ptr,
        })),
        _ => Err(r.to_string()),
      },
      Err(e) => Err(e),
    }
  }
}

pub fn get_header_map_value(htype: HeaderMapType, key: String) -> Result<Box<WasmData>, String> {
  let type_num = header_map_type_to_int(htype);
  let data_ptr: *mut c_char = null_mut::<c_char>();
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe {
    let code = proxy_get_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.bytes().len(),
      &data_ptr,
      size_ptr,
    );
    match WasmResult::try_from(code) {
      Ok(r) => match r {
        WasmResult::Ok => Ok(Box::new(WasmData {
          data: data_ptr,
          len: *size_ptr,
        })),
        _ => Err(r.to_string()),
      },
      Err(e) => Err(e),
    }
  }
}

pub fn add_header_map_value(htype: HeaderMapType, key: String, value: String) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  unsafe {
    let code = proxy_add_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.as_bytes().len(),
      value.as_ptr() as *const c_char,
      value.as_bytes().len(),
    );
    match WasmResult::try_from(code) {
      Ok(r) => r,
      Err(e) => panic!(format!("failed to convert: {}", e)),
    }
  }
}

pub fn replace_header_map_value(htype: HeaderMapType, key: String, value: String) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  unsafe {
    let code = proxy_replace_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.as_bytes().len(),
      value.as_ptr() as *const c_char,
      value.as_bytes().len(),
    );
    match WasmResult::try_from(code) {
      Ok(r) => r,
      Err(e) => panic!(format!("failed to convert: {}", e)),
    }
  }
}

pub fn remove_header_map_value(htype: HeaderMapType, key: String) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  unsafe {
    let code = proxy_remove_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.as_bytes().len(),
    );
    match WasmResult::try_from(code) {
      Ok(r) => r,
      Err(e) => panic!(format!("failed to convert: {}", e)),
    }
  }
}

pub fn get_header_map_value_size(htype: HeaderMapType) -> u32 {
  let type_num = header_map_type_to_int(htype);
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe { proxy_get_header_map_size(type_num, size_ptr) }
}

// ======================= Low-Level Proxy API Wrapper =============================

// ====================== Request Header Processing API ===========================
pub fn set_request_header_pairs(_pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn get_request_header_pairs() -> Box<WasmData> {
  match get_header_map_pairs(HeaderMapType::RequestHeaders) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn get_request_header(key: String) -> Box<WasmData> {
  match get_header_map_value(HeaderMapType::RequestHeaders, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn add_request_header(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::RequestHeaders, key, value)
}

pub fn replace_request_header(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::RequestHeaders, key, value)
}

pub fn remove_request_header(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::RequestHeaders, key)
}

pub fn get_request_header_size() -> u32 {
  get_header_map_value_size(HeaderMapType::RequestHeaders)
}
// ====================== Request Header Processing API ===========================

// ====================== Response Header Processing API ===========================
pub fn set_response_header_pairs(_pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn get_response_header_pairs() -> Box<WasmData> {
  match get_header_map_pairs(HeaderMapType::ResponseHeaders) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn get_response_header(key: String) -> Box<WasmData> {
  match get_header_map_value(HeaderMapType::ResponseHeaders, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn add_response_header(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::ResponseHeaders, key, value)
}

pub fn replace_response_header(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::ResponseHeaders, key, value)
}

pub fn remove_response_header(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::ResponseHeaders, key)
}

pub fn get_response_header_size() -> u32 {
  get_header_map_value_size(HeaderMapType::ResponseHeaders)
}
// ====================== Response Header Processing API ===========================

// ====================== Request Trailer Processing API ===========================
pub fn set_request_trailer_pairs(_pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn get_request_trailer_pairs() -> Box<WasmData> {
  match get_header_map_pairs(HeaderMapType::RequestTrailers) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn get_request_trailer(key: String) -> Box<WasmData> {
  match get_header_map_value(HeaderMapType::RequestTrailers, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn add_request_trailer(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::RequestTrailers, key, value)
}

pub fn replace_request_trailer(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::RequestTrailers, key, value)
}

pub fn remove_request_trailer(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::RequestTrailers, key)
}

pub fn get_request_trailer_size() -> u32 {
  get_header_map_value_size(HeaderMapType::RequestTrailers)
}
// ====================== Request Trailer Processing API ===========================

// ====================== Response Trailer Processing API ===========================
pub fn set_response_trailer_pairs(_pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn get_response_trailer_pairs() -> Box<WasmData> {
  match get_header_map_pairs(HeaderMapType::ResponseTrailers) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn get_response_trailer(key: String) -> Box<WasmData> {
  match get_header_map_value(HeaderMapType::ResponseTrailers, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn add_response_trailer(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::ResponseTrailers, key, value)
}

pub fn replace_response_trailer(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::ResponseTrailers, key, value)
}

pub fn remove_response_trailer(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::ResponseTrailers, key)
}

pub fn get_response_trailer_size() -> u32 {
  get_header_map_value_size(HeaderMapType::ResponseTrailers)
}
// ====================== Response Trailer Processing API ===========================
