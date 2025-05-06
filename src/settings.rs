use cobalto::settings::{Settings, TemplateSettings};
use std::collections::HashMap;

pub trait DefaultSettings {
    fn new() -> Self;
}

impl DefaultSettings for Settings {
    fn new() -> Self {
        Self {
            debug: true,
            host: "127.0.0.1".to_string(),
            port: 8080,
            ws_port: 9000,
            template: TemplateSettings {
                dir: "templates".to_string(),
                debug: true,
            },
            other: HashMap::new(),
        }
    }
}
