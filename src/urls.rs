use cobalto::router::Router;
use crate::apps::core::urls::register_urls as core_register_urls;

pub fn register_routes(router: &mut Router) {
    core_register_urls(router);
}