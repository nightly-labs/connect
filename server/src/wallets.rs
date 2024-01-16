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
            chains: vec![Network::new("solana"), Network::new("near"), Network::new("sui"), Network::new("aptos"), Network::new("polkadot")],
            desktop: None,
            mobile: Some(Deeplink {
                native: Some("nightly".to_string()),
                universal: Some("https://wallet.nightly.app".to_string()),
                redirect_to_app_browser: None
            }),
            image: Images {
                default: format!("https://registry.nightly.app/wallets/nightly/default.png"),
                sm: format!("https://registry.nightly.app/wallets/nightly/sm.png"),
                md: format!("https://registry.nightly.app/wallets/nightly/md.png"),
                lg: format!("https://registry.nightly.app/wallets/nightly/lg.png"),
            },
            inject_path: HashMap::from([
                (Network::new("solana"),"window.nightly.solana".to_string()),
                (Network::new("sui"),"window.nightly.sui".to_string()),
                (Network::new("aptos"),"window.nightly.aptos".to_string()),
                (Network::new("near"),"window.nightly.near".to_string()),
                (Network::new("polkadot"),"window.nightly.polkadot".to_string()),
            ]),
            last_updated_timestamp: 1686303253,
            version: Version("0.0.1".to_string()),
            wallet_type: WalletType::hybrid,
        },
        // Aleph Zero Signer
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
        },
         // SubWallet
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
        },
          // Talisman
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
        },
          // Polkadot{.js}
          WalletMetadata {
            slug: "polkadot-js".to_string(),
            name: "Polkadot{.js}".to_string(),
            description: "Polkadot{.js} extension".to_string(),
            homepage: "https://polkadot.js.org/extension/".to_string(),
            app: HashMap::from([
                (
                    Platform::chrome,
                    "https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd".to_string(),
                ),
                (
                    Platform::edge,
                    "https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd".to_string(),
                ),
                (
                    Platform::browser,
                    "https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd".to_string(),
                ),
                (
                    Platform::brave,
                    "https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd".to_string(),
                ),
                (
                    Platform::opera,
                    "https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd".to_string(),
                ),
                (
                    Platform::opera,
                    "https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd".to_string(),
                ),
                (
                    Platform::firefox,
                    "https://addons.mozilla.org/en-US/firefox/addon/polkadot-js-extension/".to_string(),
                ),
            ]),
            chains: vec![Network::new("polkadot")],
            desktop: None,
            mobile: None,
            image: Images {
                default: format!("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiIHN0YW5kYWxvbmU9InllcyI/PjxzdmcgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmVyc2lvbj0iMS4xIiBpZD0iTGF5ZXJfMSIgeD0iMHB4IiB5PSIwcHgiIHZpZXdCb3g9IjE1IDE1IDE0MCAxNDAiIHN0eWxlPSJlbmFibGUtYmFja2dyb3VuZDpuZXcgMCAwIDE3MCAxNzA7em9vbTogMTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPjxzdHlsZSB0eXBlPSJ0ZXh0L2NzcyI+LmJnMHtmaWxsOiNGRjhDMDB9IC5zdDB7ZmlsbDojRkZGRkZGfTwvc3R5bGU+PGc+PGNpcmNsZSBjbGFzcz0iYmcwIiBjeD0iODUiIGN5PSI4NSIgcj0iNzAiPjwvY2lyY2xlPjxnPjxwYXRoIGNsYXNzPSJzdDAiIGQ9Ik04NSwzNC43Yy0yMC44LDAtMzcuOCwxNi45LTM3LjgsMzcuOGMwLDQuMiwwLjcsOC4zLDIsMTIuM2MwLjksMi43LDMuOSw0LjIsNi43LDMuM2MyLjctMC45LDQuMi0zLjksMy4zLTYuNyBjLTEuMS0zLjEtMS42LTYuNC0xLjUtOS43QzU4LjEsNTcuNiw2OS41LDQ2LDgzLjYsNDUuM2MxNS43LTAuOCwyOC43LDExLjcsMjguNywyNy4yYzAsMTQuNS0xMS40LDI2LjQtMjUuNywyNy4yIGMwLDAtNS4zLDAuMy03LjksMC43Yy0xLjMsMC4yLTIuMywwLjQtMywwLjVjLTAuMywwLjEtMC42LTAuMi0wLjUtMC41bDAuOS00LjRMODEsNzMuNGMwLjYtMi44LTEuMi01LjYtNC02LjIgYy0yLjgtMC42LTUuNiwxLjItNi4yLDRjMCwwLTExLjgsNTUtMTEuOSw1NS42Yy0wLjYsMi44LDEuMiw1LjYsNCw2LjJjMi44LDAuNiw1LjYtMS4yLDYuMi00YzAuMS0wLjYsMS43LTcuOSwxLjctNy45IGMxLjItNS42LDUuOC05LjcsMTEuMi0xMC40YzEuMi0wLjIsNS45LTAuNSw1LjktMC41YzE5LjUtMS41LDM0LjktMTcuOCwzNC45LTM3LjdDMTIyLjgsNTEuNiwxMDUuOCwzNC43LDg1LDM0Ljd6IE04Ny43LDEyMS43IGMtMy40LTAuNy02LjgsMS40LTcuNSw0LjljLTAuNywzLjQsMS40LDYuOCw0LjksNy41YzMuNCwwLjcsNi44LTEuNCw3LjUtNC45QzkzLjMsMTI1LjcsOTEuMiwxMjIuNCw4Ny43LDEyMS43eiI+PC9wYXRoPjwvZz48L2c+PC9zdmc+Cg=="),
                sm: format!("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiIHN0YW5kYWxvbmU9InllcyI/PjxzdmcgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmVyc2lvbj0iMS4xIiBpZD0iTGF5ZXJfMSIgeD0iMHB4IiB5PSIwcHgiIHZpZXdCb3g9IjE1IDE1IDE0MCAxNDAiIHN0eWxlPSJlbmFibGUtYmFja2dyb3VuZDpuZXcgMCAwIDE3MCAxNzA7em9vbTogMTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPjxzdHlsZSB0eXBlPSJ0ZXh0L2NzcyI+LmJnMHtmaWxsOiNGRjhDMDB9IC5zdDB7ZmlsbDojRkZGRkZGfTwvc3R5bGU+PGc+PGNpcmNsZSBjbGFzcz0iYmcwIiBjeD0iODUiIGN5PSI4NSIgcj0iNzAiPjwvY2lyY2xlPjxnPjxwYXRoIGNsYXNzPSJzdDAiIGQ9Ik04NSwzNC43Yy0yMC44LDAtMzcuOCwxNi45LTM3LjgsMzcuOGMwLDQuMiwwLjcsOC4zLDIsMTIuM2MwLjksMi43LDMuOSw0LjIsNi43LDMuM2MyLjctMC45LDQuMi0zLjksMy4zLTYuNyBjLTEuMS0zLjEtMS42LTYuNC0xLjUtOS43QzU4LjEsNTcuNiw2OS41LDQ2LDgzLjYsNDUuM2MxNS43LTAuOCwyOC43LDExLjcsMjguNywyNy4yYzAsMTQuNS0xMS40LDI2LjQtMjUuNywyNy4yIGMwLDAtNS4zLDAuMy03LjksMC43Yy0xLjMsMC4yLTIuMywwLjQtMywwLjVjLTAuMywwLjEtMC42LTAuMi0wLjUtMC41bDAuOS00LjRMODEsNzMuNGMwLjYtMi44LTEuMi01LjYtNC02LjIgYy0yLjgtMC42LTUuNiwxLjItNi4yLDRjMCwwLTExLjgsNTUtMTEuOSw1NS42Yy0wLjYsMi44LDEuMiw1LjYsNCw2LjJjMi44LDAuNiw1LjYtMS4yLDYuMi00YzAuMS0wLjYsMS43LTcuOSwxLjctNy45IGMxLjItNS42LDUuOC05LjcsMTEuMi0xMC40YzEuMi0wLjIsNS45LTAuNSw1LjktMC41YzE5LjUtMS41LDM0LjktMTcuOCwzNC45LTM3LjdDMTIyLjgsNTEuNiwxMDUuOCwzNC43LDg1LDM0Ljd6IE04Ny43LDEyMS43IGMtMy40LTAuNy02LjgsMS40LTcuNSw0LjljLTAuNywzLjQsMS40LDYuOCw0LjksNy41YzMuNCwwLjcsNi44LTEuNCw3LjUtNC45QzkzLjMsMTI1LjcsOTEuMiwxMjIuNCw4Ny43LDEyMS43eiI+PC9wYXRoPjwvZz48L2c+PC9zdmc+Cg=="),
                md: format!("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiIHN0YW5kYWxvbmU9InllcyI/PjxzdmcgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmVyc2lvbj0iMS4xIiBpZD0iTGF5ZXJfMSIgeD0iMHB4IiB5PSIwcHgiIHZpZXdCb3g9IjE1IDE1IDE0MCAxNDAiIHN0eWxlPSJlbmFibGUtYmFja2dyb3VuZDpuZXcgMCAwIDE3MCAxNzA7em9vbTogMTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPjxzdHlsZSB0eXBlPSJ0ZXh0L2NzcyI+LmJnMHtmaWxsOiNGRjhDMDB9IC5zdDB7ZmlsbDojRkZGRkZGfTwvc3R5bGU+PGc+PGNpcmNsZSBjbGFzcz0iYmcwIiBjeD0iODUiIGN5PSI4NSIgcj0iNzAiPjwvY2lyY2xlPjxnPjxwYXRoIGNsYXNzPSJzdDAiIGQ9Ik04NSwzNC43Yy0yMC44LDAtMzcuOCwxNi45LTM3LjgsMzcuOGMwLDQuMiwwLjcsOC4zLDIsMTIuM2MwLjksMi43LDMuOSw0LjIsNi43LDMuM2MyLjctMC45LDQuMi0zLjksMy4zLTYuNyBjLTEuMS0zLjEtMS42LTYuNC0xLjUtOS43QzU4LjEsNTcuNiw2OS41LDQ2LDgzLjYsNDUuM2MxNS43LTAuOCwyOC43LDExLjcsMjguNywyNy4yYzAsMTQuNS0xMS40LDI2LjQtMjUuNywyNy4yIGMwLDAtNS4zLDAuMy03LjksMC43Yy0xLjMsMC4yLTIuMywwLjQtMywwLjVjLTAuMywwLjEtMC42LTAuMi0wLjUtMC41bDAuOS00LjRMODEsNzMuNGMwLjYtMi44LTEuMi01LjYtNC02LjIgYy0yLjgtMC42LTUuNiwxLjItNi4yLDRjMCwwLTExLjgsNTUtMTEuOSw1NS42Yy0wLjYsMi44LDEuMiw1LjYsNCw2LjJjMi44LDAuNiw1LjYtMS4yLDYuMi00YzAuMS0wLjYsMS43LTcuOSwxLjctNy45IGMxLjItNS42LDUuOC05LjcsMTEuMi0xMC40YzEuMi0wLjIsNS45LTAuNSw1LjktMC41YzE5LjUtMS41LDM0LjktMTcuOCwzNC45LTM3LjdDMTIyLjgsNTEuNiwxMDUuOCwzNC43LDg1LDM0Ljd6IE04Ny43LDEyMS43IGMtMy40LTAuNy02LjgsMS40LTcuNSw0LjljLTAuNywzLjQsMS40LDYuOCw0LjksNy41YzMuNCwwLjcsNi44LTEuNCw3LjUtNC45QzkzLjMsMTI1LjcsOTEuMiwxMjIuNCw4Ny43LDEyMS43eiI+PC9wYXRoPjwvZz48L2c+PC9zdmc+Cg=="),
                lg: format!("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiIHN0YW5kYWxvbmU9InllcyI/PjxzdmcgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmVyc2lvbj0iMS4xIiBpZD0iTGF5ZXJfMSIgeD0iMHB4IiB5PSIwcHgiIHZpZXdCb3g9IjE1IDE1IDE0MCAxNDAiIHN0eWxlPSJlbmFibGUtYmFja2dyb3VuZDpuZXcgMCAwIDE3MCAxNzA7em9vbTogMTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPjxzdHlsZSB0eXBlPSJ0ZXh0L2NzcyI+LmJnMHtmaWxsOiNGRjhDMDB9IC5zdDB7ZmlsbDojRkZGRkZGfTwvc3R5bGU+PGc+PGNpcmNsZSBjbGFzcz0iYmcwIiBjeD0iODUiIGN5PSI4NSIgcj0iNzAiPjwvY2lyY2xlPjxnPjxwYXRoIGNsYXNzPSJzdDAiIGQ9Ik04NSwzNC43Yy0yMC44LDAtMzcuOCwxNi45LTM3LjgsMzcuOGMwLDQuMiwwLjcsOC4zLDIsMTIuM2MwLjksMi43LDMuOSw0LjIsNi43LDMuM2MyLjctMC45LDQuMi0zLjksMy4zLTYuNyBjLTEuMS0zLjEtMS42LTYuNC0xLjUtOS43QzU4LjEsNTcuNiw2OS41LDQ2LDgzLjYsNDUuM2MxNS43LTAuOCwyOC43LDExLjcsMjguNywyNy4yYzAsMTQuNS0xMS40LDI2LjQtMjUuNywyNy4yIGMwLDAtNS4zLDAuMy03LjksMC43Yy0xLjMsMC4yLTIuMywwLjQtMywwLjVjLTAuMywwLjEtMC42LTAuMi0wLjUtMC41bDAuOS00LjRMODEsNzMuNGMwLjYtMi44LTEuMi01LjYtNC02LjIgYy0yLjgtMC42LTUuNiwxLjItNi4yLDRjMCwwLTExLjgsNTUtMTEuOSw1NS42Yy0wLjYsMi44LDEuMiw1LjYsNCw2LjJjMi44LDAuNiw1LjYtMS4yLDYuMi00YzAuMS0wLjYsMS43LTcuOSwxLjctNy45IGMxLjItNS42LDUuOC05LjcsMTEuMi0xMC40YzEuMi0wLjIsNS45LTAuNSw1LjktMC41YzE5LjUtMS41LDM0LjktMTcuOCwzNC45LTM3LjdDMTIyLjgsNTEuNiwxMDUuOCwzNC43LDg1LDM0Ljd6IE04Ny43LDEyMS43IGMtMy40LTAuNy02LjgsMS40LTcuNSw0LjljLTAuNywzLjQsMS40LDYuOCw0LjksNy41YzMuNCwwLjcsNi44LTEuNCw3LjUtNC45QzkzLjMsMTI1LjcsOTEuMiwxMjIuNCw4Ny43LDEyMS43eiI+PC9wYXRoPjwvZz48L2c+PC9zdmc+Cg=="),
            },
            inject_path: HashMap::from([
                (Network::new("polkadot"),"window.injectedWeb3.polkadot-js".to_string()),
            ]),
            last_updated_timestamp: 1696942859,
            version: Version("0.1.0".to_string()),
            wallet_type: WalletType::extension,
        },
    ];
});
