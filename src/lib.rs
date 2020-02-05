mod context;

mod envoy_log;
mod host;
use crate::envoy_log::Logger;
use context::*;
use lazy_static::*;
use log::info;
use std::sync::{Arc, Mutex};

// =============== RootContext ============================
struct SampleRootContext {}

impl RootContext for SampleRootContext {
    fn on_start(&self) -> u32 {
        // info!("Hello Envoy!");
        0
    }
}

struct SampleRootContextFactory {}

impl RootContextFactory for SampleRootContextFactory {
    fn create(&self) -> Arc<Mutex<dyn RootContext + Sync + Send>> {
        Arc::new(Mutex::new(SampleRootContext {}))
    }
}
// ========================================================

// =================== Context ============================
struct SampleContext {}

impl Context for SampleContext {
    fn on_create(&self) {}
}

struct SampleContextFactory {}

impl ContextFactory for SampleContextFactory {
    fn create(&self) -> Arc<dyn Context + Sync + Send> {
        Arc::new(SampleContext {})
    }
}
// ========================================================

static mut RETRY: bool = true;

lazy_static! {
    static ref SAMPLE_ROOT_CONTEXT_FACTORY: Box<dyn RootContextFactory + Sync> =
        Box::new(SampleRootContextFactory {});
    static ref SAMPLE_CONTEXT_FACTORY: Box<dyn ContextFactory + Sync> =
        Box::new(SampleContextFactory {});
}

/// Always hook into host's logging system.
#[no_mangle]
fn _start() {
    Logger::init().unwrap();
    unsafe {
        if RETRY {
            info!("hello VM!");
            register_factory(
                "my_root_id",
                &SAMPLE_CONTEXT_FACTORY,
                &SAMPLE_ROOT_CONTEXT_FACTORY,
            );
            info!("context factory registered!");
            RETRY = false;
        }
    }
}
