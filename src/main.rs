use cron::run_every;

fn future(user: Option<(&str, )>) {
    if user.is_some() {
        println!("print from future {}", user.unwrap().0);
    }
}

#[tokio::main]
async fn main() {
    // fire and forget.
    let _result = tokio::task::spawn_blocking(
        || {
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            runtime.block_on(run_every(
                1000,
                future,
                Some(("nikhil",)),
                None
            ))
        }
    );

    // continue with rest of the app/server
    println!("continue");
    return;
}
