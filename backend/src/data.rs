use color_eyre::eyre;
use serde::Serialize;

pub struct MemcacheClient {
    pub client: memcache::Client,
}

#[derive(Debug, Serialize)]
pub enum DmarcPolicy {
    None,
    Quarantine,
    Reject,
}

impl TryFrom<&str> for DmarcPolicy {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "none" => Ok(DmarcPolicy::None),
            "quarantine" => Ok(DmarcPolicy::Quarantine),
            "reject" => Ok(DmarcPolicy::Reject),
            _ => Err(eyre::eyre!("Could not parse DMARC policy: {}", s)),
        }
    }

    type Error = color_eyre::eyre::Error;
}
