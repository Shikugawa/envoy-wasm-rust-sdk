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
  fn create(&self) -> Arc<Mutex<dyn RootContext + Sync + Send>>;
}
pub trait ContextFactory {
  fn create(
    &self,
    _root_context: Arc<Mutex<dyn RootContext + Sync + Send>>,
  ) -> Arc<Mutex<dyn Context + Sync + Send>>;
}

lazy_static! {
  static ref ROOT_CONTEXT_FACTORY_MAP: Mutex<HashMap<&'static str, &'static Box<dyn RootContextFactory + Sync>>> =
    Mutex::new(HashMap::new());
  static ref CONTEXT_FACTORY_MAP: Mutex<HashMap<&'static str, &'static Box<dyn ContextFactory + Sync>>> =
    Mutex::new(HashMap::new());
  static ref ROOT_CONTEXT_MAP: Mutex<HashMap<u32, Arc<Mutex<dyn RootContext + Sync + Send>>>> =
    Mutex::new(HashMap::new());
  static ref CONTEXT_MAP: Mutex<HashMap<u32, Arc<Mutex<dyn Context + Sync + Send>>>> =
    Mutex::new(HashMap::new());
}

pub fn register_factory(
  _root_id: &'static str,
  _cf: &'static Box<dyn ContextFactory + Sync>,
  _rcf: &'static Box<dyn RootContextFactory + Sync>,
) {
  ROOT_CONTEXT_FACTORY_MAP
    .lock()
    .unwrap()
    .insert(_root_id, &_rcf);
  CONTEXT_FACTORY_MAP.lock().unwrap().insert(_root_id, &_cf);
}

pub fn ensure_root_context(root_context_id: u32) -> Arc<Mutex<dyn RootContext + Sync + Send>> {
  let root_context = match ROOT_CONTEXT_MAP.lock().unwrap().get(&root_context_id) {
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
        .get(&root_id_str.as_ref())
      {
        Some(root_factory) => root_factory.create(),
        None => panic!("failed"),
      };
      root_context
    }
  };
  let mut locked_map = ROOT_CONTEXT_MAP.lock().unwrap();
  if !locked_map.contains_key(&root_context_id) {
    locked_map.insert(root_context_id, Arc::clone(&root_context));
  }
  root_context
}

pub fn ensure_context(
  context_id: u32,
  root_context_id: u32,
) -> Arc<Mutex<dyn Context + Sync + Send>> {
  let context = match CONTEXT_MAP.lock().unwrap().get(&context_id) {
    Some(x) => Arc::clone(x),
    None => {
      let root_id_str = String::from("my_root_id");
      let context = match CONTEXT_FACTORY_MAP
        .lock()
        .unwrap()
        .get(&root_id_str.as_ref())
      {
        Some(factory) => {
          let context = match ROOT_CONTEXT_MAP.lock().unwrap().get(&root_context_id) {
            Some(root_context) => factory.create(Arc::clone(root_context)),
            None => unimplemented!(), // Can't find specified root_context_id
          };
          context
        }
        None => unimplemented!(), // can't find speficied context_id
      };
      context
    }
  };
  context
}

pub fn get_context(context_id: u32) -> Arc<Mutex<dyn Context + Sync + Send>> {
  let context = match CONTEXT_MAP.lock().unwrap().get(&context_id) {
    Some(x) => Arc::clone(x),
    None => unimplemented!(),
  };
  context
}

pub fn get_root_context(root_context_id: u32) -> Arc<Mutex<dyn RootContext + Sync + Send>> {
  let root_context = match ROOT_CONTEXT_MAP.lock().unwrap().get(&root_context_id) {
    Some(x) => Arc::clone(x),
    None => unimplemented!(),
  };
  root_context
}
