pub struct MonitorClient {
    reqwest: reqwest::Client,
    address: String,
    token: String,
}

impl MonitorClient {
    pub fn new_with_token(address: impl Into<String>, token: impl Into<String>) -> MonitorClient {
        MonitorClient {
            reqwest: Default::default(),
            address: address.into(),
            token: token.into(),
        }
    }

    pub fn new_with_credentials() {}
}
