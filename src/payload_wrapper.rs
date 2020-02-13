use crate::buffer::*;
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

// ======================= Low-Level Proxy API Wrapper =============================

pub fn get_header_map_pairs(htype: HeaderMapType) -> Result<HashMap<String, String>, String> {
  let type_num = header_map_type_to_int(htype);
  let data_ptr: *mut c_char = null_mut::<c_char>();
  let size_ptr = Box::into_raw(Box::new(0));
  unsafe {
    let code = proxy_get_header_map_pairs(type_num, &data_ptr, size_ptr);
    match WasmResult::try_from(code) {
      Ok(r) => match r {
        WasmResult::Ok => {
          let header_map = buffer_into_hashmap(data_ptr as *mut u8, *size_ptr);
          Ok(header_map)
        }
        _ => Err(r.to_string()),
      },
      Err(e) => Err(e),
    }
  }
}

pub fn set_header_map_pairs(htype: HeaderMapType, _pairs: &HashMap<String, String>) -> WasmResult {
  let type_num = header_map_type_to_int(htype);
  let (buffer, size) = export_hashmap(_pairs);
  unsafe {
    info!(
      "data {}",
      String::from_raw_parts(buffer as *mut u8, size, size)
    );
  }
  unsafe {
    let code = proxy_set_header_map_pairs(type_num, buffer, size);
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
