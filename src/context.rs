use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait RootContext {
  fn on_start(&self);
}
pub trait Context {}
pub trait RootContextFactory {
  fn create(&self) -> Box<dyn Sync + RootContext>;
}
pub trait ContextFactory {
  fn create(&self) -> Box<dyn Sync + Context>;
}

struct RootContextFactoryStore {
  hangar: Vec<&'static (dyn RootContextFactory + Sync)>,
}

impl RootContextFactoryStore {
  fn new() -> RootContextFactoryStore {
    RootContextFactoryStore { hangar: Vec::new() }
  }

  fn add<U: RootContextFactory + Sync>(&mut self, factory: &'static U) {
    if self.hangar.len() != 0 {
      self.hangar.push(factory);
    }
  }

  fn top(&self) -> Option<&&(dyn RootContextFactory + Sync)> {
    self.hangar.get(0)
  }
}

struct ContextFactoryStore {
  hangar: HashMap<&'static str, &'static (dyn ContextFactory + Sync)>,
}

impl ContextFactoryStore {
  fn new() -> ContextFactoryStore {
    ContextFactoryStore {
      hangar: HashMap::new(),
    }
  }

  fn add<U: ContextFactory + Sync>(&mut self, id: &'static str, factory: &'static U) {
    self.hangar.insert(id, factory);
  }

  fn get(&self, id: &str) -> Option<&&(dyn ContextFactory + Sync)> {
    self.hangar.get(id)
  }
}

lazy_static! {
  static ref ROOT_CONTEXT_FACTORY_STORE: Mutex<RootContextFactoryStore> =
    Mutex::new(RootContextFactoryStore::new()); // takes only zero or one value
  static ref CONTEXT_FACTORY_STORE: Mutex<ContextFactoryStore> =
    Mutex::new(ContextFactoryStore::new());
}

pub struct Registered {}

pub fn register_factory<T: RootContextFactory + Sync, U: ContextFactory + Sync>(
  _id: &'static str,
  _rcf: &'static T,
  _cf: &'static U,
) -> Registered {
  ROOT_CONTEXT_FACTORY_STORE.lock().unwrap().add(_rcf);
  CONTEXT_FACTORY_STORE.lock().unwrap().add(_id, _cf);
  Registered {}
}

pub fn ensure_root_context() -> Box<dyn Sync + RootContext> {
  match ROOT_CONTEXT_FACTORY_STORE.lock().unwrap().top() {
    None => panic!("failed to find root context factory!"),
    Some(store) => store.create(),
  }
}

pub fn ensure_context(id: &str) -> Box<dyn Sync + Context> {
  match CONTEXT_FACTORY_STORE.lock().unwrap().get(id) {
    None => panic!("failed to find context factory!"),
    Some(store) => store.create(),
  }
}
