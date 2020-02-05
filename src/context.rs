use crate::host::*;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex};

pub trait RootContext {
  fn on_start(&self) -> u32;
}
pub trait Context {
  fn on_create(&self);
}

pub trait RootContextFactory {
  fn create(&self) -> Arc<Mutex<dyn Sync + RootContext + Send>>;
}
pub trait ContextFactory {
  fn create(&self) -> Arc<dyn Sync + Context + Send>;
}

struct RootContextFactoryMap<'a> {
  pub hangar: HashMap<&'static str, &'a Box<dyn RootContextFactory + Sync>>,
}

// HashMap Wrapper
impl<'a> RootContextFactoryMap<'a> {
  fn new() -> RootContextFactoryMap<'a> {
    RootContextFactoryMap {
      hangar: HashMap::new(),
    }
  }
}

// HashMap Wrapper
struct ContextFactoryMap<'a> {
  hangar: HashMap<&'static str, &'a Box<dyn ContextFactory + Sync>>,
}

impl<'a> ContextFactoryMap<'a> {
  fn new() -> ContextFactoryMap<'a> {
    ContextFactoryMap {
      hangar: HashMap::new(),
    }
  }
}

// HashMap Wrapper
struct RootContextMap {
  pub hangar: HashMap<u32, Arc<Mutex<dyn RootContext + Sync + Send>>>,
}

impl RootContextMap {
  fn new() -> RootContextMap {
    RootContextMap {
      hangar: HashMap::new(),
    }
  }
}

// HashMap Wrapper
struct ContextMap {
  hangar: HashMap<u32, Arc<dyn Context + Sync + Send>>,
}

impl ContextMap {
  fn new() -> ContextMap {
    ContextMap {
      hangar: HashMap::new(),
    }
  }
}

lazy_static! {
  static ref ROOT_CONTEXT_FACTORY_MAP: Mutex<RootContextFactoryMap<'static>> =
    Mutex::new(RootContextFactoryMap::new());
  static ref CONTEXT_FACTORY_MAP: Mutex<ContextFactoryMap<'static>> =
    Mutex::new(ContextFactoryMap::new());
  static ref ROOT_CONTEXT_MAP: Mutex<RootContextMap> = Mutex::new(RootContextMap::new());
  static ref CONTEXT_MAP: Mutex<ContextMap> = Mutex::new(ContextMap::new());
}

struct SampleRootContextTest {}

impl RootContext for SampleRootContextTest {
  fn on_start(&self) -> u32 {
    // info!("Hello Envoy!");
    0
  }
}

pub fn register_factory(
  _root_id: &'static str,
  _cf: &'static Box<dyn ContextFactory + Sync>,
  _rcf: &'static Box<dyn RootContextFactory + Sync>,
) {
  info!("{}", _root_id);
  let a: Arc<Mutex<dyn RootContext + Sync + Send>> = Arc::new(Mutex::new(SampleRootContextTest {}));
  ROOT_CONTEXT_MAP
    .lock()
    .unwrap()
    .hangar
    .insert(0, Arc::clone(&a));
  ROOT_CONTEXT_FACTORY_MAP
    .lock()
    .unwrap()
    .hangar
    .insert(_root_id, &_rcf);
  CONTEXT_FACTORY_MAP
    .lock()
    .unwrap()
    .hangar
    .insert(_root_id, &_cf);
}

pub fn ensure_root_context<'a>(root_context_id: u32) -> Arc<Mutex<dyn RootContext + Sync + Send>> {
  let root_context = match ROOT_CONTEXT_MAP
    .lock()
    .unwrap()
    .hangar
    .get(&root_context_id)
  {
    Some(x) => Arc::clone(x),
    None => {
      let path = "plugin_root_id";
      let root_id: *const u8 = null::<u8>();
      let root_id_size: *mut usize = null_mut::<usize>();
      // proxy_get_property(path.as_ptr(), path.len(), &root_id, root_id_size);
      // let root_id_str: String = CString::from_raw(root_id as *mut i8)
      //   .into_string()
      //   .unwrap_or(String::from("my_root_id"));
      let root_id_str = String::from("my_root_id");
      let root_context = match ROOT_CONTEXT_FACTORY_MAP
        .lock()
        .unwrap()
        .hangar
        .get(&root_id_str.as_ref())
      {
        Some(root_factory) => root_factory.create(),
        None => panic!("failed"),
      };
      root_context
    }
  };
  if !ROOT_CONTEXT_MAP
    .lock()
    .unwrap()
    .hangar
    .contains_key(&root_context_id)
  {
    ROOT_CONTEXT_MAP
      .lock()
      .unwrap()
      .hangar
      .insert(root_context_id, Arc::clone(&root_context));
  }
  root_context
}

// pub fn ensure_context<'a>(root_context_id: u32, context_id: u32) -> &'a Box<dyn Sync + Context> {

// }
