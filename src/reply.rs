use crate::buffer::*;
use crate::host::*;
use crate::types::*;
use log::warn;
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn send_local_response(
  status_code: u32,
  details: String,
  body: String,
  additional_header: &HashMap<String, String>,
  grpc_status: GrpcStatus,
) -> WasmResult {
  let (buffer_ptr, size) = export_hashmap(additional_header);
  unsafe {
    let code = proxy_send_local_response(
      status_code,
      details.as_ptr() as *const i8,
      details.as_bytes().len(),
      body.as_ptr() as *const i8,
      body.as_bytes().len(),
      buffer_ptr,
      size,
      grpc_status_to_int(grpc_status),
    );
    match WasmResult::try_from(code) {
      Ok(r) => r,
      Err(e) => {
        warn!("failed to convert: {}", e);
        WasmResult::InternalFailure
      }
    }
  }
}
