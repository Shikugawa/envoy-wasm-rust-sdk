extern crate proxy_wasm;

use lazy_static::*;
use log::info;
use proxy_wasm::context::*;
use proxy_wasm::envoy_log::*;
use proxy_wasm::payload::*;
use proxy_wasm::reply::*;
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
    // {
    //   let header = get_request_header_pairs();
    //   for (k, v) in header.unwrap().iter() {
    //     info!("{} {}", k, v);
    //   }
    // }
    {
      let mut h = HashMap::new();
      let header = get_request_header_pairs().unwrap();
      h.insert(":path".to_string(), "/value".to_string());
      h.insert(":method".to_string(), "GET".to_string());
      h.insert("Host".to_string(), "example.com".to_string());
      set_request_header_pairs(&h);
      let header = get_request_header_pairs().unwrap();
      for (k, v) in header.iter() {
        info!("{} {}", k, v);
      }
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
