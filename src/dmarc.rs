use crate::data;
use color_eyre::{eyre, Result};
use tracing::{info, instrument};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    lookup::TxtLookup,
    TokioAsyncResolver,
};

pub async fn get_dmarc_policy_for_domain(domain: &str) -> Result<data::DmarcPolicy> {
    info!(%domain, "Retrieving DMARC policy");

    let txt_lookup = get_txt_lookup(domain).await?;
    let dmarc_record = extract_dmarc_record(&txt_lookup)?;
    let p_value = retrieve_p_value(&dmarc_record)?;

    let policy = match p_value.as_str() {
        "none" => data::DmarcPolicy::None,
        "quarantine" => data::DmarcPolicy::Quarantine,
        "reject" => data::DmarcPolicy::Reject,
        _ => return Err(eyre::eyre!("Unknown p= value: {}", p_value)),
    };

    Ok(policy)
}

#[instrument]
async fn get_txt_lookup(domain: &str) -> Result<TxtLookup> {
    info!(%domain, "Retrieving TXT record using trust-dns");

    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())?;
    let response = resolver.txt_lookup("_dmarc.".to_owned() + domain).await?;
    Ok(response)
}

#[instrument(skip(txt_lookup))]
fn extract_dmarc_record(txt_lookup: &TxtLookup) -> Result<String> {
    let dmarc_record = txt_lookup
        .iter()
        .find(|txt| txt.to_string().starts_with("v=DMARC1"))
        .ok_or_else(|| eyre::eyre!("No DMARC record found in TXT"))?;

    Ok(dmarc_record.to_string())
}

fn retrieve_p_value(dmarc_record: &str) -> Result<String> {
    let p_value = dmarc_record
        .split(';')
        .map(str::trim) // some records have spaces after the semicolon
        .find(|s| s.starts_with("p="))
        .ok_or_else(|| eyre::eyre!("No p= key-value pair found"))?
        .split('=')
        .nth(1)
        .ok_or_else(|| {
            eyre::eyre!("Found p property in TXT key-value pairs, but it has no value")
        })?;

    Ok(p_value.to_string())
}
