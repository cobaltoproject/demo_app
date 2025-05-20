use super::models::{NewPost, PatchPost, Post};
use cobalto::router::{Request, Response};
use serde_json::json;

pub async fn list_posts(_req: Request) -> Response {
    // Hard-coded posts list as mock data
    let posts = vec![
        Post {
            id: 1,
            title: "Hello".into(),
            content: "Demo post".into(),
        },
        Post {
            id: 2,
            title: "Second".into(),
            content: "Another post".into(),
        },
    ];
    Response::json(posts)
}

pub async fn create_post(req: Request) -> Response {
    let body: Result<NewPost, _> = req.json();
    match body {
        Ok(new_post) => {
            let post = Post {
                id: 3,
                title: new_post.title,
                content: new_post.content,
            };

            Response::json(post).with_status(201)
        }
        Err(e) => Response::json(json!({
            "error": e.to_string()
        }))
        .with_status(400),
    }
}

pub async fn get_post(req: Request) -> Response {
    let post_id = req.params.get("post_id").cloned().unwrap_or_default();
    let post = Post {
        id: post_id.parse::<i64>().unwrap(),
        title: format!("Mock post {}", post_id),
        content: "Demo content (GET)".into(),
    };
    Response::json(post)
}

pub async fn update_post(req: Request) -> Response {
    let post_id = req.params.get("post_id").cloned().unwrap_or_default();
    let body: Result<NewPost, _> = req.json();
    match body {
        Ok(new_post) => {
            let post = Post {
                id: post_id.parse::<i64>().unwrap(),
                title: new_post.title,
                content: new_post.content,
            };
            Response::json(post).with_status(201)
        }
        Err(e) => Response::json(json!({
            "error": e.to_string()
        }))
        .with_status(400),
    }
}

pub async fn patch_post(req: Request) -> Response {
    let post_id = req.params.get("post_id").cloned().unwrap_or_default();
    let body: Result<PatchPost, _> = req.json();
    match body {
        Ok(new_post) => {
            let post = Post {
                id: post_id.parse::<i64>().unwrap(),
                title: new_post
                    .title
                    .or(Some(format!("Mock post {}", post_id)))
                    .unwrap(),
                content: new_post
                    .content
                    .or(Some("Demo content (PATCH)".into()))
                    .unwrap(),
            };
            Response::json(post).with_status(201)
        }
        Err(e) => Response::json(json!({
            "error": e.to_string()
        }))
        .with_status(400),
    }
}
