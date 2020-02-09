use crate::host::*;
use crate::types::*;
use log::error;
use std::collections::HashMap;
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
pub fn getHeaderMapPairs(htype: HeaderMapType) -> Result<Box<WasmData>, &'static str> {
  let type_num = header_map_type_to_int(htype);
  let data_ptr: *mut c_char = null_mut::<c_char>();
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe {
    let code = proxy_get_header_map_pairs(type_num, &data_ptr, size_ptr);
    match to_wasm_result(code) {
      WasmResult::Ok => Ok(Box::new(WasmData {
        data: data_ptr,
        len: *size_ptr,
      })),
      _ => Err(wasm_result_to_str(to_wasm_result(code))),
    }
  }
}

pub fn getHeaderMapValue(htype: HeaderMapType, key: String) -> Result<Box<WasmData>, &'static str> {
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
    match to_wasm_result(code) {
      WasmResult::Ok => Ok(Box::new(WasmData {
        data: data_ptr,
        len: *size_ptr,
      })),
      _ => Err(wasm_result_to_str(to_wasm_result(code))),
    }
  }
}

pub fn addHeaderMapValue(htype: HeaderMapType, key: String, value: String) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  unsafe {
    let code = proxy_add_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.as_bytes().len(),
      value.as_ptr() as *const c_char,
      value.as_bytes().len(),
    );
    to_wasm_result(code)
  }
}

pub fn replaceHeaderMapValue(htype: HeaderMapType, key: String, value: String) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  unsafe {
    let code = proxy_replace_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.as_bytes().len(),
      value.as_ptr() as *const c_char,
      value.as_bytes().len(),
    );
    to_wasm_result(code)
  }
}

pub fn removeHeaderMapValue(htype: HeaderMapType, key: String) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  unsafe {
    let code = proxy_remove_header_map_value(
      type_num,
      key.as_ptr() as *const c_char,
      key.as_bytes().len(),
    );
    to_wasm_result(code)
  }
}

pub fn getHeaderMapValueSize(htype: HeaderMapType) -> u32 {
  let type_num = header_map_type_to_int(htype);
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe { proxy_get_header_map_size(type_num, size_ptr) }
}

// ======================= Low-Level Proxy API Wrapper =============================

// ====================== Request Header Processing API ===========================
pub fn setRequestHeaderPairs(pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn getRequestHeaderPairs() -> Box<WasmData> {
  match getHeaderMapPairs(HeaderMapType::RequestHeaders) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn getRequestHeader(key: String) -> Box<WasmData> {
  match getHeaderMapValue(HeaderMapType::RequestHeaders, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn addRequestHeader(key: String, value: String) -> WasmResult {
  addHeaderMapValue(HeaderMapType::RequestHeaders, key, value)
}

pub fn replaceRequestHeader(key: String, value: String) -> WasmResult {
  replaceHeaderMapValue(HeaderMapType::RequestHeaders, key, value)
}

pub fn removeRequestHeader(key: String) -> WasmResult {
  removeHeaderMapValue(HeaderMapType::RequestHeaders, key)
}

pub fn getRequestHeaderSize() -> u32 {
  getHeaderMapValueSize(HeaderMapType::RequestHeaders)
}
// ====================== Request Header Processing API ===========================

// ====================== Response Header Processing API ===========================
pub fn setResponseHeaderPairs(pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn getResponseHeaderPairs() -> Box<WasmData> {
  match getHeaderMapPairs(HeaderMapType::ResponseHeaders) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn getResponseHeader(key: String) -> Box<WasmData> {
  match getHeaderMapValue(HeaderMapType::ResponseHeaders, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn addResponseHeader(key: String, value: String) -> WasmResult {
  addHeaderMapValue(HeaderMapType::ResponseHeaders, key, value)
}

pub fn replaceResponseHeader(key: String, value: String) -> WasmResult {
  replaceHeaderMapValue(HeaderMapType::ResponseHeaders, key, value)
}

pub fn removeResponseHeader(key: String) -> WasmResult {
  removeHeaderMapValue(HeaderMapType::ResponseHeaders, key)
}

pub fn getResponseHeaderSize() -> u32 {
  getHeaderMapValueSize(HeaderMapType::ResponseHeaders)
}
// ====================== Response Header Processing API ===========================

// ====================== Request Trailer Processing API ===========================
pub fn setRequestTrailerPairs(pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn getRequestTrailerPairs() -> Box<WasmData> {
  match getHeaderMapPairs(HeaderMapType::RequestTrailers) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn getRequestTrailer(key: String) -> Box<WasmData> {
  match getHeaderMapValue(HeaderMapType::RequestTrailers, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn addRequestTrailer(key: String, value: String) -> WasmResult {
  addHeaderMapValue(HeaderMapType::RequestTrailers, key, value)
}

pub fn replaceRequestTrailer(key: String, value: String) -> WasmResult {
  replaceHeaderMapValue(HeaderMapType::RequestTrailers, key, value)
}

pub fn removeRequestTrailer(key: String) -> WasmResult {
  removeHeaderMapValue(HeaderMapType::RequestTrailers, key)
}

pub fn getRequestTrailerSize() -> u32 {
  getHeaderMapValueSize(HeaderMapType::RequestTrailers)
}
// ====================== Request Trailer Processing API ===========================

// ====================== Response Trailer Processing API ===========================
pub fn setResponseTrailerPairs(pairs: &HashMap<String, String>) -> Box<WasmData> {
  unimplemented!()
  // setHeaderMapPairs(HeaderMapType::RequestHeaders, pairs)
}

pub fn getResponseTrailerPairs() -> Box<WasmData> {
  match getHeaderMapPairs(HeaderMapType::ResponseTrailers) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn getResponseTrailer(key: String) -> Box<WasmData> {
  match getHeaderMapValue(HeaderMapType::ResponseTrailers, key) {
    Ok(t) => t,
    Err(e) => {
      error!("{}", e);
      panic!();
    }
  }
}

pub fn addResponseTrailer(key: String, value: String) -> WasmResult {
  addHeaderMapValue(HeaderMapType::ResponseTrailers, key, value)
}

pub fn replaceResponseTrailer(key: String, value: String) -> WasmResult {
  replaceHeaderMapValue(HeaderMapType::ResponseTrailers, key, value)
}

pub fn removeResponseTrailer(key: String) -> WasmResult {
  removeHeaderMapValue(HeaderMapType::ResponseTrailers, key)
}

pub fn getResponseTrailerSize() -> u32 {
  getHeaderMapValueSize(HeaderMapType::ResponseTrailers)
}
// ====================== Response Trailer Processing API ===========================
