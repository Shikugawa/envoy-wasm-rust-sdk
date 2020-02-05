mod context;

mod envoy_log;
mod host;

use crate::envoy_log::Logger;
use context::*;
use log::info;

// =============== RootContext ============================
struct SampleRootContext {}

impl RootContext for SampleRootContext {
    fn on_start(&self) -> u32 {
        info!("Hello Envoy!");
        0
    }
}

struct SampleRootContextFactory {}

impl RootContextFactory for SampleRootContextFactory {
    fn create(&self) -> Box<dyn RootContext + Sync> {
        Box::new(SampleRootContext {})
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
    fn create(&self) -> Box<dyn Context + Sync> {
        Box::new(SampleContext {})
    }
}
// ========================================================

/// Always hook into host's logging system.
#[no_mangle]
fn _start() {
    Logger::init().unwrap();
    info!("hello VM!");
    register_factory(
        "my_root_id",
        &SampleRootContextFactory {},
        &SampleContextFactory {},
    );
    info!("context factory registered!");
}
