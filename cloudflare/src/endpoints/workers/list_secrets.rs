use super::WorkersSecret;

use crate::framework::endpoint::{Endpoint, Method};

/// List Secrets
/// Lists all secrets mappings for a given script
/// https://api.cloudflare.com/#worker-secrets-list-secrets
pub struct ListSecrets {
    pub account_identifier: &'static str,
    pub script_name: &'static str,
}

impl Endpoint<Vec<WorkersSecret>> for ListSecrets {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
            self.account_identifier, self.script_name
        )
    }
}
