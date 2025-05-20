mod apps;
mod settings;
mod urls;

use cobalto::router::Router;
use settings::DefaultSettings;

#[tokio::main]
async fn main() {
    let mut router = Router::new(DefaultSettings::new());
    urls::register_routes(&mut router);
    for (method, path) in router.list_routes() {
        println!("{} {}", method, path);
    }

    router.run().await.unwrap();
}
