use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::structs::{
    common::{Network, Version},
    wallet_metadata::{Deeplink, Images, Platform, WalletMetadata},
    wallet_type::WalletType,
};

pub static WALLETS_METADATA: Lazy<Vec<WalletMetadata>> = Lazy::new(|| {
    // TODO refactor it to separate file for each wallet
    return vec![
        // Nightly
        WalletMetadata {
            slug: "nightly".to_string(),
            name: "Nightly".to_string(),
            description: "TODO".to_string(),
            homepage: "https://wallet.nightly.app".to_string(),
            app: HashMap::from([
                (
                    Platform::chrome,
                    "https://chrome.google.com/webstore/detail/nightly/fiikommddbeccaoicoejoniammnalkfa".to_string(),
                ),
                (
                    Platform::edge,
                    "https://chrome.google.com/webstore/detail/nightly/fiikommddbeccaoicoejoniammnalkfa".to_string(),
                ),
                (
                    Platform::browser,
                    "https://chrome.google.com/webstore/detail/nightly/fiikommddbeccaoicoejoniammnalkfa".to_string(),
                ),
                (
                    Platform::brave,
                    "https://chrome.google.com/webstore/detail/nightly/fiikommddbeccaoicoejoniammnalkfa".to_string(),
                ),
                (
                    Platform::opera,
                    "https://chrome.google.com/webstore/detail/nightly/fiikommddbeccaoicoejoniammnalkfa".to_string(),
                ),
                (
                    Platform::opera,
                    "https://chrome.google.com/webstore/detail/nightly/fiikommddbeccaoicoejoniammnalkfa".to_string(),
                ),
                (
                    Platform::firefox,
                    "https://addons.mozilla.org/en-GB/firefox/addon/nightly-app/".to_string(),
                ),
                (
                    Platform::android,
                    "https://play.google.com/store/apps/details?id=com.nightlymobile".to_string(),
                ),
                (
                    Platform::ios,
                    "https://apps.apple.com/pl/app/nightly-multichain-wallet/id6444768157".to_string(),
                ),
            ]),
            chains: vec![Network::new("solana"), Network::new("near"), Network::new("sui"), Network::new("aptos")],
            desktop: None,
            mobile: Some(Deeplink {
                native: Some("nightly".to_string()),
                universal: Some("https://wallet.nightly.app".to_string()),
            }),
            image: Images {
                default: format!("https://registry.connect.nightly.app/wallets/nightly/default.png"),
                sm: format!("https://registry.connect.nightly.app/wallets/nightly/sm.png"),
                md: format!("https://registry.connect.nightly.app/wallets/nightly/md.png"),
                lg: format!("https://registry.connect.nightly.app/wallets/nightly/lg.png"),
            },
            inject_path: HashMap::from([
                (Network::new("solana"),"window.nightly.solana".to_string()),
                (Network::new("sui"),"window.nightly.sui".to_string()),
                (Network::new("aptos"),"window.nightly.aptos".to_string()),
                (Network::new("near"),"window.nightly.near".to_string())
            ]),
            last_updated_timestamp: 1686303253,
            version: Version("0.0.1".to_string()),
            wallet_type: WalletType::hybrid,
        },
    ];
});
