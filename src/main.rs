use color_eyre::Result;
use tracing::info;

mod dmarc;
mod setup;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure errors and tracing
    setup::configure()?;

    let client =
        memcache::connect("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true").unwrap();

    let domains = ["adressa.no", "nsm.no", "posten.no", "xxl.no"];

    for domain in domains {
        match dmarc::get_dmarc_policy_for_domain(domain).await {
            Ok(policy) => {
                client
                    .set::<String>(domain, format!("{:?}", policy), 60 * 60 * 24)
                    .unwrap();
                info!("{domain}: {policy:?}");
            }
            Err(_) => info!("Missing DMARC for {domain}"),
        }
    }

    // Cleanup :)
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
