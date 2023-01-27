use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use color_eyre::Result;
use tracing::info;

mod actions;
mod data;
mod dmarc;
mod setup;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure errors and tracing
    setup::configure()?;

    let client =
        memcache::connect("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true").unwrap();

    info!("Connected to memcached server");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(data::MemcacheClient {
                client: client.to_owned(),
            }))
            .service(actions::dmarc_lookup::get_dmarc_status)
    })
    .bind(("127.1", 8080))?
    .run()
    .await?;

    // Cleanup :)
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
