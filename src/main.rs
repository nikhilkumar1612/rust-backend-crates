mod handlers;

use cron::run_every;
use actix_web::{App, HttpServer};
use handlers::health::health;
use handlers::health::redirect;
// use aws_utils::AwsUtils;

async fn future(user: Option<(&str, )>) {
    if user.is_some() {
        println!("print from future {}", user.unwrap().0);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // fire and forget.
    let _result = tokio::task::spawn_blocking(
        || {
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            runtime.block_on(run_every(
                1000, // number of ms
                future, // fn which will run for every `interval`
                Some(("nikhil",)),
                None
            ))
        }
    );

    // let _aws_utils = AwsUtils::init("".to_string()).await;

    let server = HttpServer::new(move || {
        App::new()
            .service(health)
            .service(redirect)

    })
         .bind(("127.0.0.1", 8080));
    println!("Server listening on http://localhost:8080");
    server?.run().await
}
