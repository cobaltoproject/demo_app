use crate::apps::core::views::{create_post, get_post, list_posts, patch_post, update_post};
use cobalto::route;
use cobalto::router::Router;
use std::sync::Arc;

pub fn register_urls(router: &mut Router) {
    route![router,
        GET "/posts" => list_posts,
        POST "/posts" => create_post,
        GET "/posts/:post_id" => get_post,
        PUT "/posts/:post_id" => update_post,
        PATCH "/posts/:post_id" => patch_post,
    ];
}
