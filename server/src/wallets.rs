use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::structs::{
    common::{Network, Version},
    wallet_metadata::{Images, Platform, WalletMetadata},
    wallet_type::WalletType,
};

pub static WALLETS_METADATA: Lazy<Vec<WalletMetadata>> = Lazy::new(|| {
    let host = match std::env::var("ENV").unwrap_or_default() == "DEV" {
        true => "http://localhost:6969",
        false => "https://nc2.nightly.app",
    };
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
            ]),
            chains: vec![Network::new("solana"), Network::new("near"), Network::new("sui"), Network::new("aptos")],
            desktop: None,
            mobile: None, // TODO
            image: Images {
                default: format!("{host}/images/nightly/default.svg"),
                sm: format!("{host}/images/nightly/sm.svg"),
                md: format!("{host}/images/nightly/md.svg"),
                lg: format!("{host}/images/nightly/lg.svg"),
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
