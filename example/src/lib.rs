extern crate proxy_wasm;

use lazy_static::*;
use log::info;
use proxy_wasm::context::*;
use proxy_wasm::envoy_log::*;
use proxy_wasm::payload::*;
use proxy_wasm::types::*;
use std::collections::HashMap;
use std::sync::Arc;

// =============== RootContext ============================
struct SampleRootContext {}

impl RootContext for SampleRootContext {
  fn on_start(&self, _configuration_size: u32) -> u32 {
    info!("Hello Envoy!");
    0
  }
}

struct SampleRootContextFactory {}

impl RootContextFactory for SampleRootContextFactory {
  fn create(&self) -> Arc<dyn RootContext + Sync + Send> {
    Arc::from(SampleRootContext {})
  }
}
// ========================================================

// =================== Context ============================
struct SampleContext {
  pub root_context: Arc<dyn RootContext + Sync + Send>,
}

impl Context for SampleContext {
  fn on_create(&self) {
    info!("Hello Envoy Create!");
  }

  fn on_request_headers(&self, _headers: u32) -> FilterHeadersStatus {
    {
      let header = get_request_header_pairs();
      for (k, v) in header {
        info!("{} {}", k, v);
      }
      // info!("header pairs: {}", header.to_string());
    }
    {
      // let mut h = HashMap::new();
      // h.insert("x-key".to_string(), "value".to_string());
      // set_request_header_pairs(&h);
      // let header = get_request_header_pairs();
      // info!("header pairs: {}", header.to_string());
    }
    {
      // let path = get_request_header(String::from(":path"));
      // info!("prev path: {}", path.to_string());
      // replace_request_header(":path".to_string(), "/blue".to_string());
      // let path = get_request_header(String::from(":path"));
      // info!("current path: {}", path.to_string());
      // remove_request_header(String::from(":path"));
      // add_request_header(String::from(":path"), "/green".to_string());
      // let path = get_request_header(String::from(":path"));
      // info!("next path: {}", path.to_string());
      // let size = get_request_trailer_size();
      // info!("size: {}", size);
    }
    FilterHeadersStatus::Continue
  }
}

struct SampleContextFactory {}

impl ContextFactory for SampleContextFactory {
  fn create(
    &self,
    _root_context: Arc<dyn RootContext + Sync + Send>,
  ) -> Arc<dyn Context + Sync + Send> {
    Arc::from(SampleContext {
      root_context: _root_context,
    })
  }
}
// ========================================================

lazy_static! {
  static ref SAMPLE_ROOT_CONTEXT_FACTORY: Box<dyn RootContextFactory + Sync + Send> =
    Box::new(SampleRootContextFactory {});
  static ref SAMPLE_CONTEXT_FACTORY: Box<dyn ContextFactory + Sync + Send> =
    Box::new(SampleContextFactory {});
}

#[no_mangle]
fn _start() {
  Logger::init().unwrap();
  register_factory(
    "my_root_id",
    &SAMPLE_CONTEXT_FACTORY,
    &SAMPLE_ROOT_CONTEXT_FACTORY,
  );
}
