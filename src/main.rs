use cobalto::router::Router;

mod apps;
mod middleware;
mod settings;
mod urls;
use crate::settings::DefaultSettings;
use env_logger;
use log::debug;

use crate::middleware::{
    auth_required, logging_middleware, timing_middleware, timing_post_middleware,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Logger initialized successfully");

    let settings = DefaultSettings::new();

    let mut router = Router::new();

    router.add_middleware(logging_middleware());
    router.add_middleware(auth_required());
    router.add_middleware(timing_middleware());
    router.add_post_middleware(timing_post_middleware());

    crate::urls::register_routes(&mut router);

    if let Err(e) = router.run(settings).await {
        eprintln!("Server error: {e}");
    }
}
