pub enum Region{
    UsEast,
    UsWest,
    EuWest,
    EuCentral,
    AsiaTokyo,
    AsiaSingapore
};

pub enum AuthMethod {
    QueryParam { key: String, value: String},
    Header { key: String, value: String},
    UrlPath { api_key: String},
    None,
}

pub struct Endpoint {
    pub url: String,
    pub region: Region,
    pub label: String,
}

pub trait Provider {
    fn name(&self) -> &str;
    fn endpoints(&self) -> &[Endpoint];
    fn auth(&self) -> AuthMethod;
    fn min_tip_lamports(&self) -> u64; //Note !!!!! : Lists Axiom and Padre fees management.
    fn supports_bundles(&self) -> bool;
    fn rpc_method(&self) -> &str;
}