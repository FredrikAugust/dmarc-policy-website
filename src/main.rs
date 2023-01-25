use color_eyre::Result;
use tracing::info;

mod dmarc;
mod setup;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure errors and tracing
    setup::configure()?;

    let domains = ["adressa.no", "nsm.no", "posten.no", "xxl.no"];

    for domain in domains {
        match dmarc::get_dmarc_policy_for_domain(domain).await {
            Ok(policy) => info!("{domain}: {policy:?}"),
            Err(_) => info!("Missing DMARC for {domain}"),
        }
    }

    // Cleanup :)
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
