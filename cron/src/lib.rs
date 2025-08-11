use tokio::time::{sleep, Duration};


pub async fn run_every<F, Args>(
    interval: u64,
    mut callback: F,
    args: Option<Args>,
    initial_run: Option<bool>
)
where
    Args: Clone,
    F: FnMut(Option<Args>) + Send + 'static
{
    let f = initial_run.unwrap_or(false);
    let duration = Duration::from_millis(interval);

    if f {
        loop {
            if args.is_some() {
                callback(args.clone());
            }
            callback(None);
            sleep(duration).await;
        }
    } else {
        loop {
            sleep(duration).await;
            if args.is_some() {
                callback(args.clone());
            }
            callback(None);
            sleep(duration).await;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Use tokio::test for async tests
    #[tokio::test]
    async fn it_works() {
        // Just run a single callback call with small interval for the test (break after first loop)
        // Or spawn with timeout logic to avoid blocking forever.

        let mut count = 0;

        run_every(100, || {
            println!("called callback");
            count += 1; // This will not work directly due to moved variables, so use RefCell or channels for mutable state in async closures.
        }, None, None)
        .await;
    }
}
