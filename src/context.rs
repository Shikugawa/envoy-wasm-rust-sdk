use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait RootContext {
  fn on_start(&self) -> u32;
}
pub trait Context {
  fn on_create(&self);
}

pub trait RootContextFactory {
  fn create(&self) -> Box<dyn Sync + RootContext>;
}
pub trait ContextFactory {
  fn create(&self) -> Box<dyn Sync + Context>;
}

struct RootContextFactoryMap {
  hangar: HashMap<&'static str, &'static (dyn RootContextFactory + Sync)>,
}

// HashMap Wrapper
impl RootContextFactoryMap {
  fn new() -> RootContextFactoryMap {
    RootContextFactoryMap {
      hangar: HashMap::new(),
    }
  }

  fn add<U: RootContextFactory + Sync>(&mut self, root_id: &'static str, factory: &'static U) {
    self.hangar.insert(root_id, factory);
  }

  fn get(&self, root_id: &str) -> Option<&&(dyn RootContextFactory + Sync)> {
    self.hangar.get(root_id)
  }
}

// HashMap Wrapper
struct ContextFactoryMap {
  hangar: HashMap<&'static str, &'static (dyn ContextFactory + Sync)>,
}

impl ContextFactoryMap {
  fn new() -> ContextFactoryMap {
    ContextFactoryMap {
      hangar: HashMap::new(),
    }
  }

  fn add<U: ContextFactory + Sync>(&mut self, root_id: &'static str, factory: &'static U) {
    self.hangar.insert(root_id, factory);
  }

  fn get(&self, root_id: &str) -> Option<&&(dyn ContextFactory + Sync)> {
    self.hangar.get(root_id)
  }
}

// HashMap Wrapper
struct RootContextMap {
  hangar: HashMap<u32, &'static (dyn RootContext + Sync)>,
}

impl RootContextMap {
  fn new() -> RootContextMap {
    RootContextMap {
      hangar: HashMap::new(),
    }
  }

  fn add<U: RootContext + Sync>(&mut self, root_context_id: u32, context: &'static U) {
    self.hangar.insert(root_context_id, context);
  }

  fn get(&self, root_context_id: u32) -> Option<&&(dyn RootContext + Sync)> {
    self.hangar.get(&root_context_id)
  }
}

// HashMap Wrapper
struct ContextMap {
  hangar: HashMap<u32, &'static (dyn Context + Sync)>,
}

impl ContextMap {
  fn new() -> ContextMap {
    ContextMap {
      hangar: HashMap::new(),
    }
  }

  fn add<U: Context + Sync>(&mut self, context_id: u32, context: &'static U) {
    self.hangar.insert(context_id, context);
  }

  fn get(&self, context_id: u32) -> Option<&&(dyn Context + Sync)> {
    self.hangar.get(&context_id)
  }
}

lazy_static! {
  static ref ROOT_CONTEXT_FACTORY_MAP: Mutex<RootContextFactoryMap> =
    Mutex::new(RootContextFactoryMap::new());
  static ref CONTEXT_FACTORY_MAP: Mutex<ContextFactoryMap> = Mutex::new(ContextFactoryMap::new());
  static ref ROOT_CONTEXT_MAP: Mutex<RootContextMap> = Mutex::new(RootContextMap::new());
  static ref CONTEXT_MAP: Mutex<ContextMap> = Mutex::new(ContextMap::new());
}

pub struct Registered {}

pub fn register_factory<T: RootContextFactory + Sync, U: ContextFactory + Sync>(
  _root_id: &'static str,
  _rcf: &'static T,
  _cf: &'static U,
) -> Registered {
  ROOT_CONTEXT_FACTORY_MAP.lock().unwrap().add(_root_id, _rcf);
  CONTEXT_FACTORY_MAP.lock().unwrap().add(_root_id, _cf);
  Registered {}
}

pub fn ensure_root_context(root_context_id: u32) -> Box<dyn Sync + RootContext> {
  // match ROOT_CONTEXT_MAP.lock().unwrap().get(root_context_id) {
  //   Some(root_context) => Box::new(root_context),
  // None => {
  let root_id = "my_root_id";
  let root_context = match ROOT_CONTEXT_FACTORY_MAP.lock().unwrap().get(root_id) {
    Some(root_factory) => root_factory.create(),
    None => panic!("failed"),
  };
  root_context
  // }
  // }
}

pub fn ensure_context(context_id: u32) -> Box<dyn Sync + Context> {
  let root_id = "my_root_id";
  let context = match CONTEXT_FACTORY_MAP.lock().unwrap().get(root_id) {
    Some(factory) => factory.create(),
    None => panic!("failed"),
  };
  context
}
