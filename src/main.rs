use cobalto::{
    router::{AppState, Router},
    settings::Settings,
};

mod apps;
mod middleware;
mod settings;
mod urls;
use crate::settings::DefaultSettings;
use env_logger;
use log::debug;

use crate::middleware::{
    auth_required, logging_middleware, timing_middleware, timing_post_middleware,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Logger initialized successfully");

    let settings: Settings = DefaultSettings::new();

    // --- Ensure persistent DB path exists ---
    use std::fs;
    let db_dir = "db";
    fs::create_dir_all(db_dir).expect("Failed to create db directory");
    let db_path = format!("{}/sqlite.db", db_dir);
    // ---------------------------------------

    // --- ORM DB connection and auto-migration ---
    use cobalto::orm::{Db, auto_migrate};
    use std::sync::Arc;
    let db = Arc::new(
        Db::connect(&db_path)
            .await
            .expect("Failed to connect to DB"),
    );
    cobalto::orm::apply_migration_files(db.clone(), "migrations")
        .await
        .expect("Failed to apply migrations");
    auto_migrate(db.clone())
        .await
        .expect("DB auto-migration failed");
    // --- End ORM setup ---

    let mut router = Router::new();
    let app_state = AppState {
        db: db.clone(),
        settings: settings.clone(),
    };
    router.set_app_state(app_state.clone());

    router.add_middleware(logging_middleware());
    router.add_middleware(auth_required());
    router.add_middleware(timing_middleware());
    router.add_post_middleware(timing_post_middleware());

    crate::urls::register_routes(&mut router);

    if let Err(e) = router.run(settings).await {
        eprintln!("Server error: {e}");
    }
}
