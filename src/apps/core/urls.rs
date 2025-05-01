use cobalto::router::Router;
use cobalto::route;
use crate::apps::core::views::{home, about, user_detail, dashboard};
use crate::middleware::auth_required;
use std::sync::Arc;

pub fn register_urls(router: &mut Router) {
    route![router,
        GET "/" => { home },
        GET "/about" => { about },
        GET "/user/:id" => { user_detail },
        GET "/dashboard" => { dashboard, auth_required() }
    ];
}