use std::collections::HashMap;

use color_eyre::Result;
use tracing::{debug, info};

mod setup;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure errors and tracing
    setup::configure()?;

    debug!("Tracing and error handling using color-eyre configured.");

    let mut cache = HashMap::<i32, i32>::new();
    cache.insert(0, 0);
    cache.insert(1, 1);

    info!(
        ten = fibonacci(10, &mut cache),
        "Calculating 10th fibonacci number."
    );

    // Cleanup :)
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

#[tracing::instrument(skip(cache))]
fn fibonacci(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    let value = cache.get(&n);

    match value {
        Some(&number) => number,
        None => {
            let value = fibonacci(n - 2, cache) + fibonacci(n - 1, cache);
            debug!(value, n, "Value was not present in cache");
            cache.insert(n, value);
            value
        }
    }
}
