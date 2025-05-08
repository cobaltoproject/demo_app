use crate::apps::core::views::{about, dashboard, home, user_detail, users_view};
use crate::middleware::auth_required;
use cobalto::route;
use cobalto::router::Router;
use std::sync::Arc;

pub fn register_urls(router: &mut Router) {
    route![router,
        GET "/" => { home },
        GET "/about" => { about },
        GET "/user/:id" => { user_detail },
        GET "/dashboard" => { dashboard, auth_required() },
        GET "/users" => { users_view },
    ];
}
