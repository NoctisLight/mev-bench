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