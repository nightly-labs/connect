use crate::structs::{
    common::{Network, Version},
    wallet_metadata::{Images, Platform, WalletMetadata},
    wallet_type::WalletType,
};
use std::collections::HashMap;

pub fn talisman_metadata() -> WalletMetadata {
    WalletMetadata {
    slug: "talisman".to_string(),
    name: "Talisman".to_string(),
    description: "Talisman".to_string(),
    homepage: "https://www.talisman.xyz/".to_string(),
    app: HashMap::from([
        (
            Platform::chrome,
            "https://chrome.google.com/webstore/detail/talisman-polkadot-and-eth/fijngjgcjhjmmpcmkeiomlglpeiijkld".to_string(),
        ),
        (
            Platform::edge,
            "https://chrome.google.com/webstore/detail/talisman-polkadot-and-eth/fijngjgcjhjmmpcmkeiomlglpeiijkld".to_string(),
        ),
        (
            Platform::browser,
            "https://chrome.google.com/webstore/detail/talisman-polkadot-and-eth/fijngjgcjhjmmpcmkeiomlglpeiijkld".to_string(),
        ),
        (
            Platform::brave,
            "https://chrome.google.com/webstore/detail/talisman-polkadot-and-eth/fijngjgcjhjmmpcmkeiomlglpeiijkld".to_string(),
        ),
        (
            Platform::opera,
            "https://chrome.google.com/webstore/detail/talisman-polkadot-and-eth/fijngjgcjhjmmpcmkeiomlglpeiijkld".to_string(),
        ),
        (
            Platform::opera,
            "https://chrome.google.com/webstore/detail/talisman-polkadot-and-eth/fijngjgcjhjmmpcmkeiomlglpeiijkld".to_string(),
        ),
        (
            Platform::firefox,
            "https://addons.mozilla.org/en-US/firefox/addon/talisman-wallet-extension".to_string(),
        ),
    ]),
    chains: vec![Network::new("polkadot")],
    desktop: None,
    mobile: None,
    image: Images {
        default: format!("https://registry.nightly.app/wallets/talisman/default.png"),
        sm: format!("https://registry.nightly.app/wallets/talisman/default.png"),
        md: format!("https://registry.nightly.app/wallets/talisman/default.png"),
        lg: format!("https://registry.nightly.app/wallets/talisman/default.png"),
    },
    inject_path: HashMap::from([
        (Network::new("polkadot"),"window.injectedWeb3.talisman".to_string()),
    ]),
    last_updated_timestamp: 1696942859,
    version: Version("0.1.0".to_string()),
    wallet_type: WalletType::extension,
}
}
