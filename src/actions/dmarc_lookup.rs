use actix_web::{get, web, HttpResponse, Responder};
use color_eyre::{eyre, Result};
use memcache::Client;
use serde::Serialize;
use tracing::{info, instrument, warn};

use crate::{data, dmarc};

#[derive(Serialize)]
struct DmarcPolicyResponse {
    domain: String,
    policy: data::DmarcPolicy,
}

#[get("/dmarc/{domain}")]
#[instrument(skip(data), fields(domain = %path))]
pub async fn get_dmarc_status(
    data: web::Data<data::MemcacheClient>,
    path: web::Path<String>,
) -> impl Responder {
    let domain = path.into_inner();
    let cache_client = data.client.to_owned();

    info!(%domain, "Retrieving DMARC status");

    match get_dmarc_status_from_cache_or_set(&domain, &cache_client).await {
        Ok(policy) => HttpResponse::Ok().json(DmarcPolicyResponse { domain, policy }),
        Err(err) => {
            warn!(?err, "Something went wrong when retrieving DMARC status");
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_dmarc_status_from_cache_or_set(
    domain: &str,
    memcache: &Client,
) -> Result<data::DmarcPolicy> {
    let cache_result = memcache.get::<String>(&domain).unwrap_or_else(|err| {
        warn!(?err, "Something went wrong when querying memcache");
        None
    });

    match cache_result {
        Some(policy) => {
            info!(%domain, "Found cached DMARC policy");

            data::DmarcPolicy::try_from(policy.as_str())
                .map_err(|err| eyre::eyre!("Could not parse cached DMARC policy: {:?}", err))
        }
        None => {
            warn!(%domain,"No cached DMARC policy");

            let policy = dmarc::get_dmarc_policy_for_domain(&domain)
                .await
                .unwrap_or(data::DmarcPolicy::None);

            memcache.set::<String>(&domain, format!("{:?}", policy), 60 * 60 * 24)?;

            Ok(policy)
        }
    }
}
