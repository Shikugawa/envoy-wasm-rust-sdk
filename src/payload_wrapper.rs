use crate::host::*;
use crate::types::*;
use log::{info, warn};
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

pub fn hashmap_into_buffer(_pairs: &HashMap<String, String>) -> (*mut c_char, usize) {
  let mut buffer_size = 0;
  let mut tmp_buffer = Vec::<u8>::with_capacity(0);
  for (key, value) in _pairs {
    let key_size = key.as_bytes().len();
    buffer_size += &key_size;
    tmp_buffer.reserve(buffer_size);
    unsafe {
      tmp_buffer.append(&mut Vec::from_raw_parts(
        key.as_ptr() as *mut u8,
        key_size,
        key_size,
      ));
    }
    let value_size = value.as_bytes().len();
    buffer_size += &value_size;
    tmp_buffer.reserve(buffer_size);
    unsafe {
      tmp_buffer.append(&mut Vec::from_raw_parts(
        value.as_ptr() as *mut u8,
        value_size,
        value_size,
      ));
    }
  }
  (
    Box::into_raw(tmp_buffer.into_boxed_slice()) as *mut c_char,
    buffer_size,
  )
}

pub fn export_hashmap(_pairs: &HashMap<String, String>) -> (*mut c_char, usize) {
  hashmap_into_buffer(_pairs)
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

pub fn set_header_map_pairs(htype: HeaderMapType, _pairs: &HashMap<String, String>) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  let (buffer_ptr, buffer_size) = export_hashmap(_pairs);
  unsafe {
    let code = proxy_set_header_map_pairs(type_num, buffer_ptr, buffer_size);
    match WasmResult::try_from(code) {
      Ok(r) => r,
      Err(e) => {
        warn!("failed to convert: {}", e);
        WasmResult::InternalFailure
      }
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
