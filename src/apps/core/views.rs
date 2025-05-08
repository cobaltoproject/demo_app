use cobalto::router::AppState;
use cobalto::router::Response;
use cobalto::template::{TemplateValue, render_template};
use std::collections::HashMap;

pub async fn home(_params: HashMap<String, String>, _state: AppState) -> Response {
    // cobalto::template::set_display_logs(true);
    let context = HashMap::from([
        (
            "username".to_string(),
            TemplateValue::String("Alessandro".into()),
        ),
        (
            "shopping_list".to_string(),
            TemplateValue::List(vec![
                TemplateValue::Object(HashMap::from([
                    ("name".into(), TemplateValue::String("Apple".into())),
                    ("available".into(), TemplateValue::Bool(true)),
                ])),
                TemplateValue::Object(HashMap::from([
                    ("name".into(), TemplateValue::String("Banana".into())),
                    ("available".into(), TemplateValue::Bool(false)),
                ])),
                TemplateValue::Object(HashMap::from([
                    ("name".into(), TemplateValue::String("Orange".into())),
                    ("available".into(), TemplateValue::Bool(true)),
                ])),
            ]),
        ),
    ]);

    render_template("home.html", &context)
}

pub async fn about(_params: HashMap<String, String>, _state: AppState) -> Response {
    let context = HashMap::from([(
        "description".into(),
        TemplateValue::String("Cobalto è il framework del futuro!".into()),
    )]);

    render_template("about.html", &context)
}

pub async fn user_detail(params: HashMap<String, String>, _state: AppState) -> Response {
    let user_id = params
        .get("id")
        .unwrap_or(&"unknown".to_string())
        .to_string();

    let context = HashMap::from([("user_id".into(), TemplateValue::String(user_id))]);

    render_template("user_detail.html", &context)
}

pub async fn dashboard(_params: HashMap<String, String>, _state: AppState) -> Response {
    let context = HashMap::from([(
        "admin_username".into(),
        TemplateValue::String("Admin".into()),
    )]);

    render_template("dashboard.html", &context)
}

use crate::apps::core::models::User;

// Handler factory that returns an actual HTTP handler closure with captured DB
pub async fn users_view(_params: HashMap<String, String>, state: AppState) -> Response {
    let db = state.db.clone();
    // Insert example user
    let username = "Alice (inserted)";
    let insert_res = db
        .execute(&format!("INSERT INTO users (username) VALUES ('{}')", username))
        .await;

    // Fetch all users
    let users: Vec<User> = db
        .fetch_all("SELECT id, username FROM users")
        .await
        .unwrap_or_default();

    // Build a JSON payload
    let payload = serde_json::json!({
        "insert_result": format!("{:?}", insert_res),
        "users": users.iter().map(|u| serde_json::json!({
            "id": u.id,
            "username": u.username
        })).collect::<Vec<_>>()
    });

    // Return as JSON Response
    cobalto::router::Response::json(payload, 200, std::collections::HashMap::new())
}
