pub mod context;

mod envoy_log;
mod host;

use context::*;
use lazy_static::lazy_static;
use log::debug;

// =============== RootContext ============================
struct SampleRootContext {}

impl RootContext for SampleRootContext {
    fn on_start(&self) {
        debug!("Hello Envoy!");
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

impl Context for SampleContext {}

impl SampleContext {
    fn on_create(&self) {
        debug!("Hello Envoy!")
    }
}

struct SampleContextFactory {}

impl ContextFactory for SampleContextFactory {
    fn create(&self) -> Box<dyn Context + Sync> {
        Box::new(SampleContext {})
    }
}
// ========================================================

lazy_static! {
    static ref REGISTERED: Registered = register_factory(
        "sample",
        &SampleRootContextFactory {},
        &SampleContextFactory {}
    );
}
