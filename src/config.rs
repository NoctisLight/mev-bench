struct Config {
    keys: Keys,
    rpc: Rpc,
}

// Everything is optinal until i figure how i will make the benchmark [Ping, sending etc]
struct Keys{
    jito_uuid: Option<String>,
    temporal_api_key: Option<String>,
    zeroslot_api_key: Option<String>,
    astralane_api_key: Option<String>,
    stellium_api_key: Option<String>,
    helius_api_key: Option:<String>
}