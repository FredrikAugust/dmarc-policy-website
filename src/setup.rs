use color_eyre::Result;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};

pub fn configure() -> Result<()> {
    // Errors
    color_eyre::install()?;

    // Tracing
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("fibonacci-service")
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .init();

    Ok(())
}
