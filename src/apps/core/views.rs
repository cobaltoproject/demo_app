use std::collections::HashMap;
use cobalto::template::{render_template, TemplateValue};
use cobalto::router::Response;

pub async fn home(_params: HashMap<String, String>) -> Response {
    let context = HashMap::from([
        ("username".to_string(), TemplateValue::String("Alessandro".into())),
        ("shopping_list".to_string(), TemplateValue::List(vec![
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
        ])),
    ]);

    render_template("home.html", &context)
}

pub async fn about(_params: HashMap<String, String>) -> Response {
    let context = HashMap::from([
        ("description".into(), TemplateValue::String("Cobalto è il framework del futuro!".into())),
    ]);

    render_template("about.html", &context)
}

pub async fn user_detail(params: HashMap<String, String>) -> Response {
    let user_id = params.get("id").unwrap_or(&"unknown".to_string()).to_string();

    let context = HashMap::from([
        ("user_id".into(), TemplateValue::String(user_id)),
    ]);

    render_template("user_detail.html", &context)
}

pub async fn dashboard(_params: HashMap<String, String>) -> Response {
    let context = HashMap::from([
        ("admin_username".into(), TemplateValue::String("Admin".into())),
    ]);

    render_template("dashboard.html", &context)
}