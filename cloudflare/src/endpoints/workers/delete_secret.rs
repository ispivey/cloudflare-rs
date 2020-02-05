use super::WorkersSecret;

use crate::framework::endpoint::{Endpoint, Method};

/// Delete Secret
/// https://api.cloudflare.com/#worker-delete-secret
pub struct DeleteSecret {
    /// Account ID of script owner
    pub account_identifier: &'static str,
    /// The name of the script to remove the secret from
    pub script_name: &'static str,
    /// The variable name of the secret
    pub secret_name: &'static str,
}

impl Endpoint<WorkersSecret> for DeleteSecret {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets/{}",
            self.account_identifier, self.script_name, self.secret_name
        )
    }
}
