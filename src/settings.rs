use std::collections::HashMap;
use cobalto::settings::{Settings, TemplateSettings};

pub trait DefaultSettings {
    fn new() -> Self;
}

impl DefaultSettings for Settings {
    fn new() -> Self {
        Self {
            debug: true,
            host: "127.0.0.1".to_string(),
            port: 8080,
            template: TemplateSettings {
                dir: "templates".to_string(),
                debug: true,
            },
            other: HashMap::new(),
        }
    }
}