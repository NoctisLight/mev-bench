use super::endpoints::{
    JITO_ENDPOINTS, TEMPORAL_ENDPOINTS, ZEROSLOT_ENDPOINTS, ASTRALANE_ENDPOINTS, STELLIUM_ENDPOINTS, HELIUS_ENDPOINTS, BAM_ENDPOINTS, HARMONIC_ENDPOINTS
};
use super::{AuthMethod, ProviderConfig};

pub fn all_providers() -> Vec<ProviderConfig> {
    vec![
        ProviderConfig {
            name: "Jito",
            endpoints: JITO_ENDPOINTS,
            auth: AuthMethod::QueryParam { key: "uuid".to_string(), value: String::new() },
            min_tip_lamports: 1_000,
            supports_bundles: true,
        },
        ProviderConfig {
            name: "Temporal",
            endpoints: TEMPORAL_ENDPOINTS,
            auth: AuthMethod::QueryParam { key: "c".to_string(), value: String::new() },
            min_tip_lamports: 1_000_000,
            supports_bundles: false,
        },
        ProviderConfig {
            name: "0xSlot",
            endpoints: ZEROSLOT_ENDPOINTS,
            auth: AuthMethod::QueryParam { key: "api-key".to_string(), value: String::new() },
            min_tip_lamports: 1_000_000,
            supports_bundles: false,
        },
        ProviderConfig {
            name: "Astralane",
            endpoints: ASTRALANE_ENDPOINTS,
            auth: AuthMethod::Header { key: "api-key".to_string(), value: String::new() },
            min_tip_lamports: 100_000,
            supports_bundles: true,
        },
        ProviderConfig {
            name: "Stellium",
            endpoints: STELLIUM_ENDPOINTS,
            auth: AuthMethod::UrlPath { api_key: String::new() },
            min_tip_lamports: 1_000_000,
            supports_bundles: false,
        },
        ProviderConfig {
            name: "Helius",
            endpoints: HELIUS_ENDPOINTS,
            auth: AuthMethod::Header { key: "api-key".to_string(), value: String::new() },
            min_tip_lamports: 1_000_000,
            supports_bundles: false,
        },
        ProviderConfig {
            name: "BAM",
            endpoints: BAM_ENDPOINTS,
            auth: AuthMethod::None,
            min_tip_lamports: 1_000,
            supports_bundles: true,
        },
        ProviderConfig {
            name: "Harmonic",
            endpoints: HARMONIC_ENDPOINTS,
            auth: AuthMethod::None, // protobuf keypair — ping only for v0.1
            min_tip_lamports: 0, //check
            supports_bundles: true,
        },
    ]
}
