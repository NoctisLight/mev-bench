pub mod endpoints;
pub mod providers;

pub enum Region{
    UsEast,
    UsWest,
    EuWest,
    EuCentral,
    AsiaTokyo,
    AsiaSingapore
}

pub enum AuthMethod {
    QueryParam { key: String, value: String},
    Header { key: String, value: String}, //Maybe not useful but lets keep it here
    UrlPath { api_key: String},
    None,
}

pub struct Endpoint {
    pub url: &'static str,
    pub region: Region,
    pub label: &'static str,
    pub rpc_method: &'static str
}

pub struct ProviderConfig{
    pub name: &'static str,
    pub endpoints: &'static [Endpoint],
    pub auth: AuthMethod,
    pub min_tip_lamports: u64, // Algorithm still to be made later 
    pub supports_bundles: bool
}