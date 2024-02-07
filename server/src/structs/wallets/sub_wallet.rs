use crate::structs::{
    common::{Network, Version},
    wallet_metadata::{Deeplink, Images, Platform, WalletMetadata},
    wallet_type::WalletType,
};
use std::collections::HashMap;

pub fn subwallet_metadata() -> WalletMetadata {
    WalletMetadata {
        slug: "subwallet-js".to_string(),
        name: "SubWallet".to_string(),
        description: "The comprehensive non-custodial wallet solution for Polkadot, Substrate & Ethereum ecosystem".to_string(),
        homepage: "https://www.subwallet.app/".to_string(),
        app: HashMap::from([
            (
                Platform::chrome,
                "https://chrome.google.com/webstore/detail/subwallet-polkadot-wallet/onhogfjeacnfoofkfgppdlbmlmnplgbn".to_string(),
            ),
            (
                Platform::edge,
                "https://chrome.google.com/webstore/detail/subwallet-polkadot-wallet/onhogfjeacnfoofkfgppdlbmlmnplgbn".to_string(),
            ),
            (
                Platform::browser,
                "https://chrome.google.com/webstore/detail/subwallet-polkadot-wallet/onhogfjeacnfoofkfgppdlbmlmnplgbn".to_string(),
            ),
            (
                Platform::brave,
                "https://chrome.google.com/webstore/detail/subwallet-polkadot-wallet/onhogfjeacnfoofkfgppdlbmlmnplgbn".to_string(),
            ),
            (
                Platform::opera,
                "https://chrome.google.com/webstore/detail/subwallet-polkadot-wallet/onhogfjeacnfoofkfgppdlbmlmnplgbn".to_string(),
            ),
            (
                Platform::opera,
                "https://chrome.google.com/webstore/detail/subwallet-polkadot-wallet/onhogfjeacnfoofkfgppdlbmlmnplgbn".to_string(),
            ),
            (
                Platform::firefox,
                "https://addons.mozilla.org/en-US/firefox/addon/subwallet/".to_string(),
            ),
        ]),
        chains: vec![Network::new("polkadot")],
        desktop: None,
        mobile: Some(Deeplink {
            native: None,
            universal: None,
            redirect_to_app_browser: Some("https://mobile.subwallet.app/browser?url={{url}}".to_string()),
        }),
        image: Images {
            default: format!("https://registry.nightly.app/wallets/subwallet-js/default.png"),
            sm: format!("https://registry.nightly.app/wallets/subwallet-js/default.png"),
            md: format!("https://registry.nightly.app/wallets/subwallet-js/default.png"),
            lg: format!("https://registry.nightly.app/wallets/subwallet-js/default.png"),
        },
        inject_path: HashMap::from([
            (Network::new("polkadot"),"window.injectedWeb3.subwallet-js".to_string()),
        ]),
        last_updated_timestamp: 1705113229,
        version: Version("0.1.0".to_string()),
        wallet_type: WalletType::hybrid,
    }
}
