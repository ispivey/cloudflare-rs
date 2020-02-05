use super::WorkersSecret;

use crate::framework::endpoint::{Endpoint, Method};

/// Create Secret
/// https://api.cloudflare.com/#worker-create-secret
pub struct CreateSecret {
    /// Account ID of script owner
    pub account_identifier: &'static str,
    /// The name of the script to attach the secret to
    pub script_name: &'static str,
    /// The contents of the secret
    pub params: CreateSecretParams,
}

impl Endpoint<WorkersSecret, (), CreateSecretParams> for CreateSecret {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
            self.account_identifier, self.script_name
        )
    }
    fn body(&self) -> Option<CreateSecretParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateSecretParams {
    /// the variable name of the secret that will be bound to the script
    pub name: String,
    /// the string value of the secret
    pub text: &'static [u8],
    // type of binding (e.g.secret_text)
    #[serde(rename = "type")]
    pub secret_type: String,
}
