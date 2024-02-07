use crate::structs::{
    common::{Network, Version},
    wallet_metadata::{Images, Platform, WalletMetadata},
    wallet_type::WalletType,
};
use std::collections::HashMap;

pub fn aleph_zero_signer_metadata() -> WalletMetadata {
    WalletMetadata {
        slug: "aleph-zero-signer".to_string(),
        name: "Aleph Zero Signer".to_string(),
        description: "Aleph Zero Signer".to_string(),
        homepage: "https://alephzero.org/signer".to_string(),
        app: HashMap::from([
            (
                Platform::chrome,
                "https://chrome.google.com/webstore/detail/aleph-zero-signer/opbinaebpmphpefcimknblieddamhmol".to_string(),
            ),
            (
                Platform::edge,
                "https://chrome.google.com/webstore/detail/aleph-zero-signer/opbinaebpmphpefcimknblieddamhmol".to_string(),
            ),
            (
                Platform::browser,
                "https://chrome.google.com/webstore/detail/aleph-zero-signer/opbinaebpmphpefcimknblieddamhmol".to_string(),
            ),
            (
                Platform::brave,
                "https://chrome.google.com/webstore/detail/aleph-zero-signer/opbinaebpmphpefcimknblieddamhmol".to_string(),
            ),
            (
                Platform::opera,
                "https://chrome.google.com/webstore/detail/aleph-zero-signer/opbinaebpmphpefcimknblieddamhmol".to_string(),
            ),
            (
                Platform::opera,
                "https://chrome.google.com/webstore/detail/aleph-zero-signer/opbinaebpmphpefcimknblieddamhmol".to_string(),
            ),
            (
                Platform::firefox,
                "https://addons.mozilla.org/en-GB/firefox/addon/aleph-zero-signer/".to_string(),
            ),
        ]),
        chains: vec![Network::new("polkadot")],
        desktop: None,
        mobile: None,
        image: Images {
            default: format!("https://registry.nightly.app/wallets/aleph-zero-signer/default.png"),
            sm: format!("https://registry.nightly.app/wallets/aleph-zero-signer/default.png"),
            md: format!("https://registry.nightly.app/wallets/aleph-zero-signer/default.png"),
            lg: format!("https://registry.nightly.app/wallets/aleph-zero-signer/default.png"),
        },
        inject_path: HashMap::from([
            (Network::new("polkadot"),"window.injectedWeb3.aleph-zero-signer".to_string()),
        ]),
        last_updated_timestamp: 1696942859,
        version: Version("0.1.0".to_string()),
        wallet_type: WalletType::extension,
    }
}
