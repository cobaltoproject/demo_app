use cobalto::router::{Middleware, PostMiddleware, RequestContext, Response};
use std::sync::Arc;
use std::time::Instant;

pub fn logging_middleware() -> Middleware {
    Arc::new(|ctx: &mut RequestContext| {
        println!("➡️  Incoming request to {}", ctx.path);
        None
    })
}

pub fn auth_required() -> Middleware {
    Arc::new(|ctx: &mut RequestContext| {
        if ctx.path == "/dashboard" && !ctx.is_authenticated {
            Some(Response::forbidden("403 Forbidden: Authentication Required"))
        } else {
            None
        }
    })
}


pub fn timing_middleware() -> Middleware {
    Arc::new(|ctx: &mut RequestContext| {
        ctx.start_time = Some(Instant::now());
        None
    })
}

pub fn timing_post_middleware() -> PostMiddleware {
    Arc::new(|ctx: &RequestContext, mut response: Response| {
        if let Some(start) = ctx.start_time {
            let elapsed_ms = start.elapsed().as_millis();
            println!("⏱️ Request to '{}' completed in {}ms", ctx.path, elapsed_ms);
            response.headers.insert("X-Response-Time".to_string(), format!("{}ms", elapsed_ms));
        }
        response
    })
}